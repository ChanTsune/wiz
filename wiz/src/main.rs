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
use std::env::args;
use inkwell::support::LLVMString;
use crate::llvm_ir::codegen::CodeGen;

mod ast;
mod parser;
mod llvm_ir;

fn main() -> Result<(), Box<dyn Error>> {
    let mut module_name = "main";
    for arg in args() {
        println!("{}", arg);
    }
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;
    let file = std::fs::File::open(Path::new("../helloworld.wiz"));
    let ast_file = parse_from_file(file.unwrap());

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }
    codegen.builtin_print();
    // codegen.initialize();
    println!("{:?}", ast_file.unwrap());
    // codegen.file(ast_file.unwrap());
    codegen.print_to_file(Path::new("./sample.ll"));

    Ok(())
}
