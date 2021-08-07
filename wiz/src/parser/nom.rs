pub mod character;
pub mod declaration;
pub mod expression;
pub mod keywords;
pub mod lexical_structure;
pub mod operators;
pub mod type_;

use crate::ast::expr::{Expr, NameExprSyntax};
use crate::ast::file::FileSyntax;
use crate::ast::stmt::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt,
};
use crate::parser::nom::declaration::{block, decl};
use crate::parser::nom::expression::{expr, postfix_expr, prefix_expr};
use crate::parser::nom::keywords::while_keyword;
use crate::parser::nom::lexical_structure::{identifier, whitespace0, whitespace1};
use crate::parser::nom::operators::assignment_operator;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{error, IResult};

pub fn decl_stmt(s: &str) -> IResult<&str, Stmt> {
    map(decl, |d| Stmt::Decl { decl: d })(s)
}

pub fn expr_stmt(s: &str) -> IResult<&str, Stmt> {
    map(expr, |e| Stmt::Expr { expr: e })(s)
}

/*
<assignment_stmt> ::= ((<directly_assignable_expr> '=') | (<assignable_expr> <assignment_and_operator>)) <expr>
*/
pub fn assignment_stmt(s: &str) -> IResult<&str, Stmt> {
    map(
        tuple((
            alt((
                tuple((directly_assignable_expr, whitespace0, assignment_operator)),
                tuple((assignable_expr, whitespace0, assignment_and_operator)),
            )),
            whitespace0,
            expr,
        )),
        |((target, _, op), _, value)| {
            if op == "=" {
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target,
                    value,
                }))
            } else {
                Stmt::Assignment(AssignmentStmt::AssignmentAndOperator(
                    AssignmentAndOperatorSyntax {
                        target,
                        operator: op.to_string(),
                        value,
                    },
                ))
            }
        },
    )(s)
}
/*
<directly_assignable_expr> ::= <postfix_expr> <assignable_suffix>
                             | <identifier>
                             | <parenthesized_directly_assignable_expr>
*/
pub fn directly_assignable_expr(s: &str) -> IResult<&str, Expr> {
    alt((
        _directly_assignable_postfix_expr,
        map(identifier, |name| Expr::Name(NameExprSyntax { name })),
        map(parenthesized_directly_assignable_expr, |e| e),
    ))(s)
}

/*
<assignable_suffix> ::= <type_arguments>
  | <indexing_suffix>
  | <navigation_suffix>
*/
fn _directly_assignable_postfix_expr(s: &str) -> IResult<&str, Expr> {
    let (e, expr) = postfix_expr(s)?;
    match expr {
        Expr::Member { .. } => IResult::Ok((e, expr)),
        Expr::Subscript { .. } => IResult::Ok((e, expr)),
        _ => IResult::Err(Error(error::Error::new(e, error::ErrorKind::Alt))),
    }
}

/*
<assignable_expr> ::= <prefix_expr>
  | <parenthesized_assignable_expression>
*/
pub fn assignable_expr(s: &str) -> IResult<&str, Expr> {
    alt((prefix_expr, parenthesized_assignable_expression))(s)
}
/*
<parenthesized_assignable_expression> ::= "(" <assignable_expr> ")"
*/
pub fn parenthesized_assignable_expression(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((char('('), assignable_expr, char(')'))),
        |(_, e, _)| e,
    )(s)
}

/*
<parenthesized_directly_assignable_expr> ::= '(' <directly_assignable_expr> ')'
*/
pub fn parenthesized_directly_assignable_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((char('('), directly_assignable_expr, char(')'))),
        |(_, e, _)| e,
    )(s)
}
pub fn assignment_and_operator(s: &str) -> IResult<&str, &str> {
    alt((tag("+="), tag("-="), tag("*="), tag("/="), tag("%=")))(s)
}

pub fn loop_stmt(s: &str) -> IResult<&str, Stmt> {
    map(alt((while_stmt, while_stmt)), |l| Stmt::Loop(l))(s)
}

pub fn while_stmt(s: &str) -> IResult<&str, LoopStmt> {
    map(
        tuple((while_keyword, whitespace1, expr, whitespace1, block)),
        |(_, _, e, _, b)| LoopStmt::While {
            condition: e,
            block: b,
        },
    )(s)
}

pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    map(
        tuple((
            whitespace0,
            alt((decl_stmt, assignment_stmt, loop_stmt, expr_stmt)),
        )),
        |(ws, stm)| stm,
    )(s)
}

pub fn stmts(s: &str) -> IResult<&str, Vec<Stmt>> {
    many0(stmt)(s)
}

pub fn file(s: &str) -> IResult<&str, FileSyntax> {
    map(many0(tuple((whitespace0, decl, whitespace0))), |decls| {
        FileSyntax {
            body: decls.into_iter().map(|(_, f, _)| f).collect(),
        }
    })(s)
}

mod tests {
    use crate::ast::block::Block;
    use crate::ast::expr::{Expr, NameExprSyntax};
    use crate::ast::literal::Literal;
    use crate::ast::stmt::{AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt};
    use crate::parser::nom::{
        assignable_expr, assignment_stmt, directly_assignable_expr, while_stmt,
    };

    #[test]
    fn test_while_stmt_with_bracket() {
        assert_eq!(
            while_stmt(
                r"while (a < b) {
            a = a + 1
        }"
            ),
            Ok((
                "",
                LoopStmt::While {
                    condition: Expr::BinOp {
                        left: Box::new(Expr::Name(NameExprSyntax {
                            name: "a".to_string()
                        })),
                        kind: "<".to_string(),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name: "b".to_string()
                        }))
                    },
                    block: Block {
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name: "a".to_string()
                                }),
                                value: Expr::BinOp {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name: "a".to_string()
                                    })),
                                    kind: "+".to_string(),
                                    right: Box::new(Expr::Literal(Literal::Integer {
                                        value: "1".to_string()
                                    }))
                                }
                            }
                        ))]
                    }
                }
            ))
        )
    }

    #[test]
    fn test_while_stmt() {
        assert_eq!(
            while_stmt(
                r"while a.c < b {
            a = a + 1
        }"
            ),
            Ok((
                "",
                LoopStmt::While {
                    condition: Expr::BinOp {
                        left: Box::new(Expr::Member {
                            target: Box::new(Expr::Name(NameExprSyntax {
                                name: "a".to_string()
                            })),
                            name: "c".to_string(),
                            is_safe: false
                        }),
                        kind: "<".to_string(),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name: "b".to_string()
                        }))
                    },
                    block: Block {
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name: "a".to_string()
                                }),
                                value: Expr::BinOp {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name: "a".to_string()
                                    })),
                                    kind: "+".to_string(),
                                    right: Box::new(Expr::Literal(Literal::Integer {
                                        value: "1".to_string()
                                    }))
                                }
                            }
                        ))]
                    }
                }
            ))
        )
    }

    #[test]
    fn test_directly_assignable_expr() {
        assert_eq!(
            directly_assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    })),
                    name: "b".to_string(),
                    is_safe: false
                }
            ))
        )
    }

    #[test]
    fn test_assignable_expr() {
        assert_eq!(
            assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    })),
                    name: "b".to_string(),
                    is_safe: false
                }
            ))
        )
    }

    #[test]
    fn test_assignment() {
        assert_eq!(
            assignment_stmt("a = b"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    }),
                    value: Expr::Name(NameExprSyntax {
                        name: "b".to_string()
                    })
                }))
            ))
        )
    }
    #[test]
    fn test_assignment_struct_field() {
        assert_eq!(
            assignment_stmt("a = b.c"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    }),
                    value: Expr::Member {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name: "b".to_string()
                        })),
                        name: "c".to_string(),
                        is_safe: false
                    }
                }))
            ))
        )
    }
    #[test]
    fn test_assignment_to_struct_field() {
        assert_eq!(
            assignment_stmt("a.b = c"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Member {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name: "a".to_string()
                        })),
                        name: "b".to_string(),
                        is_safe: false
                    },
                    value: Expr::Name(NameExprSyntax {
                        name: "c".to_string()
                    }),
                }))
            ))
        )
    }
}
