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
use crate::ast::stmt::{Stmt, AssignmentStmt};
use crate::ast::decl::Decl;
use nom::sequence::tuple;
use nom::combinator::map;
use nom::branch::alt;
use crate::parser::nom::expression::expr;
use nom::multi::many0;
use crate::parser::nom::declaration::decl;
use crate::parser::nom::lexical_structure::{whitespace0, identifier};


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

pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    map(tuple((
        whitespace0,
        alt((
            decl_stmt,
            assignment_stmt,
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
