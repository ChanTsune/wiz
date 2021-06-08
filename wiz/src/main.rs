use crate::parser::parser::parse_from_string;
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

mod ast;
mod parser;
mod llvm_ir;


/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    /**
    * Generate main function as entry point.
    */
    fn initialize(&self) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        let sum_function = self.module.get_function("sum").unwrap();
        let x = self.context.i64_type().const_int(1, false);
        let y = self.context.i64_type().const_int(2, false);

        let sum = self.builder.build_call(sum_function, &[x.into(), y.into()], "sum");


        let put_function = self.module.get_function("puts").unwrap();

        self.builder.build_call(put_function, &[sum.try_as_basic_value().left().unwrap().into()], "_");

        self.builder.build_return(None);
    }

    fn puts(&self) {
        let i32_type = self.context.i32_type();
        let i8p_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let fn_type = i32_type.fn_type(&[i8p_type.into()], false);
        let function = self.module.add_function("puts", fn_type, Some(Linkage::External));
    }

    fn builtin_print(&self) {

    }

    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
    /**
    * Write the LLVM IR to a file in the path.
    */
    fn print_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), LLVMString> {
        self.module.print_to_file(path)
    }
}


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

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }
    codegen.puts();
    codegen.initialize();
    codegen.print_to_file(Path::new("./sample.ll"));

    Ok(())
}
