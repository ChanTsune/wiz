use nom::{Err, IResult};
use nom::character::complete::{digit1, space0};
use crate::ast::literal::Literal;
use crate::ast::expr::Expr;
use crate::ast::file::File;
use crate::ast::stmt::Stmt;
use crate::ast::decl::Decl;


pub fn space(s: &str) -> IResult<&str, &str> {
    space0(s)
}

pub fn integer_literal(s: &str) -> IResult<&str, Literal> {
    digit1(s).map(|(s, n)| {
        (s, Literal::IntegerLiteral {value: n.to_string()})
    })
}

pub fn literal_expr(s: &str) -> IResult<&str, Expr> {
    integer_literal(s).map(|(s, l)| {
        (s, Expr::Literal {literal: l})
    })
}

pub fn expr(s: &str) -> IResult<&str, Expr> {
    literal_expr(s)
}

pub fn stmt(s: &str) -> IResult<&str, Stmt> {
    expr(s).map(|(s, e)| {
        (s, Stmt::Expr {expr: e})
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

#[cfg(test)]
mod tests {
    use crate::parser::nom::integer_literal;
    use nom::error::ErrorKind;
    use nom::Err::Error;

    #[test]
    fn test_numeric() {
        assert_eq!(integer_literal("1"), Ok(("", "1")));
        assert_eq!(integer_literal("12"), Ok(("", "12")));
    }
}
