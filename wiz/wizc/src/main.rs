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
use std::path::Path;
use std::process::exit;
use std::result;
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

fn get_builtin_syntax() -> parser::result::Result<Vec<WizFile>> {
    let builtin_dir = std::fs::read_dir(Path::new("../../builtin")).unwrap();
    builtin_dir
        .flatten()
        .map(|p| p.path())
        .filter(|path| path.ends_with("builtin.ll.wiz"))
        .map(|path| parse_from_file_path(path.as_path()))
        .collect::<parser::result::Result<Vec<WizFile>>>()
}

fn main() -> result::Result<(), Box<dyn Error>> {
    let app = App::new("wizc")
        .arg(Arg::with_name("input").required(true).multiple(true))
        .arg(Arg::with_name("output").short("o").takes_value(true))
        .arg(Arg::with_name("execute").short("e").takes_value(true));

    let matches = app.get_matches();
    let inputs = matches.values_of_lossy("input").unwrap();
    let output = matches.value_of("output");

    println!("=== parse files ===");

    let builtin_syntax = get_builtin_syntax()?;

    let std_package_source_set = read_package_from_path(Path::new("../../std"))?;

    println!("=== convert to hlir ===");

    let mut ast2hlir = Ast2HLIR::new();

    let builtin_hlir: Vec<TypedFile> = builtin_syntax
        .into_iter()
        .map(|w| ast2hlir.file(w))
        .collect();

    let std_hlir = ast2hlir.source_set(std_package_source_set);

    let ast_files = inputs
        .iter()
        .map(|s| parse_from_file_path_str(s))
        .collect::<parser::result::Result<Vec<_>>>()?;

    let hlfiles: Vec<TypedFile> = ast_files.into_iter().map(|f| ast2hlir.file(f)).collect();

    println!("=== resolve type ===");

    let mut type_resolver = TypeResolver::new();
    type_resolver.global_use(vec!["builtin.ll"]);

    println!("===== detect types =====");
    // detect types
    for hlir in builtin_hlir.iter() {
        type_resolver.detect_type(hlir)?;
    }

    type_resolver.detect_type_from_source_set(&std_hlir)?;

    for hlir in hlfiles.iter() {
        type_resolver.detect_type(hlir)?;
    }

    println!("===== preload decls =====");
    // preload decls
    for hlir in builtin_hlir.iter() {
        type_resolver.preload_file(hlir.clone())?;
    }

    type_resolver.preload_source_set(std_hlir.clone())?;

    for hlir in hlfiles.iter() {
        type_resolver.preload_file(hlir.clone())?;
    }

    println!("===== resolve types =====");
    // resolve types
    let builtin_hlir = builtin_hlir
        .into_iter()
        .map(|f| type_resolver.file(f))
        .collect::<Result<Vec<TypedFile>>>()?;

    let std_hlir = type_resolver.source_set(std_hlir)?;

    let hlfiles = hlfiles
        .into_iter()
        .map(|f| type_resolver.file(f))
        .collect::<Result<Vec<TypedFile>>>()?;

    println!("===== convert to mlir =====");

    let mut hlir2mlir = HLIR2MLIR::new();

    let builtin_mlir: Vec<MLFile> = builtin_hlir
        .into_iter()
        .map(|w| hlir2mlir.file(w))
        .collect();

    let std_mlir = hlir2mlir.source_set(std_hlir);

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

        for m in builtin_mlir.iter() {
            codegen.file(m.clone());
        }

        for m in std_mlir.iter() {
            codegen.file(m.clone());
        }

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

        codegen.file(mlfile.clone());

        println!("Output Path -> {:?}", output);

        codegen.print_to_file(Path::new(&output))?;

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
