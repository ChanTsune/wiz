use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::typed_file::TypedSourceSet;
use crate::high_level_ir::Ast2HLIR;
use crate::middle_level_ir::HLIR2MLIR;
use wiz_mir::expr::{MLCall, MLCallArg, MLExpr, MLLiteral, MLMember, MLName};
use wiz_mir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLAssignmentStmt, MLReturn, MLStmt};
use wiz_syntax::parser::wiz::parse_from_string;

fn check(source: &str, except: MLFile) {
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let hl_file = resolver.file(file).unwrap();

    let mut hlir2mlir = HLIR2MLIR::new();

    let f = hlir2mlir.convert_from_source_set(TypedSourceSet::File(hl_file));

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
                    modifiers: vec![],
                    name: "test::A::init##a#Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    modifiers: vec![],
                    name: "test::A::init##a#Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: Some(MLFunBody {
                        body: vec![
                            MLStmt::Var(MLVar {
                                is_mute: true,
                                name: "self".to_string(),
                                type_: MLType::Value(MLValueType::Struct(String::from("test::A"))),
                                value: MLExpr::Literal(MLLiteral::Struct {
                                    type_: MLValueType::Struct(String::from("test::A")),
                                }),
                            }),
                            MLStmt::Assignment(MLAssignmentStmt {
                                target: MLExpr::Member(MLMember {
                                    target: Box::new(MLExpr::Name(MLName {
                                        name: "self".to_string(),
                                        type_: MLType::Value(MLValueType::Struct(String::from(
                                            "test::A",
                                        ))),
                                    })),
                                    name: "a".to_string(),
                                    type_: MLType::Value(MLValueType::Primitive(
                                        MLPrimitiveType::Int64,
                                    )),
                                }),
                                value: MLExpr::Name(MLName {
                                    name: "a".to_string(),
                                    type_: MLType::Value(MLValueType::Primitive(
                                        MLPrimitiveType::Int64,
                                    )),
                                }),
                            }),
                            MLStmt::Expr(MLExpr::Return(MLReturn {
                                value: Some(Box::new(MLExpr::Name(MLName {
                                    name: String::from("self"),
                                    type_: MLType::Value(MLValueType::Struct(String::from(
                                        "test::A",
                                    ))),
                                }))),
                            })),
                        ],
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
        return A.init(a: 1)
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
                    modifiers: vec![],
                    name: "test::A::init##a#Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    modifiers: vec![],
                    name: "test::initA".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    modifiers: vec![],
                    name: "test::A::init##a#Int64".to_string(),
                    arg_defs: vec![MLArgDef {
                        name: "a".to_string(),
                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                    }],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: Some(MLFunBody {
                        body: vec![
                            MLStmt::Var(MLVar {
                                is_mute: true,
                                name: "self".to_string(),
                                type_: MLType::Value(MLValueType::Struct(String::from("test::A"))),
                                value: MLExpr::Literal(MLLiteral::Struct {
                                    type_: MLValueType::Struct(String::from("test::A")),
                                }),
                            }),
                            MLStmt::Assignment(MLAssignmentStmt {
                                target: MLExpr::Member(MLMember {
                                    target: Box::new(MLExpr::Name(MLName {
                                        name: "self".to_string(),
                                        type_: MLType::Value(MLValueType::Struct(String::from(
                                            "test::A",
                                        ))),
                                    })),
                                    name: "a".to_string(),
                                    type_: MLType::Value(MLValueType::Primitive(
                                        MLPrimitiveType::Int64,
                                    )),
                                }),
                                value: MLExpr::Name(MLName {
                                    name: "a".to_string(),
                                    type_: MLType::Value(MLValueType::Primitive(
                                        MLPrimitiveType::Int64,
                                    )),
                                }),
                            }),
                            MLStmt::Expr(MLExpr::Return(MLReturn {
                                value: Some(Box::new(MLExpr::Name(MLName {
                                    name: String::from("self"),
                                    type_: MLType::Value(MLValueType::Struct(String::from(
                                        "test::A",
                                    ))),
                                }))),
                            })),
                        ],
                    }),
                }),
                MLDecl::Fun(MLFun {
                    modifiers: vec![],
                    name: "test::initA".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Struct(String::from("test::A")),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::from(MLExpr::Call(MLCall {
                                target: MLName {
                                    name: "test::A::init##a#Int64".to_string(),
                                    type_: MLType::Function(MLFunctionType {
                                        arguments: vec![MLValueType::Primitive(
                                            MLPrimitiveType::Int64,
                                        )],
                                        return_type: MLValueType::Struct(String::from("test::A")),
                                    }),
                                },
                                args: vec![MLCallArg {
                                    arg: MLExpr::Literal(MLLiteral::Integer {
                                        value: "1".to_string(),
                                        type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                                    }),
                                }],
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
                    modifiers: vec![],
                    name: "test::integer".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: None,
                }),
                MLDecl::Fun(MLFun {
                    modifiers: vec![],
                    name: "test::integer".to_string(),
                    arg_defs: vec![],
                    return_type: MLValueType::Primitive(MLPrimitiveType::Int64),
                    body: Some(MLFunBody {
                        body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                            value: Some(Box::new(MLExpr::Literal(MLLiteral::Integer {
                                value: "1".to_string(),
                                type_: MLValueType::Primitive(MLPrimitiveType::Int64),
                            }))),
                        }))],
                    }),
                }),
            ],
        },
    );
}
