use crate::parser::parser::{parse_from_file_path, parse_from_file_path_str};

use crate::ast::file::WizFile;
use crate::high_level_ir::type_resolver::{ResolverResult, TypeResolver};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::Ast2HLIR;
use crate::llvm_ir::codegen::{CodeGen, MLContext};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::HLIR2MLIR;
use crate::utils::stacked_hash_map::StackedHashMap;
use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::process::exit;

mod ast;
mod constants;
mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod parser;
mod utils;

fn get_builtin_syntax() -> Vec<WizFile> {
    let builtin_dir = std::fs::read_dir(Path::new("../builtin")).unwrap();
    builtin_dir
        .flatten()
        .map(|p| p.path())
        .filter(|path| path.ends_with("builtin.ll.wiz"))
        .map(|path| parse_from_file_path(path.as_path()))
        .flatten()
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .arg(Arg::with_name("input").required(true).multiple(true))
        .arg(Arg::with_name("output").short("o").takes_value(true));

    let matches = app.get_matches();
    let inputs = matches.values_of_lossy("input").unwrap();
    let output = matches.value_of("output");

    let builtin_syntax = get_builtin_syntax();

    let mut ast2hlir = Ast2HLIR::new();

    for builtin in builtin_syntax.iter() {
        ast2hlir.preload_types(builtin.clone());
    }

    let builtin_hlir: Vec<TypedFile> = builtin_syntax
        .into_iter()
        .map(|w| ast2hlir.file(w))
        .collect();

    let ast_files: Vec<WizFile> = inputs
        .iter()
        .map(|s| parse_from_file_path_str(s))
        .flatten()
        .collect();

    for ast_file in ast_files.iter() {
        ast2hlir.preload_types(ast_file.clone());
    }

    let hlfiles: Vec<TypedFile> = ast_files.into_iter().map(|f| ast2hlir.file(f)).collect();

    let mut type_resolver = TypeResolver::new();

    for hlir in builtin_hlir.iter() {
        type_resolver.detect_type(hlir.clone());
    }

    for hlir in hlfiles.iter() {
        type_resolver.detect_type(hlir.clone());
    }

    for hlir in builtin_hlir.iter() {
        type_resolver.preload_file(hlir.clone());
    }

    for hlir in hlfiles.iter() {
        type_resolver.preload_file(hlir.clone());
    }

    let hlfiles = hlfiles
        .into_iter()
        .map(|f| type_resolver.file(f))
        .collect::<ResolverResult<Vec<TypedFile>>>()?;

    let mut hlir2mlir = HLIR2MLIR::new();

    let builtin_mlir: Vec<MLFile> = builtin_hlir
        .into_iter()
        .map(|w| hlir2mlir.file(w))
        .collect();

    let mlfiles: Vec<MLFile> = hlfiles.into_iter().map(|f| hlir2mlir.file(f)).collect();

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
            ml_context: MLContext {
                struct_environment: StackedHashMap::from(HashMap::new()),
                local_environments: StackedHashMap::from(HashMap::new()),
                current_function: None,
            },
        };

        for m in builtin_mlir.iter() {
            codegen.file(m.clone());
        }

        println!("{:?}", &mlfile);

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
    }

    Ok(())
}
