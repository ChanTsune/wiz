pub mod character;
pub mod combinator;
pub mod declaration;
pub mod expression;
pub mod keywords;
pub mod lexical_structure;
pub mod type_;

use crate::ast::decl::Decl;
use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::literal::Literal;
use crate::ast::stmt::{AssignmentStmt, LoopStmt, Stmt};
use crate::parser::nom::declaration::{block, decl};
use crate::parser::nom::expression::expr;
use crate::parser::nom::keywords::while_keyword;
use crate::parser::nom::lexical_structure::{identifier, whitespace0, whitespace1};
use nom::branch::alt;
use nom::character::complete::{char, digit1, one_of, space0, space1};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{Err, IResult};

pub fn decl_stmt(s: &str) -> IResult<&str, Stmt> {
    map(decl, |d| Stmt::Decl { decl: d })(s)
}

pub fn expr_stmt(s: &str) -> IResult<&str, Stmt> {
    map(expr, |e| Stmt::Expr { expr: e })(s)
}

pub fn assignment_stmt(s: &str) -> IResult<&str, Stmt> {
    map(
        tuple((identifier, whitespace0, char('='), whitespace0, expr)),
        |(name, _, _, _, e)| {
            Stmt::Assignment(AssignmentStmt {
                target: name,
                value: e,
            })
        },
    )(s)
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

pub fn file(s: &str) -> IResult<&str, File> {
    map(many0(tuple((whitespace0, decl, whitespace0))), |decls| {
        File {
            body: decls.into_iter().map(|(_, f, _)| f).collect(),
        }
    })(s)
}

mod tests {
    use crate::ast::block::Block;
    use crate::ast::expr::Expr;
    use crate::ast::literal::Literal;
    use crate::ast::stmt::{AssignmentStmt, LoopStmt, Stmt};
    use crate::parser::nom::while_stmt;

    #[test]
    fn test_while_stmt() {
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
                        left: Box::new(Expr::Name {
                            name: "a".to_string()
                        }),
                        kind: "<".to_string(),
                        right: Box::new(Expr::Name {
                            name: "b".to_string()
                        })
                    },
                    block: Block {
                        body: vec![Stmt::Assignment(AssignmentStmt {
                            target: "a".to_string(),
                            value: Expr::BinOp {
                                left: Box::new(Expr::Name {
                                    name: "a".to_string()
                                }),
                                kind: "+".to_string(),
                                right: Box::new(Expr::Literal {
                                    literal: Literal::IntegerLiteral {
                                        value: "1".to_string()
                                    }
                                })
                            }
                        })]
                    }
                }
            ))
        )
    }
}
