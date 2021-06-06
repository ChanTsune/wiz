pub mod lexical_structure;
pub mod expression;
pub mod character;
pub mod combinator;
pub mod type_;
pub mod declaration;
pub mod keywords;

use nom::{Err, IResult};
use nom::character::complete::{digit1, space0, space1, one_of};
use crate::ast::literal::Literal;
use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::stmt::Stmt;
use crate::ast::decl::Decl;
use nom::sequence::tuple;
use nom::combinator::map;
use nom::branch::alt;
use crate::parser::nom::expression::expr;
use nom::multi::many0;
use crate::parser::nom::declaration::decl;


pub fn decl_stmt(s: &str) -> IResult<&str, Stmt> {
    map(decl, |d| {
        Stmt::Decl { decl: d }
    })(s)
}

pub fn expr_stmt(s: &str) -> IResult<&str, Stmt> {
    map(expr, |e| {
        Stmt::Expr { expr: e }
    })(s)
}

pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    alt((
        expr_stmt,
        decl_stmt,
    ))(s)
}

pub fn stmts(s: &str) -> IResult<&str, Vec<Stmt>> {
    many0(stmt)(s)
}

pub fn file(s: &str) -> IResult<&str, File> {
    map(many0(decl), |decls| {
        File{ body: decls }
    })(s)
}
