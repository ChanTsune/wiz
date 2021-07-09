use crate::parser::parser::{parse_from_file, parse_from_string};

use crate::high_level_ir::Ast2HLIR;
use crate::llvm_ir::codegen::CodeGen;
use crate::middle_level_ir::HLIR2MLIR;
use clap::{App, Arg};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

mod ast;
mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .default_value("out.ll"),
        );

    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let mut module_name = "main";
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let mut codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
        local_environments: vec![],
        current_function: None,
    };

    let mut ast2hlir = Ast2HLIR::new();

    let builtin_dir = std::fs::read_dir(Path::new("../builtin")).unwrap();
    for path in builtin_dir {
        let path = path.unwrap().path();
        if path.ends_with("builtin.ll.wiz") {
            println!("{:?}", &path);
            let built_in = parse_from_string(read_to_string(path).unwrap());
            let builtin_clone = built_in.clone();
            ast2hlir.preload_types(builtin_clone);
            println!("{:?}", &built_in);
            codegen.file(built_in.syntax);
        }
    }

    let file = std::fs::File::open(Path::new(input));
    let ast_file = parse_from_file(file.unwrap()).unwrap().syntax;
    let ast = ast_file.clone();

    // println!("{:?}", &ast_file.unwrap());
    codegen.file(ast_file);
    let _ = codegen.print_to_file(Path::new(output));

    let hlfile = ast2hlir.file(ast);

    let mut hlir2mlir = HLIR2MLIR::new();

    let ml = hlir2mlir.file(hlfile);
    println!("{:?}", ml);

    Ok(())
}
