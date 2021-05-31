use nom::IResult;
use crate::ast::literal::Literal;
use nom::character::complete::{digit1, one_of};
use crate::ast::expr::Expr;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::branch::alt;

pub fn integer_literal(s: &str) -> IResult<&str, Literal> {
    digit1(s).map(|(s, n)| {
        (s, Literal::IntegerLiteral { value: n.to_string() })
    })
}

pub fn binary_operator(s: &str) -> IResult<&str, String> {
    one_of("+")(s).map(|(s, c)| {
        (s, c.to_string())
    })
}

pub fn literal_expr(s: &str) -> IResult<&str, Expr> {
    integer_literal(s).map(|(s, l)| {
        (s, Expr::Literal { literal: l })
    })
}

pub fn single_expr(s: &str) -> IResult<&str, Expr> {
    literal_expr(s)
}

pub fn binop_expr(s: &str) -> IResult<&str, Expr> {
    map(tuple((
        single_expr,
        binary_operator,
        single_expr,
    )), |(left, op_kind, right)| {
        Expr::BinOp {
            left: Box::from(left),
            kind: op_kind,
            right: Box::from(right),
        }
    })(s)
}

pub fn expr(s: &str) -> IResult<&str, Expr> {
    alt((
        binop_expr,
        literal_expr,
    ))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use nom::Err::Error;
    use crate::parser::nom::expression::integer_literal;
    use crate::ast::literal::Literal::IntegerLiteral;

    #[test]
    fn test_numeric() {
        // assert_eq!(integer_literal("1"), Ok(("", IntegerLiteral { value: "1".to_string() } )));
        // assert_eq!(integer_literal("12"), Ok(("", IntegerLiteral { value: "12".to_string() })));
    }
}
