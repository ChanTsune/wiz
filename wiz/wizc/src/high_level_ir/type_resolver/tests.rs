use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::AstLowering;
use wiz_hir::typed_decl::{
    TypedArgDef, TypedDecl, TypedDeclKind, TypedFun, TypedFunBody, TypedMemberFunction,
    TypedStoredProperty, TypedStruct, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExprKind, TypedIf,
    TypedInstanceMember, TypedLiteral, TypedName, TypedPrefixUnaryOp, TypedPrefixUnaryOperator,
    TypedReturn, TypedSubscript, TypedUnaryOp,
};
use wiz_hir::typed_file::TypedFile;
use wiz_hir::typed_stmt::{TypedBlock, TypedStmt};
use wiz_hir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedValueType,
};
use wiz_session::Session;
use wiz_syntax_parser::parser::wiz::parse_from_string;

fn check(source: &str, typed_file: TypedFile) {
    let ast = parse_from_string(source).unwrap();

    let mut arena = ResolverArena::default();

    let mut ast2hlir = AstLowering::new(&mut arena);

    let mut file = ast2hlir.file(ast);
    file.name = typed_file.name.clone();

    let mut session = Session::new();
    let mut arena = ResolverArena::default();

    let mut resolver = TypeResolver::new(&mut session, &mut arena);
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(&file).unwrap();
    let f = resolver.file(file);

    assert_eq!(f, Ok(typed_file));
}

#[test]
fn test_empty() {
    let source = "";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![],
        },
    );
}

#[test]
fn test_unsafe_pointer() {
    let source = r"
        struct A {
            val a: *UInt8
        }
        fun function(_ a: A): Unit {
            val a = a.a
        }
        ";
    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::unsafe_pointer(TypedType::uint8()),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::unsafe_pointer(TypedType::uint8())),
                                    value: TypedExprKind::Member(TypedInstanceMember {
                                        target: Box::new(TypedExprKind::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::new()),
                                            name: "a".to_string(),
                                            type_: Some(TypedType::Value(TypedValueType::Value(
                                                TypedNamedValueType {
                                                    package: TypedPackage::Resolved(Package::from(
                                                        &vec!["test"],
                                                    )),
                                                    name: "A".to_string(),
                                                    type_args: None,
                                                },
                                            ))),
                                            type_arguments: None,
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::unsafe_pointer(TypedType::uint8())),
                                    }),
                                }),
                            })],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
            ],
        },
    );
}

#[test]
fn test_struct_stored_property() {
    let source = r"
        struct A {
            val a: Int64
        }
        fun function(_ a: A) {
            val a = a.a
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExprKind::Member(TypedInstanceMember {
                                        target: Box::new(TypedExprKind::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::new()),
                                            name: "a".to_string(),
                                            type_: Some(TypedType::Value(TypedValueType::Value(
                                                TypedNamedValueType {
                                                    package: TypedPackage::Resolved(Package::from(
                                                        &vec!["test"],
                                                    )),
                                                    name: "A".to_string(),
                                                    type_args: None,
                                                },
                                            ))),
                                            type_arguments: None,
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::int64()),
                                    }),
                                }),
                            })],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
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
        fun function(_ a: A) {
            val a = A(a:1)
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![],
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::Value(TypedValueType::Value(
                                        TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(&vec![
                                                "test",
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None,
                                        },
                                    ))),
                                    value: TypedExprKind::Call(TypedCall {
                                        target: Box::new(TypedExprKind::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::from(&vec![
                                                "test",
                                            ])),
                                            name: "A".to_string(),
                                            type_: Some(TypedType::Type(Box::new(
                                                TypedType::Value(TypedValueType::Value(
                                                    TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&vec!["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    },
                                                )),
                                            ))),
                                            type_arguments: None,
                                        })),
                                        args: vec![TypedCallArg {
                                            label: Some(String::from("a")),
                                            arg: Box::new(TypedExprKind::Literal(
                                                TypedLiteral::Integer {
                                                    value: "1".to_string(),
                                                    type_: Some(TypedType::int64()),
                                                },
                                            )),
                                            is_vararg: false,
                                        }],
                                        type_: Some(TypedType::Value(TypedValueType::Value(
                                            TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &vec!["test"],
                                                )),
                                                name: "A".to_string(),
                                                type_args: None,
                                            },
                                        ))),
                                    }),
                                }),
                            })],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
            ],
        },
    );
}

#[test]
fn test_struct_member_function() {
    let source = r"
        struct A {
            val a: Int64

            fun getA(&self): Int64 {
                return self.a
            }
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Struct(TypedStruct {
                    name: "A".to_string(),
                    type_params: None,
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64(),
                    }],
                    computed_properties: vec![],
                    member_functions: vec![TypedMemberFunction {
                        name: "getA".to_string(),
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "self".to_string(),
                            type_: TypedType::Value(
                                // TODO: Reference
                                TypedValueType::Value(TypedNamedValueType {
                                    package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                    name: "A".to_string(),
                                    type_args: None,
                                }),
                            ),
                        }],
                        type_params: None,
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExprKind::Return(TypedReturn {
                                value: Some(Box::new(TypedExprKind::Member(TypedInstanceMember {
                                    target: Box::new(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "self".to_string(),
                                        type_: Some(TypedType::Value(TypedValueType::Value(
                                            TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &vec!["test"],
                                                )),
                                                name: "A".to_string(),
                                                type_args: None,
                                            },
                                        ))),
                                        type_arguments: None,
                                    })),
                                    name: "a".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::int64()),
                                }))),
                            }))],
                        })),
                        return_type: Some(TypedType::int64()),
                    }],
                }),
            }],
        },
    );
}

#[test]
fn test_struct_member_function_call() {
    let source = r"
        struct A {
            val a: Int64

            fun getA(&self): Int64 {
                return self.a
            }
        }

        fun function(_ a: A) {
            a.getA()
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![TypedMemberFunction {
                            name: "getA".to_string(),
                            arg_defs: vec![TypedArgDef {
                                label: "_".to_string(),
                                name: "self".to_string(),
                                type_: TypedType::Value(
                                    // TODO: Reference
                                    TypedValueType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(&vec![
                                            "test",
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None,
                                    }),
                                ),
                            }],
                            type_params: None,
                            body: Some(TypedFunBody::Block(TypedBlock {
                                body: vec![TypedStmt::Expr(TypedExprKind::Return(TypedReturn {
                                    value: Some(Box::new(TypedExprKind::Member(
                                        TypedInstanceMember {
                                            target: Box::new(TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::new()),
                                                name: "self".to_string(),
                                                type_: Some(TypedType::Value(
                                                    TypedValueType::Value(TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&vec!["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    }),
                                                )),
                                                type_arguments: None,
                                            })),
                                            name: "a".to_string(),
                                            is_safe: false,
                                            type_: Some(TypedType::int64()),
                                        },
                                    ))),
                                }))],
                            })),
                            return_type: Some(TypedType::int64()),
                        }],
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExprKind::Call(TypedCall {
                                target: Box::new(TypedExprKind::Member(TypedInstanceMember {
                                    target: Box::new(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "a".to_string(),
                                        type_: Some(TypedType::Value(TypedValueType::Value(
                                            TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &vec!["test"],
                                                )),
                                                name: "A".to_string(),
                                                type_args: None,
                                            },
                                        ))),
                                        type_arguments: None,
                                    })),
                                    name: "getA".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                        arguments: vec![TypedArgType {
                                            label: "_".to_string(),
                                            typ: TypedType::Value(TypedValueType::Value(
                                                TypedNamedValueType {
                                                    package: TypedPackage::Resolved(Package::from(
                                                        &vec!["test"],
                                                    )),
                                                    name: "A".to_string(),
                                                    type_args: None,
                                                },
                                            )),
                                        }],
                                        return_type: TypedType::int64(),
                                    }))),
                                })),
                                args: vec![],
                                type_: Some(TypedType::int64()),
                            }))],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
            ],
        },
    );
}

#[test]
fn test_expr_function_with_no_arg() {
    let source = r"
        fun function() = 1
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "function".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Expr(TypedExprKind::Literal(
                        TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        },
                    ))),
                    return_type: Some(TypedType::int64()),
                }),
            }],
        },
    );
}

#[test]
fn test_expr_function_with_arg() {
    let source = r"
        fun function(_ i:Int32) = i
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "function".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "i".to_string(),
                        type_: TypedType::int32(),
                    }],
                    body: Some(TypedFunBody::Expr(TypedExprKind::Name(TypedName {
                        package: TypedPackage::Resolved(Package::new()),
                        name: "i".to_string(),
                        type_: Some(TypedType::int32()),
                        type_arguments: None,
                    }))),
                    return_type: Some(TypedType::int32()),
                }),
            }],
        },
    );
}

#[test]
fn test_function_call() {
    let source = r"
        fun target_function() = 1
        fun main() {
            target_function()
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "target_function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Expr(TypedExprKind::Literal(
                            TypedLiteral::Integer {
                                value: "1".to_string(),
                                type_: Some(TypedType::int64()),
                            },
                        ))),
                        return_type: Some(TypedType::int64()),
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "main".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExprKind::Call(TypedCall {
                                target: Box::new(TypedExprKind::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::from(&vec!["test"])),
                                    name: "target_function".to_string(),
                                    type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                        arguments: vec![],
                                        return_type: TypedType::int64(),
                                    }))),
                                    type_arguments: None,
                                })),
                                args: vec![],
                                type_: Some(TypedType::int64()),
                            }))],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
            ],
        },
    );
}

#[test]
fn test_return_integer_literal() {
    let source = r"
        fun sample(): Int64 {
            return 1
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExprKind::Return(TypedReturn {
                            value: Some(Box::new(TypedExprKind::Literal(TypedLiteral::Integer {
                                value: "1".to_string(),
                                type_: Some(TypedType::int64()),
                            }))),
                        }))],
                    })),
                    return_type: Some(TypedType::int64()),
                }),
            }],
        },
    );
}

#[test]
fn test_return_floating_point_literal() {
    let source = r"
        fun sample(): Double {
            return 0.5
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExprKind::Return(TypedReturn {
                            value: Some(Box::new(TypedExprKind::Literal(
                                TypedLiteral::FloatingPoint {
                                    value: "0.5".to_string(),
                                    type_: Some(TypedType::double()),
                                },
                            ))),
                        }))],
                    })),
                    return_type: Some(TypedType::double()),
                }),
            }],
        },
    );
}

#[test]
fn test_binop() {
    let source = r"
        fun sample() {
            1 + 2
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExprKind::BinOp(TypedBinOp {
                            left: Box::new(TypedExprKind::Literal(TypedLiteral::Integer {
                                value: "1".to_string(),
                                type_: Some(TypedType::int64()),
                            })),
                            operator: TypedBinaryOperator::Add,
                            right: Box::new(TypedExprKind::Literal(TypedLiteral::Integer {
                                value: "2".to_string(),
                                type_: Some(TypedType::int64()),
                            })),
                            type_: Some(TypedType::int64()),
                        }))],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            }],
        },
    );
}

#[test]
fn test_subscript() {
    let source = r"
        fun get_first(_ p: *UInt8) = p[0]
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "get_first".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "p".to_string(),
                        type_: TypedType::unsafe_pointer(TypedType::uint8()),
                    }],
                    body: Option::from(TypedFunBody::Expr(TypedExprKind::Subscript(
                        TypedSubscript {
                            target: Box::new(TypedExprKind::Name(TypedName {
                                package: TypedPackage::Resolved(Package::new()),
                                name: "p".to_string(),
                                type_: Some(TypedType::unsafe_pointer(TypedType::uint8())),
                                type_arguments: None,
                            })),
                            indexes: vec![TypedExprKind::Literal(TypedLiteral::Integer {
                                value: "0".to_string(),
                                type_: Some(TypedType::int64()),
                            })],
                            type_: Some(TypedType::uint8()),
                        },
                    ))),
                    return_type: Some(TypedType::uint8()),
                }),
            }],
        },
    );
}

#[test]
fn test_if_else() {
    let source = r"
        fun test_if(i:Int64): Int64 {
            return if i <= 0 { 0 } else { i }
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "test_if".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![TypedArgDef {
                        label: "i".to_string(),
                        name: "i".to_string(),
                        type_: TypedType::int64(),
                    }],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExprKind::Return(TypedReturn {
                            value: Some(Box::new(TypedExprKind::If(TypedIf {
                                condition: Box::new(TypedExprKind::BinOp(TypedBinOp {
                                    left: Box::new(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "i".to_string(),
                                        type_: Some(TypedType::int64()),
                                        type_arguments: None,
                                    })),
                                    operator: TypedBinaryOperator::LessThanEqual,
                                    right: Box::new(TypedExprKind::Literal(
                                        TypedLiteral::Integer {
                                            value: "0".to_string(),
                                            type_: Some(TypedType::int64()),
                                        },
                                    )),
                                    type_: Some(TypedType::bool()),
                                })),
                                body: TypedBlock {
                                    body: vec![TypedStmt::Expr(TypedExprKind::Literal(
                                        TypedLiteral::Integer {
                                            value: "0".to_string(),
                                            type_: Some(TypedType::int64()),
                                        },
                                    ))],
                                },
                                type_: Some(TypedType::int64()),
                                else_body: Some(TypedBlock {
                                    body: vec![TypedStmt::Expr(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "i".to_string(),
                                        type_: Some(TypedType::int64()),
                                        type_arguments: None,
                                    }))],
                                }),
                            }))),
                        }))],
                    })),
                    return_type: Some(TypedType::int64()),
                }),
            }],
        },
    );
}

#[test]
fn test_if() {
    let source = r"
        fun test_if(i:Int64) {
            if i <= 0 {
                val p = 1
            }
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "test_if".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![TypedArgDef {
                        label: "i".to_string(),
                        name: "i".to_string(),
                        type_: TypedType::int64(),
                    }],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExprKind::If(TypedIf {
                            condition: Box::new(TypedExprKind::BinOp(TypedBinOp {
                                left: Box::new(TypedExprKind::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "i".to_string(),
                                    type_: Some(TypedType::int64()),
                                    type_arguments: None,
                                })),
                                operator: TypedBinaryOperator::LessThanEqual,
                                right: Box::new(TypedExprKind::Literal(TypedLiteral::Integer {
                                    value: "0".to_string(),
                                    type_: Some(TypedType::int64()),
                                })),
                                type_: Some(TypedType::bool()),
                            })),
                            body: TypedBlock {
                                body: vec![TypedStmt::Decl(TypedDecl {
                                    annotations: Default::default(),
                                    package: Package::new(),
                                    modifiers: vec![],
                                    kind: TypedDeclKind::Var(TypedVar {
                                        is_mut: false,
                                        name: "p".to_string(),
                                        type_: Some(TypedType::int64()),
                                        value: TypedExprKind::Literal(TypedLiteral::Integer {
                                            value: "1".to_string(),
                                            type_: Some(TypedType::int64()),
                                        }),
                                    }),
                                })],
                            },
                            type_: Some(TypedType::noting()),
                            else_body: None,
                        }))],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            }],
        },
    );
}

#[test]
fn test_reference_dereference() {
    let source = r"
    fun test_reference_dereference() {
        val a = 1
        val b = &a
        val c = *b
    }
    ";
    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "test_reference_dereference".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![
                            TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExprKind::Literal(TypedLiteral::Integer {
                                        value: "1".to_string(),
                                        type_: Some(TypedType::int64()),
                                    }),
                                }),
                            }),
                            TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "b".to_string(),
                                    type_: Some(TypedType::Value(TypedValueType::Reference(
                                        Box::new(TypedType::int64()),
                                    ))),
                                    value: TypedExprKind::UnaryOp(TypedUnaryOp::Prefix(
                                        TypedPrefixUnaryOp {
                                            target: Box::new(TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::new()),
                                                name: "a".to_string(),
                                                type_: Some(TypedType::int64()),
                                                type_arguments: None,
                                            })),
                                            operator: TypedPrefixUnaryOperator::Reference,
                                            type_: Some(TypedType::Value(
                                                TypedValueType::Reference(Box::new(
                                                    TypedType::int64(),
                                                )),
                                            )),
                                        },
                                    )),
                                }),
                            }),
                            TypedStmt::Decl(TypedDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "c".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExprKind::UnaryOp(TypedUnaryOp::Prefix(
                                        TypedPrefixUnaryOp {
                                            target: Box::new(TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::new()),
                                                name: "b".to_string(),
                                                type_: Some(TypedType::Value(
                                                    TypedValueType::Reference(Box::new(
                                                        TypedType::int64(),
                                                    )),
                                                )),
                                                type_arguments: None,
                                            })),
                                            operator: TypedPrefixUnaryOperator::Dereference,
                                            type_: Some(TypedType::int64()),
                                        },
                                    )),
                                }),
                            }),
                        ],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            }],
        },
    )
}

#[test]
fn test_toplevel_var() {
    let source = r"
    val i: Int32 = 1
    ";
    check(
        source,
        TypedFile {
            name: "tests".to_string(),
            uses: vec![],
            body: vec![TypedDecl {
                annotations: Default::default(),
                package: Package::from(&vec!["tests"]),
                modifiers: vec![],
                kind: TypedDeclKind::Var(TypedVar {
                    is_mut: false,
                    name: "i".to_string(),
                    type_: Some(TypedType::int32()),
                    value: TypedExprKind::Literal(TypedLiteral::Integer {
                        value: "1".to_string(),
                        type_: Some(TypedType::int32()),
                    }),
                }),
            }],
        },
    )
}

#[test]
fn test_function_overload_by_arguments() {
    let source = r"
        fun sample(_ d: Double) { }
        fun sample(_ i: Int64) { }
        fun call() {
            sample(0.5)
            sample(1)
        }
        ";

    check(
        source,
        TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "sample".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "d".to_string(),
                            type_: TypedType::double(),
                        }],
                        body: Option::from(TypedFunBody::Block(TypedBlock { body: vec![] })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "sample".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "i".to_string(),
                            type_: TypedType::int64(),
                        }],
                        body: Option::from(TypedFunBody::Block(TypedBlock { body: vec![] })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
                TypedDecl {
                    annotations: Default::default(),
                    package: Package::from(&vec!["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "call".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Option::from(TypedFunBody::Block(TypedBlock {
                            body: vec![
                                TypedStmt::Expr(TypedExprKind::Call(TypedCall {
                                    target: Box::new(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::from(&vec![
                                            "test",
                                        ])),
                                        name: "sample".to_string(),
                                        type_: Some(TypedType::Function(Box::new(
                                            TypedFunctionType {
                                                arguments: vec![TypedArgType {
                                                    label: "_".to_string(),
                                                    typ: TypedType::double(),
                                                }],
                                                return_type: TypedType::unit(),
                                            },
                                        ))),
                                        type_arguments: None,
                                    })),
                                    args: vec![TypedCallArg {
                                        label: None,
                                        arg: Box::new(TypedExprKind::Literal(
                                            TypedLiteral::FloatingPoint {
                                                value: "0.5".to_string(),
                                                type_: Some(TypedType::double()),
                                            },
                                        )),
                                        is_vararg: false,
                                    }],
                                    type_: Some(TypedType::unit()),
                                })),
                                TypedStmt::Expr(TypedExprKind::Call(TypedCall {
                                    target: Box::new(TypedExprKind::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::from(&vec![
                                            "test",
                                        ])),
                                        name: "sample".to_string(),
                                        type_: Some(TypedType::Function(Box::new(
                                            TypedFunctionType {
                                                arguments: vec![TypedArgType {
                                                    label: "_".to_string(),
                                                    typ: TypedType::int64(),
                                                }],
                                                return_type: TypedType::unit(),
                                            },
                                        ))),
                                        type_arguments: None,
                                    })),
                                    args: vec![TypedCallArg {
                                        label: None,
                                        arg: Box::new(TypedExprKind::Literal(
                                            TypedLiteral::Integer {
                                                value: "1".to_string(),
                                                type_: Some(TypedType::int64()),
                                            },
                                        )),
                                        is_vararg: false,
                                    }],
                                    type_: Some(TypedType::unit()),
                                })),
                            ],
                        })),
                        return_type: Some(TypedType::unit()),
                    }),
                },
            ],
        },
    );
}
