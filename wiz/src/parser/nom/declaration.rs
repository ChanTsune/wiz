use crate::ast::block::Block;
use crate::ast::decl::{Decl, FunSyntax, VarSyntax};
use crate::ast::expr::Expr;
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::type_name::{TypeName, TypeParam};
use crate::parser::nom::expression::expr;
use crate::parser::nom::keywords::{fun_keyword, val_keyword, var_keyword, where_keyword};
use crate::parser::nom::lexical_structure::{identifier, whitespace0, whitespace1};
use crate::parser::nom::stmts;
use crate::parser::nom::type_::type_;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{IResult, Parser};

pub fn decl(s: &str) -> IResult<&str, Decl> {
    alt((function_decl, var_decl))(s)
}

//region func

pub fn function_decl(s: &str) -> IResult<&str, Decl> {
    map(
        tuple((
            fun_keyword,
            whitespace1,
            identifier,
            // opt(type_parameters),
            function_value_parameters,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            opt(type_constraints),
            whitespace0,
            opt(function_body),
        )),
        |(f, _, name, /* type_params, */ args, _, return_type, _, t_constraints, _, body)| {
            Decl::Fun(FunSyntax {
                modifiers: vec![],
                name: name,
                arg_defs: args,
                return_type: return_type.map(|(_, _, t)| t),
                body: body,
            })
        },
    )(s)
}

pub fn function_value_parameters(s: &str) -> IResult<&str, Vec<ArgDef>> {
    map(
        tuple((
            char('('),
            opt(tuple((
                function_value_parameter,
                many0(map(
                    tuple((char(','), function_value_parameter)),
                    |(_, a)| a,
                )),
                opt(char(',')),
            ))),
            char(')'),
        )),
        |(_, args, _)| match args {
            Some((a, mut ar, _)) => {
                let mut t = vec![a];
                t.append(&mut ar);
                t
            }
            None => Vec::new(),
        },
    )(s)
}

pub fn function_value_parameter(s: &str) -> IResult<&str, ArgDef> {
    map(
        tuple((
            whitespace0,
            function_value_label,
            whitespace1,
            function_value_name,
            whitespace0,
            char(':'),
            whitespace0,
            type_,
        )),
        |(_, label, _, name, _, _, _, typ)| ArgDef {
            label: label,
            name: name,
            type_name: typ,
        },
    )(s)
}

pub fn function_value_label(s: &str) -> IResult<&str, String> {
    identifier(s)
}

pub fn function_value_name(s: &str) -> IResult<&str, String> {
    identifier(s)
}

pub fn type_constraints(s: &str) -> IResult<&str, Vec<TypeParam>> {
    map(
        tuple((
            where_keyword,
            type_constraint,
            opt(tuple((char(','), type_constraint))),
        )),
        |(_, t, ts)| match ts {
            Some((_, ts)) => {
                vec![t, ts]
            }
            None => {
                vec![t]
            }
        },
    )(s)
}

pub fn type_constraint(s: &str) -> IResult<&str, TypeParam> {
    map(tuple((identifier, char(':'), type_)), |(id, _, typ)| {
        TypeParam {
            name: id,
            type_constraints: vec![typ],
        }
    })(s)
}

pub fn function_body(s: &str) -> IResult<&str, FunBody> {
    alt((
        map(block, |b| FunBody::Block { block: b }),
        map(tuple((char('='), whitespace0, expr)), |(_, _, ex)| {
            FunBody::Expr { expr: ex }
        }),
    ))(s)
}

pub fn block(s: &str) -> IResult<&str, Block> {
    map(
        tuple((char('{'), whitespace0, stmts, whitespace0, char('}'))),
        |(_, _, stmts, _, _)| Block { body: stmts },
    )(s)
}

//endregion

//region var

pub fn var_decl(s: &str) -> IResult<&str, Decl> {
    alt((var, val))(s)
}

pub fn var(s: &str) -> IResult<&str, Decl> {
    map(
        tuple((var_keyword, whitespace1, var_body)),
        |(_, _, (name, t, e))| {
            Decl::Var(VarSyntax {
                is_mut: true,
                name: name,
                type_: t,
                value: e,
            })
        },
    )(s)
}

pub fn val(s: &str) -> IResult<&str, Decl> {
    map(
        tuple((val_keyword, whitespace1, var_body)),
        |(_, _, (name, t, e))| {
            Decl::Var(VarSyntax {
                is_mut: false,
                name: name,
                type_: t,
                value: e,
            })
        },
    )(s)
}

pub fn var_body(s: &str) -> IResult<&str, (String, Option<TypeName>, Expr)> {
    map(
        tuple((
            identifier,
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
            whitespace0,
            char('='),
            whitespace0,
            expr,
        )),
        |(name, _, t, _, _, _, e)| (name, t.map(|(_, _, t)| t), e),
    )(s)
}

//endregion

#[cfg(test)]
mod test {
    use crate::ast::block::Block;
    use crate::ast::decl::{Decl, FunSyntax, VarSyntax};
    use crate::ast::expr::Expr;
    use crate::ast::fun::arg_def::ArgDef;
    use crate::ast::fun::body_def::FunBody;
    use crate::ast::literal::Literal;
    use crate::ast::stmt::Stmt;
    use crate::ast::type_name::TypeName;
    use crate::parser::nom::declaration::{block, function_body, function_decl, var_decl};

    #[test]
    fn test_empty_block() {
        assert_eq!(block("{}"), Ok(("", Block { body: vec![] })))
    }

    #[test]
    fn test_block_with_int_literal() {
        assert_eq!(
            block("{1}"),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr {
                        expr: Expr::Literal {
                            literal: Literal::IntegerLiteral {
                                value: "1".to_string()
                            }
                        }
                    }]
                }
            ))
        )
    }

    #[test]
    fn test_block_with_binop_literal() {
        assert_eq!(
            block("{1+1}"),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr {
                        expr: Expr::BinOp {
                            left: Box::new(Expr::Literal {
                                literal: Literal::IntegerLiteral {
                                    value: "1".to_string()
                                }
                            }),
                            kind: "+".to_string(),
                            right: Box::new(Expr::Literal {
                                literal: Literal::IntegerLiteral {
                                    value: "1".to_string()
                                }
                            }),
                        }
                    }]
                }
            ))
        )
    }

    #[test]
    fn test_block() {
        assert_eq!(
            block(
                r"{
    1
}"
            ),
            Ok((
                "",
                Block {
                    body: vec![Stmt::Expr {
                        expr: Expr::Literal {
                            literal: Literal::IntegerLiteral {
                                value: "1".to_string()
                            }
                        }
                    }]
                }
            ))
        )
    }

    #[test]
    fn test_function_body_block_case() {
        assert_eq!(
            function_body("{}"),
            Ok((
                "",
                FunBody::Block {
                    block: Block { body: vec![] }
                }
            ))
        )
    }

    #[test]
    fn test_function_body_expr_case() {
        assert_eq!(
            function_body("= name"),
            Ok((
                "",
                FunBody::Expr {
                    expr: Expr::Name {
                        name: "name".to_string()
                    }
                }
            ))
        )
    }

    #[test]
    fn test_function_decl() {
        assert_eq!(
            function_decl("fun function() {}"),
            Ok((
                "",
                Decl::Fun(FunSyntax {
                    modifiers: vec![],
                    name: "function".to_string(),
                    arg_defs: vec![],
                    return_type: None,
                    body: Some(FunBody::Block {
                        block: Block { body: vec![] }
                    }),
                })
            ))
        )
    }

    #[test]
    fn test_function_no_body() {
        assert_eq!(
            function_decl("fun puts(_ item: String): Unit"),
            Ok((
                "",
                Decl::Fun(FunSyntax {
                    modifiers: vec![],
                    name: "puts".to_string(),
                    arg_defs: vec![ArgDef {
                        label: "_".to_string(),
                        name: "item".to_string(),
                        type_name: TypeName {
                            name: "String".to_string(),
                            type_params: vec![]
                        }
                    }],
                    return_type: Some(TypeName {
                        name: "Unit".to_string(),
                        type_params: vec![]
                    }),
                    body: None,
                })
            ))
        )
    }

    #[test]
    fn test_var_decl() {
        assert_eq!(
            var_decl("val a: Int = 1"),
            Ok((
                "",
                Decl::Var(VarSyntax {
                    is_mut: false,
                    name: "a".to_string(),
                    type_: Some(TypeName {
                        name: "Int".to_string(),
                        type_params: vec![]
                    }),
                    value: Expr::Literal {
                        literal: Literal::IntegerLiteral {
                            value: "1".to_string()
                        }
                    }
                })
            ))
        )
    }
    #[test]
    fn test_var_decl_without_type() {
        assert_eq!(
            var_decl("val a = 1"),
            Ok((
                "",
                Decl::Var(VarSyntax {
                    is_mut: false,
                    name: "a".to_string(),
                    type_: None,
                    value: Expr::Literal {
                        literal: Literal::IntegerLiteral {
                            value: "1".to_string()
                        }
                    }
                })
            ))
        )
    }
}
