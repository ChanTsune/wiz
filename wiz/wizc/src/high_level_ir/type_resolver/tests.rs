use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedInitializer, TypedMemberFunction,
    TypedStoredProperty, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedIf,
    TypedInstanceMember, TypedLiteral, TypedName, TypedReturn, TypedSubscript,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentStmt, TypedBlock, TypedStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType};
use crate::high_level_ir::Ast2HLIR;
use wiz_syntax::parser::wiz::parse_from_string;

#[test]
fn test_empty() {
    let source = "";

    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![]
        })
    );
}

#[test]
fn test_unsafe_pointer() {
    let source = r"
        struct A {
            val a: UnsafePointer<UInt8>
        }
        fun function(_ a: A): Unit {
            val a = a.a
        }
        ";

    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    name: "A".to_string(),
                    type_params: None,
                    initializers: vec![TypedInitializer {
                        args: vec![TypedArgDef {
                            label: "a".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::unsafe_pointer(TypedType::uint8())
                        }],
                        body: TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::global()),
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    vec!["test"]
                                                )),
                                                name: "A".to_string(),
                                                type_args: None
                                            }))
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::unsafe_pointer(TypedType::uint8()))
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "a".to_string(),
                                        type_: Some(TypedType::unsafe_pointer(TypedType::uint8()))
                                    })
                                }
                            ))]
                        })
                    }],
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::unsafe_pointer(TypedType::uint8())
                    }],
                    computed_properties: vec![],
                    member_functions: vec![],
                }),
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedNamedValueType {
                            package: TypedPackage::Resolved(Package::from(vec!["test"])),
                            name: "A".to_string(),
                            type_args: None
                        })
                    }],
                    body: Option::Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                            annotations: TypedAnnotations::new(),
                            package: TypedPackage::Resolved(Package::new()),
                            is_mut: false,
                            name: "a".to_string(),
                            type_: Some(TypedType::unsafe_pointer(TypedType::uint8())),
                            value: TypedExpr::Member(TypedInstanceMember {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "a".to_string(),
                                    type_: Some(TypedType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(vec![
                                            "test"
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None
                                    }))
                                })),
                                name: "a".to_string(),
                                is_safe: false,
                                type_: Some(TypedType::unsafe_pointer(TypedType::uint8()))
                            })
                        }))]
                    })),
                    return_type: Some(TypedType::unit())
                })
            ],
        })
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
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    name: "A".to_string(),
                    type_params: None,
                    initializers: vec![TypedInitializer {
                        args: vec![TypedArgDef {
                            label: "a".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::int64()
                        }],
                        body: TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::new()),
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    vec!["test"]
                                                )),
                                                name: "A".to_string(),
                                                type_args: None
                                            }))
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::int64())
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "a".to_string(),
                                        type_: Some(TypedType::int64())
                                    })
                                }
                            ))]
                        })
                    }],
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64()
                    }],
                    computed_properties: vec![],
                    member_functions: vec![],
                }),
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedNamedValueType {
                            package: TypedPackage::Resolved(Package::from(vec!["test"])),
                            name: "A".to_string(),
                            type_args: None
                        })
                    }],
                    body: Option::Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                            annotations: TypedAnnotations::new(),
                            package: TypedPackage::Resolved(Package::new()),
                            is_mut: false,
                            name: "a".to_string(),
                            type_: Some(TypedType::int64()),
                            value: TypedExpr::Member(TypedInstanceMember {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "a".to_string(),
                                    type_: Some(TypedType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(vec![
                                            "test"
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None
                                    }))
                                })),
                                name: "a".to_string(),
                                is_safe: false,
                                type_: Some(TypedType::int64())
                            })
                        }))]
                    })),
                    return_type: Some(TypedType::unit())
                })
            ],
        })
    );
}

#[test]
fn test_struct_init() {
    let source = r"
        struct A {
            val a: Int64
        }
        fun function(_ a: A) {
            val a = A.init(a:1)
        }
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    name: "A".to_string(),
                    type_params: None,
                    initializers: vec![TypedInitializer {
                        args: vec![TypedArgDef {
                            label: "a".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::int64()
                        }],
                        body: TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::new()),
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    vec!["test"]
                                                )),
                                                name: "A".to_string(),
                                                type_args: None
                                            }))
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::int64())
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "a".to_string(),
                                        type_: Some(TypedType::int64())
                                    })
                                }
                            ))]
                        })
                    }],
                    stored_properties: vec![TypedStoredProperty {
                        name: "a".to_string(),
                        type_: TypedType::int64(),
                    }],
                    computed_properties: vec![],
                    member_functions: vec![],
                }),
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedNamedValueType {
                            package: TypedPackage::Resolved(Package::from(vec!["test"],)),
                            name: "A".to_string(),
                            type_args: None,
                        }),
                    }],
                    body: Option::Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                            annotations: TypedAnnotations::new(),
                            package: TypedPackage::Resolved(Package::new()),
                            is_mut: false,
                            name: "a".to_string(),
                            type_: Some(TypedType::Value(TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                                name: "A".to_string(),
                                type_args: None
                            })),
                            value: TypedExpr::Call(TypedCall {
                                target: Box::new(TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "A".to_string(),
                                        type_: Some(TypedType::Type(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(vec![
                                                "test"
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None
                                        })),
                                    })),
                                    name: "init".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                        arguments: vec![TypedArgType {
                                            label: "a".to_string(),
                                            typ: TypedType::int64()
                                        }],
                                        return_type: TypedType::Value(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(vec![
                                                "test"
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None
                                        })
                                    }))),
                                })),
                                args: vec![TypedCallArg {
                                    label: Some(String::from("a")),
                                    arg: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                        value: "1".to_string(),
                                        type_: Some(TypedType::int64())
                                    })),
                                    is_vararg: false
                                }],
                                type_: Some(TypedType::Value(TypedNamedValueType {
                                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                                    name: "A".to_string(),
                                    type_args: None
                                }))
                            }),
                        }))],
                    })),
                    return_type: Some(TypedType::unit()),
                }),
            ],
        })
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
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Struct(TypedStruct {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                name: "A".to_string(),
                type_params: None,
                initializers: vec![TypedInitializer {
                    args: vec![TypedArgDef {
                        label: "a".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::int64()
                    }],
                    body: TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                            TypedAssignment {
                                target: TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "self".to_string(),
                                        type_: Some(TypedType::Value(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(vec![
                                                "test"
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None
                                        }))
                                    })),
                                    name: "a".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::int64())
                                }),
                                value: TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "a".to_string(),
                                    type_: Some(TypedType::int64())
                                })
                            }
                        ))]
                    })
                }],
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
                            // TODO: TypedType::Reference
                            TypedNamedValueType {
                                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                                name: "A".to_string(),
                                type_args: None
                            }
                        )
                    }],
                    type_params: None,
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                            value: Some(Box::new(TypedExpr::Member(TypedInstanceMember {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "self".to_string(),
                                    type_: Some(TypedType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(vec![
                                            "test"
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None
                                    }))
                                })),
                                name: "a".to_string(),
                                is_safe: false,
                                type_: Some(TypedType::int64())
                            }))),
                        }))]
                    })),
                    return_type: Some(TypedType::int64())
                }],
            }),],
        })
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
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl::Struct(TypedStruct {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    name: "A".to_string(),
                    type_params: None,
                    initializers: vec![TypedInitializer {
                        args: vec![TypedArgDef {
                            label: "a".to_string(),
                            name: "a".to_string(),
                            type_: TypedType::int64()
                        }],
                        body: TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            package: TypedPackage::Resolved(Package::new()),
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(TypedNamedValueType {
                                                package: TypedPackage::Resolved(Package::from(
                                                    vec!["test"]
                                                )),
                                                name: "A".to_string(),
                                                type_args: None
                                            }))
                                        })),
                                        name: "a".to_string(),
                                        is_safe: false,
                                        type_: Some(TypedType::int64())
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "a".to_string(),
                                        type_: Some(TypedType::int64())
                                    })
                                }
                            ))]
                        })
                    }],
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
                                // TypedType::Reference
                                TypedNamedValueType {
                                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                                    name: "A".to_string(),
                                    type_args: None
                                }
                            )
                        }],
                        type_params: None,
                        body: Some(TypedFunBody::Block(TypedBlock {
                            body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                                value: Some(Box::new(TypedExpr::Member(TypedInstanceMember {
                                    target: Box::new(TypedExpr::Name(TypedName {
                                        package: TypedPackage::Resolved(Package::new()),
                                        name: "self".to_string(),
                                        type_: Some(TypedType::Value(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(vec![
                                                "test"
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None
                                        }))
                                    })),
                                    name: "a".to_string(),
                                    is_safe: false,
                                    type_: Some(TypedType::int64())
                                }))),
                            }))]
                        })),
                        return_type: Some(TypedType::int64())
                    }],
                }),
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "function".to_string(),
                    type_params: None,
                    arg_defs: vec![TypedArgDef {
                        label: "_".to_string(),
                        name: "a".to_string(),
                        type_: TypedType::Value(TypedNamedValueType {
                            package: TypedPackage::Resolved(Package::from(vec!["test"])),
                            name: "A".to_string(),
                            type_args: None
                        })
                    }],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::Call(TypedCall {
                            target: Box::new(TypedExpr::Member(TypedInstanceMember {
                                target: Box::new(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "a".to_string(),
                                    type_: Some(TypedType::Value(TypedNamedValueType {
                                        package: TypedPackage::Resolved(Package::from(vec![
                                            "test"
                                        ])),
                                        name: "A".to_string(),
                                        type_args: None
                                    }))
                                })),
                                name: "getA".to_string(),
                                is_safe: false,
                                type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                    arguments: vec![TypedArgType {
                                        label: "_".to_string(),
                                        typ: TypedType::Value(TypedNamedValueType {
                                            package: TypedPackage::Resolved(Package::from(vec![
                                                "test"
                                            ])),
                                            name: "A".to_string(),
                                            type_args: None
                                        })
                                    }],
                                    return_type: TypedType::int64()
                                })))
                            })),
                            args: vec![],
                            type_: Some(TypedType::int64())
                        }))]
                    })),
                    return_type: Some(TypedType::unit())
                })
            ],
        })
    );
}

#[test]
fn test_expr_function_with_no_arg() {
    let source = r"
        fun function() = 1
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "function".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                    TypedLiteral::Integer {
                        value: "1".to_string(),
                        type_: Some(TypedType::int64())
                    }
                ))),
                return_type: Some(TypedType::int64())
            })],
        })
    );
}

#[test]
fn test_expr_function_with_arg() {
    let source = r"
        fun function(_ i:Int32) = i
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "function".to_string(),
                type_params: None,
                arg_defs: vec![TypedArgDef {
                    label: "_".to_string(),
                    name: "i".to_string(),
                    type_: TypedType::int32()
                }],
                body: Some(TypedFunBody::Expr(TypedExpr::Name(TypedName {
                    package: TypedPackage::Resolved(Package::new()),
                    name: "i".to_string(),
                    type_: Some(TypedType::int32())
                }))),
                return_type: Some(TypedType::int32())
            })],
        })
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
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "target_function".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Expr(TypedExpr::Literal(
                        TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        },
                    ))),
                    return_type: Some(TypedType::int64()),
                }),
                TypedDecl::Fun(TypedFun {
                    annotations: TypedAnnotations::new(),
                    package: TypedPackage::Resolved(Package::from(vec!["test"])),
                    modifiers: vec![],
                    name: "main".to_string(),
                    type_params: None,
                    arg_defs: vec![],
                    body: Some(TypedFunBody::Block(TypedBlock {
                        body: vec![TypedStmt::Expr(TypedExpr::Call(TypedCall {
                            target: Box::new(TypedExpr::Name(TypedName {
                                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                                name: "target_function".to_string(),
                                type_: Some(TypedType::Function(Box::new(TypedFunctionType {
                                    arguments: vec![],
                                    return_type: TypedType::int64()
                                })))
                            })),
                            args: vec![],
                            type_: Some(TypedType::int64())
                        }))]
                    })),
                    return_type: Some(TypedType::unit())
                })
            ],
        })
    );
}

#[test]
fn test_return_integer_literal() {
    let source = r"
        fun sample(): Int64 {
            return 1
        }
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "sample".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                        value: Option::Some(Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64())
                        }))),
                    }))]
                })),
                return_type: Some(TypedType::int64())
            })]
        })
    );
}

#[test]
fn test_return_floating_point_literal() {
    let source = r"
        fun sample(): Double {
            return 0.5
        }
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "sample".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                        value: Option::Some(Box::new(TypedExpr::Literal(
                            TypedLiteral::FloatingPoint {
                                value: "0.5".to_string(),
                                type_: Some(TypedType::double())
                            }
                        ))),
                    }))]
                })),
                return_type: Some(TypedType::double())
            })]
        })
    );
}

#[test]
fn test_binop() {
    let source = r"
        fun sample() {
            1 + 2
        }
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "sample".to_string(),
                type_params: None,
                arg_defs: vec![],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::BinOp(TypedBinOp {
                        left: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "1".to_string(),
                            type_: Some(TypedType::int64()),
                        })),
                        operator: TypedBinaryOperator::Add,
                        right: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                            value: "2".to_string(),
                            type_: Some(TypedType::int64()),
                        })),
                        type_: Some(TypedType::int64()),
                    }))],
                })),
                return_type: Some(TypedType::unit())
            })]
        })
    );
}

#[test]
fn test_subscript() {
    let source = r"
        fun get_first(_ p:UnsafePointer<UInt8>) = p[0]
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "get_first".to_string(),
                type_params: None,
                arg_defs: vec![TypedArgDef {
                    label: "_".to_string(),
                    name: "p".to_string(),
                    type_: TypedType::unsafe_pointer(TypedType::uint8())
                }],
                body: Option::from(TypedFunBody::Expr(TypedExpr::Subscript(TypedSubscript {
                    target: Box::new(TypedExpr::Name(TypedName {
                        package: TypedPackage::Resolved(Package::new()),
                        name: "p".to_string(),
                        type_: Some(TypedType::unsafe_pointer(TypedType::uint8()))
                    })),
                    indexes: vec![TypedExpr::Literal(TypedLiteral::Integer {
                        value: "0".to_string(),
                        type_: Some(TypedType::int64())
                    })],
                    type_: Some(TypedType::uint8())
                }))),
                return_type: Some(TypedType::uint8())
            })]
        })
    );
}

#[test]
fn test_if_else() {
    let source = r"
        fun test_if(i:Int64): Int64 {
            return if i <= 0 { 0 } else { i }
        }
        ";
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "test_if".to_string(),
                type_params: None,
                arg_defs: vec![TypedArgDef {
                    label: "i".to_string(),
                    name: "i".to_string(),
                    type_: TypedType::int64()
                }],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::Return(TypedReturn {
                        value: Some(Box::new(TypedExpr::If(TypedIf {
                            condition: Box::new(TypedExpr::BinOp(TypedBinOp {
                                left: Box::new(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "i".to_string(),
                                    type_: Some(TypedType::int64())
                                })),
                                operator: TypedBinaryOperator::LessThanEqual,
                                right: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                    value: "0".to_string(),
                                    type_: Some(TypedType::int64())
                                })),
                                type_: Some(TypedType::bool())
                            })),
                            body: TypedBlock {
                                body: vec![TypedStmt::Expr(TypedExpr::Literal(
                                    TypedLiteral::Integer {
                                        value: "0".to_string(),
                                        type_: Some(TypedType::int64())
                                    }
                                ))]
                            },
                            type_: Some(TypedType::int64()),
                            else_body: Some(TypedBlock {
                                body: vec![TypedStmt::Expr(TypedExpr::Name(TypedName {
                                    package: TypedPackage::Resolved(Package::new()),
                                    name: "i".to_string(),
                                    type_: Some(TypedType::int64())
                                }))]
                            })
                        }))),
                    }))]
                })),
                return_type: Some(TypedType::int64())
            })]
        })
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
    let ast = parse_from_string(source).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file).unwrap();
    let _ = resolver.preload_file(file.clone()).unwrap();
    let f = resolver.file(file);

    assert_eq!(
        f,
        Result::Ok(TypedFile {
            name: "test".to_string(),
            uses: vec![],
            body: vec![TypedDecl::Fun(TypedFun {
                annotations: TypedAnnotations::new(),
                package: TypedPackage::Resolved(Package::from(vec!["test"])),
                modifiers: vec![],
                name: "test_if".to_string(),
                type_params: None,
                arg_defs: vec![TypedArgDef {
                    label: "i".to_string(),
                    name: "i".to_string(),
                    type_: TypedType::int64()
                }],
                body: Option::from(TypedFunBody::Block(TypedBlock {
                    body: vec![TypedStmt::Expr(TypedExpr::If(TypedIf {
                        condition: Box::new(TypedExpr::BinOp(TypedBinOp {
                            left: Box::new(TypedExpr::Name(TypedName {
                                package: TypedPackage::Resolved(Package::new()),
                                name: "i".to_string(),
                                type_: Some(TypedType::int64())
                            })),
                            operator: TypedBinaryOperator::LessThanEqual,
                            right: Box::new(TypedExpr::Literal(TypedLiteral::Integer {
                                value: "0".to_string(),
                                type_: Some(TypedType::int64())
                            })),
                            type_: Some(TypedType::bool())
                        })),
                        body: TypedBlock {
                            body: vec![TypedStmt::Decl(TypedDecl::Var(TypedVar {
                                annotations: TypedAnnotations::new(),
                                package: TypedPackage::Resolved(Package::new()),
                                is_mut: false,
                                name: "p".to_string(),
                                type_: Some(TypedType::int64()),
                                value: TypedExpr::Literal(TypedLiteral::Integer {
                                    value: "1".to_string(),
                                    type_: Some(TypedType::int64())
                                })
                            }))]
                        },
                        type_: Some(TypedType::noting()),
                        else_body: None
                    }))]
                })),
                return_type: Some(TypedType::unit())
            })]
        })
    );
}
