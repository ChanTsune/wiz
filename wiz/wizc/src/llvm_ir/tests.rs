use crate::llvm_ir::codegen::{CodeGen, MLContext};
use crate::middle_level_ir::expr::{MLExpr, MLLiteral, MLName, MLReturn};
use crate::middle_level_ir::ml_decl::{MLDecl, MLFun, MLFunBody, MLVar};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::statement::MLStmt;
use crate::middle_level_ir::ml_type::{MLPrimitiveType, MLType, MLValueType};
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

#[test]
fn test_return_integer() {
    type MainFunc = unsafe extern "C" fn() -> u8;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            modifiers: vec![],
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::UInt8),
            body: Some(MLFunBody {
                body: vec![
                    MLStmt::Var(MLVar {
                        is_mute: false,
                        name: "i".to_string(),
                        type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::UInt8)),
                        value: MLExpr::Literal(MLLiteral::Integer {
                            value: "5".to_string(),
                            type_: MLValueType::Primitive(MLPrimitiveType::UInt8),
                        }),
                    }),
                    MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Name(MLName {
                            name: "i".to_string(),
                            type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::UInt8)),
                        }))),
                    })),
                ],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
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

#[test]
fn test_return_integer_literal() {
    type MainFunc = unsafe extern "C" fn() -> u8;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            modifiers: vec![],
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::UInt8),
            body: Some(MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                    value: Some(Box::new(MLExpr::Literal(MLLiteral::Integer {
                        value: "5".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::UInt8),
                    }))),
                }))],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
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

#[test]
fn test_return_floating_point() {
    type MainFunc = unsafe extern "C" fn() -> f64;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            modifiers: vec![],
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Double),
            body: Some(MLFunBody {
                body: vec![
                    MLStmt::Var(MLVar {
                        is_mute: false,
                        name: "d".to_string(),
                        type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::Double)),
                        value: MLExpr::Literal(MLLiteral::FloatingPoint {
                            value: "5.1".to_string(),
                            type_: MLValueType::Primitive(MLPrimitiveType::Double),
                        }),
                    }),
                    MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Name(MLName {
                            name: "d".to_string(),
                            type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::Double)),
                        }))),
                    })),
                ],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
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

    assert_eq!(result, 5.1);
}

#[test]
fn test_return_floating_point_literal() {
    type MainFunc = unsafe extern "C" fn() -> f64;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            modifiers: vec![],
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Double),
            body: Some(MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                    value: Some(Box::new(MLExpr::Literal(MLLiteral::FloatingPoint {
                        value: "5.1".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Double),
                    }))),
                }))],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let module = context.create_module(module_name);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
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

    assert_eq!(result, 5.1);
}
