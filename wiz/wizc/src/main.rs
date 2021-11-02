use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::Ast2HLIR;
use crate::llvm_ir::codegen::{CodeGen, MLContext};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::HLIR2MLIR;
use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use std::error::Error;
use std::option::Option::Some;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, result};
use wiz_syntax::parser;
use wiz_syntax::parser::wiz::{
    parse_from_file_path, parse_from_file_path_str, read_package_from_path,
};
use wiz_syntax::syntax::file::WizFile;

mod constants;
mod ext;
mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod utils;

type MainFunc = unsafe extern "C" fn() -> u8;

fn get_builtin_find_path() -> PathBuf {
    let mut std_path = PathBuf::from(env!("HOME"));
    std_path.push(".wiz");
    std_path.push("lib");
    std_path.push("src");
    std_path
}

fn get_find_paths() -> Vec<PathBuf> {
    vec![get_builtin_find_path()]
}

fn get_builtin_lib() -> Vec<&'static str> {
    vec!["core", "std"]
}

fn main() -> result::Result<(), Box<dyn Error>> {
    let app = App::new("wizc")
        .arg(Arg::with_name("input").required(true).multiple(true))
        .arg(Arg::with_name("name").long("name").takes_value(true))
        .arg(
            Arg::with_name("type")
                .long("type")
                .takes_value(true)
                .value_names(&["bin", "test", "lib"]),
        )
        .arg(Arg::with_name("output").short("o").takes_value(true))
        .arg(Arg::with_name("out-dir").long("out-dir").takes_value(true))
        .arg(Arg::with_name("execute").short("e").takes_value(true))
        .arg(
            Arg::with_name("path")
                .short("p")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("L")
                .short("L")
                .takes_value(true)
                .multiple(true),
        );
    let matches = app.get_matches();
    let inputs = matches.values_of_lossy("input").unwrap();
    let output = matches.value_of("output");
    let out_dir = matches.value_of("out-dir");
    let paths = matches.values_of_lossy("path").unwrap_or_default();
    let type_ = matches.value_of("type");

    if let Some(type_) = type_ {
        let name = matches.value_of("name").unwrap();
    };

    println!("=== parse files ===");

    let mut lib_paths = vec![];

    for l in get_builtin_lib() {
        for mut p in get_find_paths()
            .into_iter()
            .chain(paths.iter().map(PathBuf::from))
        {
            p.push(l);
            p.push("Package.wiz");
            if p.exists() {
                p.pop();
                println!("`{}` found at {}", l, p.display());
                lib_paths.push(p);
                break;
            } else {
                p.pop();
                println!("`{}` Not found at {}", l, p.display());
            }
        }
    }

    let source_sets = lib_paths
        .into_iter()
        .map(|p| read_package_from_path(p.as_path()))
        .collect::<parser::result::Result<Vec<_>>>()?;

    println!("=== convert to hlir ===");

    let mut ast2hlir = Ast2HLIR::new();

    let std_hlir: Vec<_> = source_sets
        .into_iter()
        .map(|s| ast2hlir.source_set(s))
        .collect();

    let ast_files = inputs
        .iter()
        .map(|s| parse_from_file_path_str(s))
        .collect::<parser::result::Result<Vec<_>>>()?;

    let hlfiles: Vec<TypedFile> = ast_files.into_iter().map(|f| ast2hlir.file(f)).collect();

    println!("=== resolve type ===");

    let mut type_resolver = TypeResolver::new();
    type_resolver.global_use(vec!["core", "builtin", "*"]);

    println!("===== detect types =====");
    // detect types
    for s in std_hlir.iter() {
        type_resolver.detect_type_from_source_set(s)?;
    }

    for hlir in hlfiles.iter() {
        type_resolver.detect_type(hlir)?;
    }

    println!("===== preload decls =====");
    // preload decls
    for hlir in std_hlir.iter() {
        type_resolver.preload_source_set(hlir.clone())?;
    }

    for hlir in hlfiles.iter() {
        type_resolver.preload_file(hlir.clone())?;
    }

    println!("===== resolve types =====");
    // resolve types

    let std_hlir: Vec<_> = std_hlir
        .into_iter()
        .map(|s| type_resolver.source_set(s))
        .collect::<Result<Vec<_>>>()?;

    let hlfiles = hlfiles
        .into_iter()
        .map(|f| type_resolver.file(f))
        .collect::<Result<Vec<TypedFile>>>()?;

    println!("===== convert to mlir =====");

    let mut hlir2mlir = HLIR2MLIR::new();

    let std_mlir = std_hlir
        .into_iter()
        .map(|w| hlir2mlir.source_set(w))
        .flatten()
        .collect::<Vec<_>>();

    for m in std_mlir.iter() {
        println!("==== {} ====", m.name);
        println!("{}", m.to_string());
    }

    let mlfiles: Vec<MLFile> = hlfiles.into_iter().map(|f| hlir2mlir.file(f)).collect();

    for m in mlfiles.iter() {
        println!("==== {} ====", m.name);
        println!("{}", m.to_string());
    }

    for mlfile in mlfiles {
        let module_name = &mlfile.name;
        let context = Context::create();
        let module = context.create_module(module_name);
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
        let mut codegen = CodeGen {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
            ml_context: MLContext::new(),
        };

        for m in std_mlir.iter() {
            codegen.file(m.clone());
        }

        codegen.file(mlfile.clone());

        let output = if let Some(output) = output {
            if inputs.len() == 1 {
                String::from(output)
            } else {
                eprintln!("multiple file detected, can not use -o option");
                exit(-1)
            }
        } else {
            let mut output_path = Path::new(&mlfile.name).to_path_buf();
            output_path.set_extension("ll");
            String::from(output_path.to_str().unwrap())
        };

        let mut out_path = out_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap());
        out_path.push(output);

        println!("Output Path -> {:?}", out_path);

        codegen.print_to_file(out_path)?;

        if let Some(fun_name) = matches.value_of("execute") {
            unsafe {
                let main: JitFunction<MainFunc> =
                    codegen.execution_engine.get_function(fun_name)?;
                let _ = main.call();
            }
        }
    }

    Ok(())
}
