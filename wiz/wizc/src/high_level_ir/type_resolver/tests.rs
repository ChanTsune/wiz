use crate::high_level_ir::AstLowering;
use crate::ModuleId;
use wiz_arena::Arena;
use wiz_hir::typed_decl::{
    TypedArgDef, TypedDeclKind, TypedFun, TypedFunBody, TypedStoredProperty, TypedStruct,
    TypedTopLevelDecl, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedExprKind, TypedIf,
    TypedInstanceMember, TypedLiteralKind, TypedName, TypedPrefixUnaryOp, TypedPrefixUnaryOperator,
    TypedReturn, TypedSubscript, TypedUnaryOp,
};
use wiz_hir::typed_file::TypedSpellBook;
use wiz_hir::typed_stmt::{TypedBlock, TypedStmt};
use wiz_hir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedValueType,
};
use wiz_session::{ParseSession, Session};
use wiz_syntax_parser::parser::wiz::parse_from_string;

fn check(source: &str, typed_file: TypedSpellBook) {
    let session = ParseSession::default();
    let ast = parse_from_string::<&str>(&session, None, source, Some(&typed_file.name)).unwrap();

    let mut session = Session::default();

    let mut arena = Arena::default();

    let mut ast2hlir = AstLowering::new(&mut session, &mut arena);

    let f = ast2hlir.lowing(ast, ModuleId::DUMMY).unwrap();

    assert_eq!(f, typed_file);
}

#[test]
fn test_empty() {
    let source = "";

    check(
        source,
        TypedSpellBook {
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::unsafe_pointer(TypedType::uint8()),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![TypedFun::size(TypedType::Value(
                            TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            }),
                        ))],
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::unsafe_pointer(TypedType::uint8())),
                                    value: TypedExpr::new(
                                        TypedExprKind::Member(TypedInstanceMember {
                                            target: Box::new(TypedExpr::new(
                                                TypedExprKind::Name(TypedName {
                                                    package: TypedPackage::Resolved(Package::new()),
                                                    name: "a".to_string(),
                                                    type_arguments: None,
                                                }),
                                                Some(TypedType::Value(TypedValueType::Value(
                                                    TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    },
                                                ))),
                                            )),
                                            name: "a".to_string(),
                                            is_safe: false,
                                        }),
                                        Some(TypedType::unsafe_pointer(TypedType::uint8())),
                                    ),
                                }),
                            })],
                        })),
                        return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![TypedFun::size(TypedType::Value(
                            TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            }),
                        ))],
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExpr::new(
                                        TypedExprKind::Member(TypedInstanceMember {
                                            target: Box::new(TypedExpr::new(
                                                TypedExprKind::Name(TypedName {
                                                    package: TypedPackage::Resolved(Package::new()),
                                                    name: "a".to_string(),
                                                    type_arguments: None,
                                                }),
                                                Some(TypedType::Value(TypedValueType::Value(
                                                    TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    },
                                                ))),
                                            )),
                                            name: "a".to_string(),
                                            is_safe: false,
                                        }),
                                        Some(TypedType::int64()),
                                    ),
                                }),
                            })],
                        })),
                        return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![TypedFun::size(TypedType::Value(
                            TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            }),
                        ))],
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::Value(TypedValueType::Value(
                                        TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(&[
                                                "test",
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None,
                                        },
                                    ))),
                                    value: TypedExpr::new(
                                        TypedExprKind::Call(TypedCall {
                                            target: Box::new(TypedExpr::new(
                                                TypedExprKind::Name(TypedName {
                                                    package: TypedPackage::Resolved(Package::from(
                                                        &["test"],
                                                    )),
                                                    name: "A".to_string(),
                                                    type_arguments: None,
                                                }),
                                                Some(TypedType::Type(Box::new(TypedType::Value(
                                                    TypedValueType::Value(TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    }),
                                                )))),
                                            )),
                                            args: vec![TypedCallArg {
                                                label: Some(String::from("a")),
                                                arg: Box::new(TypedExpr::new(
                                                    TypedExprKind::Literal(
                                                        TypedLiteralKind::Integer("1".to_string()),
                                                    ),
                                                    Some(TypedType::int64()),
                                                )),
                                                is_vararg: false,
                                            }],
                                        }),
                                        Some(TypedType::Value(TypedValueType::Value(
                                            TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &["test"],
                                                )),
                                                name: "A".to_string(),
                                                type_args: None,
                                            },
                                        ))),
                                    ),
                                }),
                            })],
                        })),
                        return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Struct(TypedStruct {
                    name: "A".to_string(),
                    type_params: None,
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64(),
                    }],
                    computed_properties: vec![],
                    member_functions: vec![
                        TypedFun {
                            name: "getA".to_string(),
                            arg_defs: vec![TypedArgDef {
                                label: "_".to_string(),
                                name: "self".to_string(),
                                type_: TypedType::Value(
                                    // TODO: Reference
                                    TypedValueType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(&[
                                            "test",
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None,
                                    }),
                                ),
                            }],
                            type_params: None,
                            body: Some(TypedFunBody::Block(TypedBlock {
                                body: vec![TypedStmt::Expr(TypedExpr::new(
                                    TypedExprKind::Return(TypedReturn {
                                        value: Some(Box::new(TypedExpr::new(
                                            TypedExprKind::Member(TypedInstanceMember {
                                                target: Box::new(TypedExpr::new(
                                                    TypedExprKind::Name(TypedName {
                                                        package: TypedPackage::Resolved(
                                                            Package::new(),
                                                        ),
                                                        name: "self".to_string(),
                                                        type_arguments: None,
                                                    }),
                                                    Some(TypedType::Value(TypedValueType::Value(
                                                        TypedNamedValueType {
                                                            package: TypedPackage::Resolved(
                                                                Package::from(&["test"]),
                                                            ),
                                                            name: "A".to_string(),
                                                            type_args: None,
                                                        },
                                                    ))),
                                                )),
                                                name: "a".to_string(),
                                                is_safe: false,
                                            }),
                                            Some(TypedType::int64()),
                                        ))),
                                    }),
                                    Some(TypedType::noting()),
                                ))],
                            })),
                            return_type: TypedType::int64(),
                            type_constraints: None,
                        },
                        TypedFun::size(TypedType::Value(TypedValueType::Value(
                            TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            },
                        ))),
                    ],
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Struct(TypedStruct {
                        name: "A".to_string(),
                        type_params: None,
                        stored_properties: vec![TypedStoredProperty {
                            name: "a".to_string(),
                            type_: TypedType::int64(),
                        }],
                        computed_properties: vec![],
                        member_functions: vec![
                            TypedFun {
                                name: "getA".to_string(),
                                arg_defs: vec![TypedArgDef {
                                    label: "_".to_string(),
                                    name: "self".to_string(),
                                    type_: TypedType::Value(
                                        // TODO: Reference
                                        TypedValueType::Value(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(&[
                                                "test",
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None,
                                        }),
                                    ),
                                }],
                                type_params: None,
                                body: Some(TypedFunBody::Block(TypedBlock {
                                    body: vec![TypedStmt::Expr(TypedExpr::new(
                                        TypedExprKind::Return(TypedReturn {
                                            value: Some(Box::new(TypedExpr::new(
                                                TypedExprKind::Member(TypedInstanceMember {
                                                    target: Box::new(TypedExpr::new(
                                                        TypedExprKind::Name(TypedName {
                                                            package: TypedPackage::Resolved(
                                                                Package::new(),
                                                            ),
                                                            name: "self".to_string(),
                                                            type_arguments: None,
                                                        }),
                                                        Some(TypedType::Value(
                                                            TypedValueType::Value(
                                                                TypedNamedValueType {
                                                                    package: TypedPackage::Resolved(
                                                                        Package::from(&[
                                                                            "test",
                                                                        ]),
                                                                    ),
                                                                    name: "A".to_string(),
                                                                    type_args: None,
                                                                },
                                                            ),
                                                        )),
                                                    )),
                                                    name: "a".to_string(),
                                                    is_safe: false,
                                                }),
                                                Some(TypedType::int64()),
                                            ))),
                                        }),
                                        Some(TypedType::noting()),
                                    ))],
                                })),
                                return_type: TypedType::int64(),
                                type_constraints: None,
                            },
                            TypedFun::size(TypedType::Value(TypedValueType::Value(
                                TypedNamedValueType {
                                    package: TypedPackage::Resolved(Package::from(&["test"])),
                                    name: "A".to_string(),
                                    type_args: None,
                                },
                            ))),
                        ],
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![TypedArgDef {
                            label: "_".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(&["test"])),
                                name: "A".to_string(),
                                type_args: None,
                            })),
                        }],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExpr::new(
                                TypedExprKind::Call(TypedCall {
                                    target: Box::new(TypedExpr::new(
                                        TypedExprKind::Member(TypedInstanceMember {
                                            target: Box::new(TypedExpr::new(
                                                TypedExprKind::Name(TypedName {
                                                    package: TypedPackage::Resolved(Package::new()),
                                                    name: "a".to_string(),
                                                    type_arguments: None,
                                                }),
                                                Some(TypedType::Value(TypedValueType::Value(
                                                    TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    },
                                                ))),
                                            )),
                                            name: "getA".to_string(),
                                            is_safe: false,
                                        }),
                                        Some(TypedType::Function(Box::new(TypedFunctionType {
                                            arguments: vec![TypedArgType {
                                                label: "_".to_string(),
                                                typ: TypedType::Value(TypedValueType::Value(
                                                    TypedNamedValueType {
                                                        package: TypedPackage::Resolved(
                                                            Package::from(&["test"]),
                                                        ),
                                                        name: "A".to_string(),
                                                        type_args: None,
                                                    },
                                                )),
                                            }],
                                            return_type: TypedType::int64(),
                                        }))),
                                    )),
                                    args: vec![],
                                }),
                                Some(TypedType::int64()),
                            ))],
                        })),
                        return_type: TypedType::unit(),
                    }),
                },
            ],
        },
    );
}

#[test]
fn test_expr_function_with_no_arg() {
    let source = r"
        fun function(): Int64 = 1
        ";

    check(
        source,
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "function".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Expr(TypedExpr::new(
                        TypedExprKind::Literal(TypedLiteralKind::Integer("1".to_string())),
                        Some(TypedType::int64()),
                    ))),
                    return_type: TypedType::int64(),
                }),
            }],
        },
    );
}

#[test]
fn test_expr_function_with_arg() {
    let source = r"
        fun function(_ i:Int32): Int32 = i
        ";

    check(
        source,
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
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
                    body: Some(TypedFunBody::Expr(TypedExpr::new(
                        TypedExprKind::Name(TypedName {
                            package: TypedPackage::Resolved(Package::new()),
                            name: "i".to_string(),
                            type_arguments: None,
                        }),
                        Some(TypedType::int32()),
                    ))),
                    return_type: TypedType::int32(),
                }),
            }],
        },
    );
}

#[test]
fn test_function_call() {
    let source = r"
        fun target_function(): Int64 = 1
        fun main() {
            target_function()
        }
        ";

    check(
        source,
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "target_function".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Expr(TypedExpr::new(
                            TypedExprKind::Literal(TypedLiteralKind::Integer("1".to_string())),
                            Some(TypedType::int64()),
                        ))),
                        return_type: TypedType::int64(),
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "main".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExpr::new(
                                TypedExprKind::Call(TypedCall {
                                    target: Box::new(TypedExpr::new(
                                        TypedExprKind::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::from(&[
                                                "test",
                                            ])),
                                            name: "target_function".to_string(),
                                            type_arguments: None,
                                        }),
                                        Some(TypedType::Function(Box::new(TypedFunctionType {
                                            arguments: vec![],
                                            return_type: TypedType::int64(),
                                        }))),
                                    )),
                                    args: vec![],
                                }),
                                Some(TypedType::int64()),
                            ))],
                        })),
                        return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::new(
                            TypedExprKind::Return(TypedReturn {
                                value: Some(Box::new(TypedExpr::new(
                                    TypedExprKind::Literal(TypedLiteralKind::Integer(
                                        "1".to_string(),
                                    )),
                                    Some(TypedType::int64()),
                                ))),
                            }),
                            Some(TypedType::noting()),
                        ))],
                    })),
                    return_type: TypedType::int64(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::new(
                            TypedExprKind::Return(TypedReturn {
                                value: Some(Box::new(TypedExpr::new(
                                    TypedExprKind::Literal(TypedLiteralKind::FloatingPoint(
                                        "0.5".to_string(),
                                    )),
                                    Some(TypedType::double()),
                                ))),
                            }),
                            Some(TypedType::noting()),
                        ))],
                    })),
                    return_type: TypedType::double(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "sample".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Option::from(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::new(
                            TypedExprKind::BinOp(TypedBinOp {
                                left: Box::new(TypedExpr::new(
                                    TypedExprKind::Literal(TypedLiteralKind::Integer(
                                        "1".to_string(),
                                    )),
                                    Some(TypedType::int64()),
                                )),
                                operator: TypedBinaryOperator::Add,
                                right: Box::new(TypedExpr::new(
                                    TypedExprKind::Literal(TypedLiteralKind::Integer(
                                        "2".to_string(),
                                    )),
                                    Some(TypedType::int64()),
                                )),
                            }),
                            Some(TypedType::int64()),
                        ))],
                    })),
                    return_type: TypedType::unit(),
                }),
            }],
        },
    );
}

#[test]
fn test_subscript() {
    let source = r"
        fun get_first(_ p: *UInt8): UInt8 = p[0]
        ";

    check(
        source,
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
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
                    body: Option::from(TypedFunBody::Expr(TypedExpr::new(
                        TypedExprKind::Subscript(TypedSubscript {
                            target: Box::new(TypedExpr::new(
                                TypedExprKind::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "p".to_string(),
                                    type_arguments: None,
                                }),
                                Some(TypedType::unsafe_pointer(TypedType::uint8())),
                            )),
                            indexes: vec![TypedExpr::new(
                                TypedExprKind::Literal(TypedLiteralKind::Integer("0".to_string())),
                                Some(TypedType::int64()),
                            )],
                        }),
                        Some(TypedType::uint8()),
                    ))),
                    return_type: TypedType::uint8(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
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
                        body: vec![TypedStmt::Expr(TypedExpr::new(
                            TypedExprKind::Return(TypedReturn {
                                value: Some(Box::new(TypedExpr::new(
                                    TypedExprKind::If(TypedIf {
                                        condition: Box::new(TypedExpr::new(
                                            TypedExprKind::BinOp(TypedBinOp {
                                                left: Box::new(TypedExpr::new(
                                                    TypedExprKind::Name(TypedName {
                                                        package: TypedPackage::Resolved(
                                                            Package::new(),
                                                        ),
                                                        name: "i".to_string(),
                                                        type_arguments: None,
                                                    }),
                                                    Some(TypedType::int64()),
                                                )),
                                                operator: TypedBinaryOperator::LessThanEqual,
                                                right: Box::new(TypedExpr::new(
                                                    TypedExprKind::Literal(
                                                        TypedLiteralKind::Integer("0".to_string()),
                                                    ),
                                                    Some(TypedType::int64()),
                                                )),
                                            }),
                                            Some(TypedType::bool()),
                                        )),
                                        body: TypedBlock {
                                            body: vec![TypedStmt::Expr(TypedExpr::new(
                                                TypedExprKind::Literal(TypedLiteralKind::Integer(
                                                    "0".to_string(),
                                                )),
                                                Some(TypedType::int64()),
                                            ))],
                                        },
                                        else_body: Some(TypedBlock {
                                            body: vec![TypedStmt::Expr(TypedExpr::new(
                                                TypedExprKind::Name(TypedName {
                                                    package: TypedPackage::Resolved(Package::new()),
                                                    name: "i".to_string(),
                                                    type_arguments: None,
                                                }),
                                                Some(TypedType::int64()),
                                            ))],
                                        }),
                                    }),
                                    Some(TypedType::int64()),
                                ))),
                            }),
                            Some(TypedType::noting()),
                        ))],
                    })),
                    return_type: TypedType::int64(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
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
                        body: vec![TypedStmt::Expr(TypedExpr::new(
                            TypedExprKind::If(TypedIf {
                                condition: Box::new(TypedExpr::new(
                                    TypedExprKind::BinOp(TypedBinOp {
                                        left: Box::new(TypedExpr::new(
                                            TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::new()),
                                                name: "i".to_string(),
                                                type_arguments: None,
                                            }),
                                            Some(TypedType::int64()),
                                        )),
                                        operator: TypedBinaryOperator::LessThanEqual,
                                        right: Box::new(TypedExpr::new(
                                            TypedExprKind::Literal(TypedLiteralKind::Integer(
                                                "0".to_string(),
                                            )),
                                            Some(TypedType::int64()),
                                        )),
                                    }),
                                    Some(TypedType::bool()),
                                )),
                                body: TypedBlock {
                                    body: vec![TypedStmt::Decl(TypedTopLevelDecl {
                                        annotations: Default::default(),
                                        package: Package::new(),
                                        modifiers: vec![],
                                        kind: TypedDeclKind::Var(TypedVar {
                                            is_mut: false,
                                            name: "p".to_string(),
                                            type_: Some(TypedType::int64()),
                                            value: TypedExpr::new(
                                                TypedExprKind::Literal(TypedLiteralKind::Integer(
                                                    "1".to_string(),
                                                )),
                                                Some(TypedType::int64()),
                                            ),
                                        }),
                                    })],
                                },
                                else_body: None,
                            }),
                            Some(TypedType::noting()),
                        ))],
                    })),
                    return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["test"]),
                modifiers: vec![],
                kind: TypedDeclKind::Fun(TypedFun {
                    name: "test_reference_dereference".to_string(),
                    type_params: None,
                    type_constraints: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![
                            TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "a".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExpr::new(
                                        TypedExprKind::Literal(TypedLiteralKind::Integer(
                                            "1".to_string(),
                                        )),
                                        Some(TypedType::int64()),
                                    ),
                                }),
                            }),
                            TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "b".to_string(),
                                    type_: Some(TypedType::Value(TypedValueType::Reference(
                                        Box::new(TypedType::int64()),
                                    ))),
                                    value: TypedExpr::new(
                                        TypedExprKind::UnaryOp(TypedUnaryOp::Prefix(
                                            TypedPrefixUnaryOp {
                                                target: Box::new(TypedExpr::new(
                                                    TypedExprKind::Name(TypedName {
                                                        package: TypedPackage::Resolved(
                                                            Package::new(),
                                                        ),
                                                        name: "a".to_string(),
                                                        type_arguments: None,
                                                    }),
                                                    Some(TypedType::int64()),
                                                )),
                                                operator: TypedPrefixUnaryOperator::Reference,
                                            },
                                        )),
                                        Some(TypedType::Value(TypedValueType::Reference(
                                            Box::new(TypedType::int64()),
                                        ))),
                                    ),
                                }),
                            }),
                            TypedStmt::Decl(TypedTopLevelDecl {
                                annotations: Default::default(),
                                package: Package::new(),
                                modifiers: vec![],
                                kind: TypedDeclKind::Var(TypedVar {
                                    is_mut: false,
                                    name: "c".to_string(),
                                    type_: Some(TypedType::int64()),
                                    value: TypedExpr::new(
                                        TypedExprKind::UnaryOp(TypedUnaryOp::Prefix(
                                            TypedPrefixUnaryOp {
                                                target: Box::new(TypedExpr::new(
                                                    TypedExprKind::Name(TypedName {
                                                        package: TypedPackage::Resolved(
                                                            Package::new(),
                                                        ),
                                                        name: "b".to_string(),
                                                        type_arguments: None,
                                                    }),
                                                    Some(TypedType::Value(
                                                        TypedValueType::Reference(Box::new(
                                                            TypedType::int64(),
                                                        )),
                                                    )),
                                                )),
                                                operator: TypedPrefixUnaryOperator::Dereference,
                                            },
                                        )),
                                        Some(TypedType::int64()),
                                    ),
                                }),
                            }),
                        ],
                    })),
                    return_type: TypedType::unit(),
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
        TypedSpellBook {
            name: "tests".to_string(),
            uses: vec![],
            body: vec![TypedTopLevelDecl {
                annotations: Default::default(),
                package: Package::from(&["tests"]),
                modifiers: vec![],
                kind: TypedDeclKind::Var(TypedVar {
                    is_mut: false,
                    name: "i".to_string(),
                    type_: Some(TypedType::int32()),
                    value: TypedExpr::new(
                        TypedExprKind::Literal(TypedLiteralKind::Integer("1".to_string())),
                        Some(TypedType::int32()),
                    ),
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
        TypedSpellBook {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
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
                        return_type: TypedType::unit(),
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
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
                        return_type: TypedType::unit(),
                    }),
                },
                TypedTopLevelDecl {
                    annotations: Default::default(),
                    package: Package::from(&["test"]),
                    modifiers: vec![],
                    kind: TypedDeclKind::Fun(TypedFun {
                        name: "call".to_string(),
                        type_params: None,
                        type_constraints: None,
                        arg_defs: vec![],
                        body: Option::from(TypedFunBody::Block(TypedBlock {
                            body: vec![
                                TypedStmt::Expr(TypedExpr::new(
                                    TypedExprKind::Call(TypedCall {
                                        target: Box::new(TypedExpr::new(
                                            TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &["test"],
                                                )),
                                                name: "sample".to_string(),
                                                type_arguments: None,
                                            }),
                                            Some(TypedType::Function(Box::new(
                                                TypedFunctionType {
                                                    arguments: vec![TypedArgType {
                                                        label: "_".to_string(),
                                                        typ: TypedType::double(),
                                                    }],
                                                    return_type: TypedType::unit(),
                                                },
                                            ))),
                                        )),
                                        args: vec![TypedCallArg {
                                            label: None,
                                            arg: Box::new(TypedExpr::new(
                                                TypedExprKind::Literal(
                                                    TypedLiteralKind::FloatingPoint(
                                                        "0.5".to_string(),
                                                    ),
                                                ),
                                                Some(TypedType::double()),
                                            )),
                                            is_vararg: false,
                                        }],
                                    }),
                                    Some(TypedType::unit()),
                                )),
                                TypedStmt::Expr(TypedExpr::new(
                                    TypedExprKind::Call(TypedCall {
                                        target: Box::new(TypedExpr::new(
                                            TypedExprKind::Name(TypedName {
                                                package: TypedPackage::Resolved(Package::from(
                                                    &["test"],
                                                )),
                                                name: "sample".to_string(),
                                                type_arguments: None,
                                            }),
                                            Some(TypedType::Function(Box::new(
                                                TypedFunctionType {
                                                    arguments: vec![TypedArgType {
                                                        label: "_".to_string(),
                                                        typ: TypedType::int64(),
                                                    }],
                                                    return_type: TypedType::unit(),
                                                },
                                            ))),
                                        )),
                                        args: vec![TypedCallArg {
                                            label: None,
                                            arg: Box::new(TypedExpr::new(
                                                TypedExprKind::Literal(TypedLiteralKind::Integer(
                                                    "1".to_string(),
                                                )),
                                                Some(TypedType::int64()),
                                            )),
                                            is_vararg: false,
                                        }],
                                    }),
                                    Some(TypedType::unit()),
                                )),
                            ],
                        })),
                        return_type: TypedType::unit(),
                    }),
                },
            ],
        },
    );
}
