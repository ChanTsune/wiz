pub mod lexical_structure;
pub mod expression;
pub mod character;
pub mod combinator;
pub mod type_;
pub mod declaration;
pub mod keywords;

use nom::{Err, IResult};
use nom::character::complete::{digit1, space0, space1, one_of, char};
use crate::ast::literal::Literal;
use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::stmt::{Stmt, AssignmentStmt, LoopStmt};
use crate::ast::decl::Decl;
use nom::sequence::tuple;
use nom::combinator::map;
use nom::branch::alt;
use crate::parser::nom::expression::expr;
use nom::multi::many0;
use crate::parser::nom::declaration::{decl, block};
use crate::parser::nom::lexical_structure::{whitespace0, identifier, whitespace1};
use crate::parser::nom::keywords::while_keyword;


pub fn decl_stmt(s: &str) -> IResult<&str, Stmt> {
    map(decl, |d| {
        Stmt::Decl { decl: d }
    })(s)
}

pub fn expr_stmt(s: &str) -> IResult<&str, Stmt> {
    map(
        expr,
        |e| {
            Stmt::Expr { expr: e }
        })(s)
}

pub fn assignment_stmt(s: &str) -> IResult<&str, Stmt> {
    map(tuple((
        identifier,
        whitespace0,
        char('='),
        whitespace0,
        expr,
    )), |(name, _, _, _, e)| {
        Stmt::Assignment(AssignmentStmt { target: name, value: e })
    })(s)
}

pub fn loop_stmt(s: &str) -> IResult<&str, Stmt> {
    map(alt((
        while_stmt,
        while_stmt,
    )), |l| {
        Stmt::Loop(l)
    })(s)
}

pub fn while_stmt(s: &str) -> IResult<&str, LoopStmt> {
    map(tuple((
        while_keyword,
        whitespace1,
        expr,
        whitespace1,
        block,
    )), |(_, _, e, _, b)| {
        LoopStmt::While { condition: e, block: b }
    })(s)
}

pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    map(tuple((
        whitespace0,
        alt((
            decl_stmt,
            assignment_stmt,
            loop_stmt,
            expr_stmt,
        ))
    )), |(ws, stm)| {
        stm
    })(s)
}

pub fn stmts(s: &str) -> IResult<&str, Vec<Stmt>> {
    many0(stmt)(s)
}

pub fn file(s: &str) -> IResult<&str, File> {
    map(many0(tuple((
        whitespace0,
        decl,
        whitespace0,
    ))), |decls| {
        File { body: decls.into_iter().map(|(_, f, _)| { f }).collect() }
    })(s)
}

mod tests {
    use crate::parser::nom::while_stmt;
    use crate::ast::stmt::{LoopStmt, Stmt, AssignmentStmt};
    use crate::ast::block::Block;
    use crate::ast::expr::Expr;
    use crate::ast::literal::Literal;

    #[test]
    fn test_while_stmt() {
        assert_eq!(while_stmt(
            r"while (a < b) {
            a = a + 1
        }"), Ok(("", LoopStmt::While { condition: Expr::BinOp {
            left: Box::new(Expr::Name { name: "a".to_string() }),
            kind: "<".to_string(),
            right: Box::new(Expr::Name { name: "b".to_string() })
        }, block: Block { body: vec![
            Stmt::Assignment(AssignmentStmt{ target: "a".to_string(), value: Expr::BinOp {
                left: Box::new(Expr::Name { name: "a".to_string() }),
                kind: "+".to_string(),
                right: Box::new(Expr::Literal { literal: Literal::IntegerLiteral { value: "1".to_string() } })
            } })
        ] } })))
    }
}
