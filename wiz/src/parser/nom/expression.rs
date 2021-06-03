use nom::{IResult, Parser};
use crate::ast::literal::Literal;
use nom::character::complete::{digit1, one_of, char, anychar};
use crate::ast::expr::Expr;
use nom::combinator::{map, opt, iterator};
use nom::sequence::tuple;
use nom::branch::alt;
use crate::parser::nom::lexical_structure::{identifier, whitespace0};
use crate::ast::expr::Expr::BinOp;
use nom::multi::many0;
use nom::error::ParseError;

pub fn integer_literal(s: &str) -> IResult<&str, Literal> {
    map(digit1, |n: &str| {
        Literal::IntegerLiteral { value: n.to_string() }
    })(s)
}

pub fn string_literal(s: &str) -> IResult<&str, Literal> {
    map(tuple((
        char('"'),
        anychar,
        char('"'),
    )), |(a, b, c)| {
        Literal::StringLiteral { value: "".to_string() }
    })(s)
}

pub fn binary_operator(s: &str) -> IResult<&str, String> {
    map(one_of("+-*/%"), |c| {
        c.to_string()
    })(s)
}

pub fn prefix_operator(s: &str) -> IResult<&str, String> {
    map(one_of("+-!"), |c| {
        c.to_string()
    })(s)
}

pub fn literal_expr(s: &str) -> IResult<&str, Expr> {
    map(alt((
        integer_literal,
        string_literal,
    )), |l| {
        Expr::Literal { literal: l }
    })(s)
}

pub fn name_expr(s: &str) -> IResult<&str, Expr> {
    map(identifier, |name| {
        Expr::Name { name }
    })(s)
}

pub fn parenthesized_expr(s: &str) -> IResult<&str, Expr> {
    map(tuple((
        char('('),
        whitespace0,
        expr,
        whitespace0,
        char(')'),
    )), |(_, _, expr, _, _)| {
        expr
    })(s)
}

pub fn primary_expr(s: &str) -> IResult<&str, Expr> {
    alt((
        name_expr,
        literal_expr,
        parenthesized_expr,
    ))(s)
}

pub fn postfix_expr(s: &str) -> IResult<&str, Expr> {
    alt((
        primary_expr,
        primary_expr,
    ))(s)
}

pub fn prefix_expr(s: &str) -> IResult<&str, Expr> {
    map(tuple((
        opt(prefix_operator),
        postfix_expr,
    )), |(op, postfix)| {
        match op {
            Some(op) => Expr::UnaryOp {
                target: Box::new(postfix),
                prefix: true,
                kind: op,
            },
            None => postfix
        }
    })(s)
}

pub fn binary_expr(s: &str) -> IResult<&str, (String, Expr)> {
    map(tuple((
        whitespace0,
        binary_operator,
        whitespace0,
        expr,
    )), |(_, op_kind, _, expr)| {
        (op_kind, expr)
    })(s)
}

pub fn wrap<I: Clone, O, E: ParseError<I>>(
    left: impl FnMut(I) -> IResult<I, Expr, E>,
    op: impl FnMut(I) -> IResult<I, String, E>,
    right: impl FnMut(I) -> IResult<I, Expr, E>,
) -> impl FnMut(I) -> IResult<I, (Expr, String, Expr), E> {
    tuple((left, op, right))
}

pub fn or_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char('|'), char('|'))), |(a, b)| { a.to_string() + &*b.to_string() })(s)
}

pub fn disjunction_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            literal_expr,
            many0(tuple((
                whitespace0,
                or_operator,
                whitespace0,
                literal_expr,
            )))
        )), |(e, v)| {
            let mut bin_op = e;
            for (_, op, _, ex) in v {
                bin_op = Expr::BinOp {
                    left: Box::new(bin_op),
                    kind: op,
                    right: Box::new(ex),
                }
            }
            bin_op
        })(s)
}

pub fn expr(s: &str) -> IResult<&str, Expr> {
    disjunction_expr(s)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use nom::Err::Error;
    use crate::parser::nom::expression::{integer_literal, disjunction_expr};
    use crate::ast::literal::Literal::IntegerLiteral;
    use crate::ast::expr::Expr::{BinOp, Literal};

    #[test]
    fn test_numeric() {
        assert_eq!(integer_literal("1"), Ok(("", IntegerLiteral { value: "1".to_string() })));
        assert_eq!(integer_literal("12"), Ok(("", IntegerLiteral { value: "12".to_string() })));
    }

    fn test_disjunction_expr() {
        assert_eq!(disjunction_expr("1||2 || 3"), Ok(("", BinOp { left: Box::from(BinOp { left: Box::from(Literal { literal: IntegerLiteral { value: "1".parse().unwrap() } }), kind: "||".parse().unwrap(), right: Box::from(Literal { literal: IntegerLiteral { value: "2".parse().unwrap() } }) }), kind: "||".parse().unwrap(), right: Box::from(Literal { literal: IntegerLiteral { value: "3".parse().unwrap() } }) })))
    }
}
