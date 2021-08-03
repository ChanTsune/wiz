use crate::parser::parser::{parse_from_file, parse_from_string};

use crate::ast::file::WizFile;
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
use std::fs::read_to_string;
use std::path::Path;

mod ast;
mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod parser;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .arg(Arg::with_name("input").required(true).multiple(true))
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .default_value("out.ll"),
        );

    let matches = app.get_matches();
    let inputs = matches.values_of_lossy("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let input: &str = &inputs[0];

    let mut ast2hlir = Ast2HLIR::new();

    let builtin_dir = std::fs::read_dir(Path::new("../builtin")).unwrap();
    let mut builtin_syntax: Vec<WizFile> = vec![];
    for path in builtin_dir {
        let path = path.unwrap().path();
        if path.ends_with("builtin.ll.wiz") {
            println!("{:?}", &path);
            let builtin = parse_from_string(read_to_string(path).unwrap());
            builtin_syntax.push(builtin.clone());
            println!("{:?}", &builtin);
            ast2hlir.preload_types(builtin);
        }
    }

    let file = std::fs::File::open(Path::new(input));
    let ast_file = parse_from_file(file.unwrap()).unwrap();

    let builtin_hlir: Vec<TypedFile> = builtin_syntax
        .into_iter()
        .map(|w| ast2hlir.file(w.syntax))
        .collect();

    ast2hlir.preload_types(ast_file.clone());
    let hlfile = ast2hlir.file(ast_file.syntax);

    println!("{:?}", &hlfile);

    let mut hlir2mlir = HLIR2MLIR::new();

    let builtin_mlir: Vec<MLFile> = builtin_hlir
        .into_iter()
        .map(|w| hlir2mlir.file(w))
        .collect();

    let ml = hlir2mlir.file(hlfile);

    let module_name = "main";
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

    for m in builtin_mlir {
        codegen.file(m);
    }
    println!("{:?}", &ml);

    codegen.file(ml);
    let _ = codegen.print_to_file(Path::new(output));

    Ok(())
}
