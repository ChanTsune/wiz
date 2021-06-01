pub mod lexical_structure;
pub mod expression;
pub mod character;
pub mod combinator;

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


pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    expr(s).map(|(s, e)| {
        (s, Stmt::Expr { expr: e })
    })
}

// pub fn var_decl(s: &str) -> IResult<&str, Decl> {
//
// }
//
// pub fn decl(s: &str) -> IResult<&str, Decl> {
//
// }
//
// pub fn parse(s: &str) -> IResult<&str, File> {
//     integer_literal(s).map(|(s, )|)
// }
