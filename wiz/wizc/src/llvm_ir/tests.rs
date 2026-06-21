use crate::llvm_ir::codegen::CodeGen;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use wiz_mir::expr::{
    MLArray, MLCall, MLCallArg, MLExpr, MLLiteral, MLLiteralKind, MLMember, MLName, MLSubscript,
};
use wiz_mir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLReturn, MLStmt};

#[test]
fn test_return_integer() {
    type MainFunc = unsafe extern "C" fn() -> u8;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::UInt8),
            body: Some(MLFunBody {
                body: vec![
                    MLStmt::Var(MLVar {
                        is_mute: false,
                        name: "i".to_string(),
                        type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::UInt8)),
                        value: MLExpr::Literal(MLLiteral {
                            kind: MLLiteralKind::Integer("5".to_string()),
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
    let mut codegen = CodeGen::new(&context, module_name, None);

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
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::UInt8),
            body: Some(MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                    value: Some(Box::new(MLExpr::Literal(MLLiteral {
                        kind: MLLiteralKind::Integer("5".to_string()),
                        type_: MLValueType::Primitive(MLPrimitiveType::UInt8),
                    }))),
                }))],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, None);

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
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Double),
            body: Some(MLFunBody {
                body: vec![
                    MLStmt::Var(MLVar {
                        is_mute: false,
                        name: "d".to_string(),
                        type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::Double)),
                        value: MLExpr::Literal(MLLiteral {
                            kind: MLLiteralKind::FloatingPoint("5.1".to_string()),
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
    let mut codegen = CodeGen::new(&context, module_name, None);

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
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Double),
            body: Some(MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                    value: Some(Box::new(MLExpr::Literal(MLLiteral {
                        kind: MLLiteralKind::FloatingPoint("5.1".to_string()),
                        type_: MLValueType::Primitive(MLPrimitiveType::Double),
                    }))),
                }))],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, None);

    codegen.file(mlfile.clone());

    let fun_name = "test";

    let result = unsafe {
        let main: JitFunction<MainFunc> = codegen.execution_engine.get_function(fun_name).unwrap();
        main.call()
    };

    assert_eq!(result, 5.1);
}

#[test]
fn test_return_global_constant() {
    type MainFunc = unsafe extern "C" fn() -> u8;
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![
            MLDecl::Var(MLVar {
                is_mute: false,
                name: "i".to_string(),
                type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::UInt8)),
                value: MLExpr::Literal(MLLiteral {
                    kind: MLLiteralKind::Integer("5".to_string()),
                    type_: MLValueType::Primitive(MLPrimitiveType::UInt8),
                }),
            }),
            MLDecl::Fun(MLFun {
                name: "test".to_string(),
                arg_defs: vec![],
                return_type: MLValueType::Primitive(MLPrimitiveType::UInt8),
                body: Some(MLFunBody {
                    body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Name(MLName {
                            name: "i".to_string(),
                            type_: MLType::Value(MLValueType::Primitive(MLPrimitiveType::UInt8)),
                        }))),
                    }))],
                }),
            }),
        ],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, None);

    codegen.file(mlfile.clone());

    let fun_name = "test";

    let result = unsafe {
        let main: JitFunction<MainFunc> = codegen.execution_engine.get_function(fun_name).unwrap();
        main.call()
    };

    assert_eq!(result, 5);
}

#[test]
fn test_reference_self_argument_member_access() {
    type MainFunc = unsafe extern "C" fn() -> i64;
    let struct_type = MLValueType::Struct("test::A".to_string());
    let self_ref_type = MLValueType::Reference(Box::new(MLType::Value(struct_type.clone())));
    let int64_type = MLValueType::Primitive(MLPrimitiveType::Int64);
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![
            MLDecl::Struct(MLStruct {
                name: "test::A".to_string(),
                fields: vec![MLField {
                    name: "a".to_string(),
                    type_: int64_type.clone(),
                }],
            }),
            MLDecl::Fun(MLFun {
                name: "test::A::getA##_#&test::A".to_string(),
                arg_defs: vec![MLArgDef {
                    name: "self".to_string(),
                    type_: self_ref_type.clone(),
                }],
                return_type: int64_type.clone(),
                body: Some(MLFunBody {
                    body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Member(MLMember {
                            target: Box::new(MLExpr::Name(MLName {
                                name: "self".to_string(),
                                type_: MLType::Value(self_ref_type.clone()),
                            })),
                            name: "a".to_string(),
                            type_: MLType::Value(int64_type.clone()),
                        }))),
                    }))],
                }),
            }),
            MLDecl::Fun(MLFun {
                name: "test".to_string(),
                arg_defs: vec![],
                return_type: int64_type.clone(),
                body: Some(MLFunBody {
                    body: vec![
                        MLStmt::Var(MLVar {
                            is_mute: false,
                            name: "a".to_string(),
                            type_: MLType::Value(struct_type.clone()),
                            value: MLExpr::Literal(MLLiteral {
                                kind: MLLiteralKind::Struct(vec![(
                                    "a".to_string(),
                                    MLExpr::Literal(MLLiteral {
                                        kind: MLLiteralKind::Integer("7".to_string()),
                                        type_: int64_type.clone(),
                                    }),
                                )]),
                                type_: struct_type,
                            }),
                        }),
                        MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::Call(MLCall {
                                target: MLName {
                                    name: "test::A::getA##_#&test::A".to_string(),
                                    type_: MLType::Function(MLFunctionType {
                                        arguments: vec![self_ref_type],
                                        return_type: int64_type.clone(),
                                    }),
                                },
                                args: vec![MLCallArg {
                                    arg: MLExpr::Name(MLName {
                                        name: "a".to_string(),
                                        type_: MLType::Value(MLValueType::Struct(
                                            "test::A".to_string(),
                                        )),
                                    }),
                                }],
                                type_: int64_type,
                            }))),
                        })),
                    ],
                }),
            }),
        ],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, None);

    codegen.file(mlfile);

    let result = unsafe {
        let main: JitFunction<MainFunc> = codegen.execution_engine.get_function("test").unwrap();
        main.call()
    };

    assert_eq!(result, 7);
}

#[test]
fn test_array_literal_stores_elements_in_bounds() {
    type MainFunc = unsafe extern "C" fn() -> i32;
    let int32_type = MLValueType::Primitive(MLPrimitiveType::Int32);
    let array_type = MLValueType::Array(Box::new(int32_type.clone()), 2);
    let mlfile = MLFile {
        name: "name".to_string(),
        body: vec![MLDecl::Fun(MLFun {
            name: "test".to_string(),
            arg_defs: vec![],
            return_type: int32_type.clone(),
            body: Some(MLFunBody {
                body: vec![
                    MLStmt::Var(MLVar {
                        is_mute: false,
                        name: "a".to_string(),
                        type_: MLType::Value(array_type.clone()),
                        value: MLExpr::Array(MLArray {
                            elements: vec![
                                MLExpr::Literal(MLLiteral {
                                    kind: MLLiteralKind::Integer("1".to_string()),
                                    type_: int32_type.clone(),
                                }),
                                MLExpr::Literal(MLLiteral {
                                    kind: MLLiteralKind::Integer("7".to_string()),
                                    type_: int32_type.clone(),
                                }),
                            ],
                            type_: array_type.clone(),
                        }),
                    }),
                    MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::PrimitiveSubscript(MLSubscript {
                            target: Box::new(MLExpr::Name(MLName {
                                name: "a".to_string(),
                                type_: MLType::Value(array_type),
                            })),
                            index: Box::new(MLExpr::Literal(MLLiteral {
                                kind: MLLiteralKind::Integer("1".to_string()),
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            })),
                            type_: int32_type,
                        }))),
                    })),
                ],
            }),
        })],
    };
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, None);

    codegen.file(mlfile);

    let result = unsafe {
        let main: JitFunction<MainFunc> = codegen.execution_engine.get_function("test").unwrap();
        main.call()
    };

    assert_eq!(result, 7);
}
