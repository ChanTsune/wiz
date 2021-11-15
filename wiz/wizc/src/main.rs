use crate::config::Config;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::Ast2HLIR;
use crate::llvm_ir::codegen::{CodeGen, MLContext};
use crate::middle_level_ir::HLIR2MLIR;
use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use std::io::Write;
use std::error::Error;
use std::option::Option::Some;
use std::path::{Path, PathBuf};
use std::{env, fs, result};
use wiz_syntax::parser;
use wiz_syntax::parser::wiz::{parse_from_file_path, read_package_from_path};
use wiz_syntax::syntax::file::SourceSet;

mod config;
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
    println!("Args {:?}", env::args());
    let app = App::new("wizc")
        .arg(Arg::with_name("input").required(true))
        .arg(Arg::with_name("name").long("name").takes_value(true))
        .arg(
            Arg::with_name("type")
                .long("type")
                .takes_value(true)
                .possible_values(&["bin", "test", "lib"]),
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
    let config = Config::from(&matches);
    let output = config.output();
    let out_dir = config.out_dir();
    let paths = config.paths();
    let input = config.input();
    let out_dir = out_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let mut mlir_out_dir = {
        let mut t = out_dir.clone();
        t.push("mlir");
        t
    };

    let input_source = if input.is_dir() {
        read_package_from_path(input)?
    } else {
        SourceSet::File(parse_from_file_path(input)?)
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

    let hlfiles = ast2hlir.source_set(input_source);

    println!("=== resolve type ===");

    let mut type_resolver = TypeResolver::new();
    type_resolver.global_use(vec!["core", "builtin", "*"]);
    type_resolver.global_use(vec!["std", "builtin", "*"]);

    println!("===== detect types =====");
    // detect types
    for s in std_hlir.iter() {
        type_resolver.detect_type_from_source_set(s)?;
    }

    type_resolver.detect_type_from_source_set(&hlfiles)?;

    println!("===== preload decls =====");
    // preload decls
    for hlir in std_hlir.iter() {
        type_resolver.preload_source_set(hlir.clone())?;
    }

    type_resolver.preload_source_set(hlfiles.clone())?;

    println!("===== resolve types =====");
    // resolve types

    let std_hlir: Vec<_> = std_hlir
        .into_iter()
        .map(|s| type_resolver.source_set(s))
        .collect::<Result<Vec<_>>>()?;

    let hlfiles = type_resolver.source_set(hlfiles)?;

    println!("===== convert to mlir =====");

    let mut hlir2mlir = HLIR2MLIR::new();

    let std_mlir = std_hlir
        .into_iter()
        .map(|w| hlir2mlir.source_set(w))
        .collect::<Vec<_>>();

    fs::create_dir_all(&mlir_out_dir)?;
    for m in std_mlir.iter() {
        println!("==== {} ====", m.name);
        mlir_out_dir.push(m.name.clone());
        let mut f = fs::File::create(&mlir_out_dir)?;
        write!(f, "{}", m.to_string())?;
        mlir_out_dir.pop();
    }

    let mlfile = hlir2mlir.source_set(hlfiles);

    println!("==== {} ====", mlfile.name);
    mlir_out_dir.push(&mlfile.name);
    let mut f = fs::File::create(&mlir_out_dir)?;
    write!(f, "{}", mlfile.to_string())?;
    mlir_out_dir.pop();

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
        String::from(output)
    } else {
        let mut output_path = Path::new(&mlfile.name).to_path_buf();
        output_path.set_extension("ll");
        String::from(output_path.to_str().unwrap())
    };

    let mut out_path = out_dir;
    out_path.push(output);

    println!("Output Path -> {:?}", out_path);

    codegen.print_to_file(out_path)?;

    if let Some(fun_name) = matches.value_of("execute") {
        unsafe {
            let main: JitFunction<MainFunc> = codegen.execution_engine.get_function(fun_name)?;
            let _ = main.call();
        }
    }

    Ok(())
}
