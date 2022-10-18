use crate::high_level_ir::AstLowering;
use crate::middle_level_ir::HLIR2MLIR;
use crate::ModuleId;
use wiz_arena::Arena;
use wiz_mir::expr::{
    MLCall, MLCallArg, MLExpr, MLLiteral, MLLiteralKind, MLName, MLUnaryOp, MLUnaryOpKind,
};
use wiz_mir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLReturn, MLStmt};
use wiz_session::{ParseSession, Session};
use wiz_syntax_parser::parser::wiz::parse_from_string;

fn check(source: &str, except: MLFile) {
    let session = ParseSession::default();
    let ast = parse_from_string::<&str>(&session, None, source, Some(&except.name)).unwrap();

    let mut session = Session::default();

    let mut arena = Arena::default();

    let mut ast2hlir = AstLowering::new(&mut session, &mut arena);

    let hl_ss = ast2hlir.lowing(ast, ModuleId::DUMMY).unwrap();

    let mut hlir2mlir = HLIR2MLIR::new(&session, &mut arena);

    let f = hlir2mlir.convert_from_file(hl_ss, false);

    assert_eq!(f, except);
}

#[test]
fn test_empty() {
    let source = "";
    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![],
        },
    );
}

#[test]
fn test_struct() {
    let source = r"
    struct A {
        val a: Int64
    }
    ";

    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![
                MLDecl::Struct(MLStruct {
                    name: "test::A".to_string(),
                    fields: vec![MLField {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::from(MLExpr::SizeOf(MLType::Value(
                                MLValueType::Struct("test::A".to_string()),
                            )))),
                        }))],
                    }),
                }),
            ],
        },
    );
}

#[test]
fn test_struct_init() {
    let source = r"
    struct A {
        val a: Int64
    }
    fun initA(): A {
        return A(a: 1)
    }
    ";

    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![
                MLDecl::Struct(MLStruct {
                    name: "test::A".to_string(),
                    fields: vec![MLField {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::initA".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::from(MLExpr::SizeOf(MLType::Value(
                                MLValueType::Struct("test::A".to_string()),
                            )))),
                        }))],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    name: "test::initA".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::from(MLExpr::Literal(MLLiteral {
                                kind: MLLiteralKind::Struct(vec![(
                                    "a".to_string(),
                                    MLExpr::Literal(MLLiteral {
                                        kind: MLLiteralKind::Integer("1".to_string()),
                                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                                    }),
                                )]),
                                type_: MLValueType::Struct(String::from("test::A")),
                            }))),
                        }))],
                    }),
                }),
            ],
        },
    );
}

#[test]
fn test_method_call() {
    let source = r"
    struct A {
        val a: Int64

        fun b(&self): Int64 {
            return 1
        }

        fun c(&self): Int64 {
            return self.b()
        }
    }
    fun sample() {
      val p = A(a: 1)
      p.c()
    }
    ";
    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![
                MLDecl::Struct(MLStruct {
                    name: "test::A".to_string(),
                    fields: vec![MLField {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::b##_#test::A".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "self".to_string(),
                        type_: MLValueType::Struct("test::A".to_string()),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::c##_#test::A".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "self".to_string(),
                        type_: MLValueType::Struct("test::A".to_string()),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::sample".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::b##_#test::A".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "self".to_string(),
                        type_: MLValueType::Struct("test::A".to_string()),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::Literal(MLLiteral {
                                kind: MLLiteralKind::Integer("1".to_string()),
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            }))),
                        }))],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::c##_#test::A".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "self".to_string(),
                        type_: MLValueType::Struct("test::A".to_string()),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::Call(MLCall {
                                target: MLName {
                                    name: "test::A::b##_#test::A".to_string(),
                                    type_: MLType::Function(MLFunctionType {
                                        arguments: vec![MLValueType::Struct("test::A".to_string())],
                                        return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                                    }),
                                },
                                args: vec![MLCallArg {
                                    arg: MLExpr::Name(MLName {
                                        name: "self".to_string(),
                                        type_: MLType::Value(MLValueType::Struct(
                                            "test::A".to_string(),
                                        )),
                                    }),
                                }],
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            }))),
                        }))],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    name: "test::A::size".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::USize),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::from(MLExpr::SizeOf(MLType::Value(
                                MLValueType::Struct("test::A".to_string()),
                            )))),
                        }))],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    name: "test::sample".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                    body: Some(MLFunBody {
                        body: vec![
                            MLStmt::Var(MLVar {
                                is_mute: false,
                                name: "p".to_string(),
                                type_: MLType::Value(MLValueType::Struct("test::A".to_string())),
                                value: MLExpr::Literal(MLLiteral {
                                    kind: MLLiteralKind::Struct(vec![(
                                        "a".to_string(),
                                        MLExpr::Literal(MLLiteral {
                                            kind: MLLiteralKind::Integer("1".to_string()),
                                            type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                                        }),
                                    )]),
                                    type_: MLValueType::Struct("test::A".to_string()),
                                }),
                            }),
                            MLStmt::Expr(MLExpr::Call(MLCall {
                                target: MLName {
                                    name: "test::A::c##_#test::A".to_string(),
                                    type_: MLType::Function(MLFunctionType {
                                        arguments: vec![MLValueType::Struct("test::A".to_string())],
                                        return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                                    }),
                                },
                                args: vec![MLCallArg {
                                    arg: MLExpr::Name(MLName {
                                        name: "p".to_string(),
                                        type_: MLType::Value(MLValueType::Struct(
                                            "test::A".to_string(),
                                        )),
                                    }),
                                }],
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            })),
                        ],
                    }),
                }),
            ],
        },
    )
}

#[test]
fn test_return_integer_literal() {
    let source = r"
    fun integer(): Int64 {
        return 1
    }
    ";

    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![
                MLDecl::Fun(MLFun {
                    name: "test::integer".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::integer".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::Literal(MLLiteral {
                                kind: MLLiteralKind::Integer("1".to_string()),
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            }))),
                        }))],
                    }),
                }),
            ],
        },
    );
}

#[test]
fn test_reference_dereference() {
    let source = r"
    fun reference_dereference(_ p: &Int64): Int64 {
        return *p
    }
    fun main() {
        val p = 1
        reference_dereference(&p)
    }
    ";
    check(
        source,
        MLFile {
            name: "test".to_string(),
            body: vec![
                MLDecl::Fun(MLFun {
                    name: "test::reference_dereference##_#&Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "p".to_string(),
                        type_: MLValueType::Reference(Box::new(MLType::Value(
                            MLValueType::Primitive(MLPrimitiveType::Int64),
                        ))),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "main".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    name: "test::reference_dereference##_#&Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "p".to_string(),
                        type_: MLValueType::Reference(Box::new(MLType::Value(
                            MLValueType::Primitive(MLPrimitiveType::Int64),
                        ))),
                    }],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::PrimitiveUnaryOp(MLUnaryOp {
                                target: Box::new(MLExpr::Name(MLName {
                                    name: "p".to_string(),
                                    type_: MLType::Value(MLValueType::Reference(Box::new(
                                        MLType::Value(MLValueType::Primitive(
                                            MLPrimitiveType::Int64,
                                        )),
                                    ))),
                                })),
                                kind: MLUnaryOpKind::DeRef,
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            }))),
                        }))],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    name: "main".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Unit),
                    body: Some(MLFunBody {
                        body: vec![
                            MLStmt::Var(MLVar {
                                is_mute: false,
                                name: "p".to_string(),
                                type_: MLType::Value(MLValueType::Primitive(
                                    MLPrimitiveType::Int64,
                                )),
                                value: MLExpr::Literal(MLLiteral {
                                    kind: MLLiteralKind::Integer("1".to_string()),
                                    type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                                }),
                            }),
                            MLStmt::Expr(MLExpr::Call(MLCall {
                                target: MLName {
                                    name: "test::reference_dereference##_#&Int64".to_string(),
                                    type_: MLType::Function(MLFunctionType {
                                        arguments: vec![MLValueType::Reference(Box::new(
                                            MLType::Value(MLValueType::Primitive(
                                                MLPrimitiveType::Int64,
                                            )),
                                        ))],
                                        return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                                    }),
                                },
                                args: vec![MLCallArg {
                                    arg: MLExpr::PrimitiveUnaryOp(MLUnaryOp {
                                        target: Box::new(MLExpr::Name(MLName {
                                            name: "p".to_string(),
                                            type_: MLType::Value(MLValueType::Primitive(
                                                MLPrimitiveType::Int64,
                                            )),
                                        })),
                                        kind: MLUnaryOpKind::Ref,
                                        type_: MLValueType::Reference(Box::new(MLType::Value(
                                            MLValueType::Primitive(MLPrimitiveType::Int64),
                                        ))),
                                    }),
                                }],
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            })),
                        ],
                    }),
                }),
            ],
        },
    )
}
