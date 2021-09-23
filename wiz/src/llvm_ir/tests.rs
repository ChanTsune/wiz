use inkwell::context::Context;
use inkwell::OptimizationLevel;
use crate::llvm_ir::codegen::{CodeGen, MLContext};
use crate::middle_level_ir::ml_file::MLFile;
use inkwell::execution_engine::JitFunction;
use crate::middle_level_ir::ml_decl::{MLDecl, MLFun, MLFunBody};
use crate::middle_level_ir::ml_type::{MLValueType, MLType};
use crate::middle_level_ir::ml_stmt::MLStmt;
use crate::middle_level_ir::ml_expr::{MLExpr, MLReturn, MLLiteral};

#[test]
fn test_fun_call() {
    type MainFunc = unsafe extern "C" fn() -> u8;
    let mlfile = MLFile { name: "name".to_string(), body: vec![
        MLDecl::Fun(MLFun {
            modifiers: vec![],
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(String::from("UInt8")),
            body: Some(MLFunBody { body: vec![
                MLStmt::Expr(MLExpr::Return(MLReturn { value: Some(Box::new(MLExpr::Literal(MLLiteral::Integer { value: "5".to_string(), type_: MLValueType::Primitive(String::from("UInt8")) }))), type_: MLType::Value(MLValueType::Primitive(String::from("UInt8"))) }))
            ] })
        })
    ] };
    let module_name = &mlfile.name;
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let mut codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
        ml_context: MLContext::new(),
    };

    codegen.file(mlfile.clone());

    let fun_name = "test";

    let result = unsafe {
        let main: JitFunction<MainFunc> = codegen.execution_engine.get_function(fun_name).unwrap();
        main.call()
    };

    assert_eq!(result, 5);
}