use crate::parser::parser::{parse_from_string, parse_from_file};
use crate::parser::nom::expression::{expr};

use inkwell::{OptimizationLevel, AddressSpace};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Module, Linkage};
use inkwell::targets::{InitializationConfig, Target};
use std::error::Error;
use std::path::Path;
use crate::llvm_ir::codegen::CodeGen;
use clap::{App, Arg};

mod ast;
mod parser;
mod llvm_ir;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .arg(Arg::with_name("input")
            .required(true)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .default_value("out.ll")
        );

    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let mut module_name = "main";
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let file = std::fs::File::open(Path::new(input));
    let ast_file = parse_from_file(file.unwrap());

    codegen.builtin_print();
    // println!("{:?}", ast_file.unwrap());
    codegen.file(ast_file.unwrap());
    codegen.print_to_file(Path::new(output));

    Ok(())
}
