use nom::{IResult, Parser};
use crate::ast::literal::Literal;
use nom::character::complete::{digit1, one_of, char, anychar};
use crate::ast::expr::Expr;
use nom::combinator::{map, opt, iterator};
use nom::sequence::tuple;
use nom::branch::alt;
use crate::parser::nom::lexical_structure::{identifier, whitespace0, whitespace1};
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

fn _binop(e:Expr, v: Vec<(&str, String, &str, Expr)>) -> Expr {
    let mut bin_op = e;
    for (_, op, _, ex) in v {
        bin_op = Expr::BinOp {
            left: Box::new(bin_op),
            kind: op,
            right: Box::new(ex),
        }
    }
    bin_op
}
// &&
pub fn conjunction_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char('&'), char('&'))), |(a, b)| { a.to_string() + &*b.to_string() })(s)
}

/*
<conjunction_expr> ::= <equality_expr> ("&&" <equality_expr>)*
*/
pub fn conjunction_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            equality_expr,
            many0(tuple((
                whitespace0,
                conjunction_operator,
                whitespace0,
                equality_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}
/*
<equality_expr> ::= <comparison_expr> (<equality_operator> <comparison_expr>)*
*/
pub fn equality_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            comparison_expr,
            many0(tuple((
                whitespace0,
                equality_operator,
                whitespace0,
                comparison_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<equality_operator> ::= "==" | "!="
*/
pub fn equality_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('='), char('='))), |(a, b)| {a.to_string() + &*b.to_string() }),
        map(tuple((char('!'), char('='))), |(a, b)| {a.to_string() + &*b.to_string() }),
        ))(s)
}

/*
<comparison_expr> ::= <generic_call_like_comparison_expr> (<comparison_operator> <generic_call_like_comparison_expr>)*
*/
pub fn comparison_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            generic_call_like_comparison_expr,
            many0(tuple((
                whitespace0,
                comparison_operator,
                whitespace0,
                generic_call_like_comparison_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<comparison_operator> ::= "<"  | ">"  | "<="  | ">="
*/
pub fn comparison_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('<'), char('='))), |(a, b)| {a.to_string() + &*b.to_string() }),
        map(tuple((char('>'), char('='))), |(a, b)| {a.to_string() + &*b.to_string() }),
        map(char('='), |a| {a.to_string() }),
        map(char('='), |a| {a.to_string() }),
        ))(s)
}

pub fn call_suffix(s: &str) -> IResult<&str, String> {
    // TODO: impl
    map(char(' '), |c|{c.to_string()})(s)
}
/*
<generic_call_like_comparison_expr> ::= <infix_operation_expr> <call_suffix>*
*/
pub fn generic_call_like_comparison_expr(s: &str) -> IResult<&str, Expr> {
    map(tuple((
        infix_operation_expr,
        many0(call_suffix)
        )), |(e, calls)| {
        // TODO: use calls
        e
    })(s)
}

/*
<infix_operation_expr> ::= <elvis_expr> ((<in_operator> <elvis_expr>) | (<is_operator> <type>))*
*/
pub fn infix_operation_expr(s: &str) -> IResult<&str, Expr> {
    enum P {
        IN {
            op: String,
            expr: Expr
        },
        IS {
            op: String,
            type_: String,
        },
    }
    map(
        tuple((
            elvis_expr,
            many0(alt((
                map(tuple((
                    whitespace1,
                    in_operator,
                    whitespace1,
                    elvis_expr,
                )), |(_, op, _, expr)| {
                    P::IN { op, expr }
                }),
                map(tuple((
                    whitespace1,
                    is_operator,
                    whitespace1,
                    type_,
                )), |(_, op, _, type_)| {
                    P::IS {op, type_}
                })
                )))
        )),
        |(op, v)| {
            let mut bin_op = op;
            for p in v {
                match p {
                    P::IS{ op, type_ } => {
                        bin_op = Expr::TypeCast {
                            target: Box::new(bin_op),
                            is_safe: op.ends_with("?"),
                            type_
                        }
                    },
                    P::IN{op, expr} => {
                        bin_op = Expr::BinOp {
                            left: Box::new(bin_op),
                            kind: op,
                            right: Box::new(expr)
                        }
                    }
                }
            }
            bin_op
        }
    )(s)
}

/*
<elvis_expr> ::= <infix_function_call> (":?" <infix_function_call_expr>)*
*/
pub fn elvis_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            infix_function_call_expr,
            many0(tuple((
                whitespace0,
                elvis_operator,
                whitespace0,
                infix_function_call_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

pub fn elvis_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char(':'), char('?'))), |(a, b)| { a.to_string() + &*b.to_string() })(s)
}

/*
<infix_function_call_expr> ::= <range_expr> (<identifier> <range_expr>)*
*/
pub fn infix_function_call_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            range_expr,
            many0(tuple((
                whitespace0,
                identifier,
                whitespace0,
                range_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<range_expr> ::= <additive_expr> (<range_operator> <additive_expr>)*
*/
pub fn range_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            additive_expr,
            many0(tuple((
                whitespace0,
                range_operator,
                whitespace0,
                additive_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<range_operator> ::= "..." || "..<"
*/
pub fn range_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('.'), char('.'), char('.'))), |(a, b, c)| { a.to_string() + &*b.to_string() + &*c.to_string() }),
        map(tuple((char('.'), char('.'), char('<'))), |(a, b, c)| { a.to_string() + &*b.to_string() + &*c.to_string() })
    ))(s)
}

/*
<additive_expr> ::= <multiplicative_expr> (<additive_operator> <multiplicative_expr>)*
*/
pub fn additive_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            multiplicative_expr,
            many0(tuple((
                whitespace0,
                additive_operator,
                whitespace0,
                multiplicative_expr,
            )))
        )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<additive_operator> ::= "+" | "-"
*/
pub fn additive_operator(s: &str) -> IResult<&str, String> {
    map(alt((
        char('+'),
        char('-'),
        )), |c| c.to_string())(s)
}

/*
<multiplicative_expr> ::= <as_expr> (<multiplicative_operator> <as_expr>)*
*/
pub fn multiplicative_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            as_expr,
            many0(tuple((
                whitespace0,
                multiplicative_operator,
                whitespace0,
                as_expr,
                )))
            )),
        |(op, v)| {
            _binop(op, v)
        }
    )(s)
}

/*
<multiplicative_operator> ::= "*" | "/" | "%"
*/
pub fn multiplicative_operator(s: &str) -> IResult<&str, String> {
    map(alt((
        char('*'),
        char('/'),
        char('%'),
    )), |c|{c.to_string()})(s)
}

/*
<as_expr> ::= <prefix_expr> (<as_operator> <type>)*
*/
pub fn as_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            prefix_expr,
            many0(tuple((
                whitespace1,
                as_operator,
                whitespace1,
                type_,
            )))
        )),
        |(e, v), | {
            let mut bin_op = e;
            for (_, op, _, typ) in v {
                bin_op = Expr::TypeCast {
                    target: Box::new(bin_op),
                    is_safe: op.ends_with("?"),
                    type_: typ,
                }
            }
            bin_op
        }
    )(s)
}

pub fn type_(s: &str) -> IResult<&str, String> {
    // TODO:
    identifier(s)
}

pub fn as_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('a'), char('s'), char('?'))), |(a, b, c)| { a.to_string() + &*b.to_string() + &*c.to_string() }),
        map(tuple((char('a'), char('s'))), |(a, b)| { a.to_string() + &*b.to_string() })
    ))(s)
}

pub fn in_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('i'), char('n'), char('!'))), |(a, b, c)| { a.to_string() + &*b.to_string() + &*c.to_string() }),
        map(tuple((char('i'), char('n'))), |(a, b)| { a.to_string() + &*b.to_string() })
    ))(s)
}

pub fn is_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('i'), char('s'), char('!'))), |(a, b, c)| { a.to_string() + &*b.to_string() + &*c.to_string() }),
        map(tuple((char('i'), char('s'))), |(a, b)| { a.to_string() + &*b.to_string() })
    ))(s)
}

pub fn disjunction_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char('|'), char('|'))), |(a, b)| { a.to_string() + &*b.to_string() })(s)
}

pub fn disjunction_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            conjunction_expr,
            many0(tuple((
                whitespace0,
                disjunction_operator,
                whitespace0,
                conjunction_expr,
            )))
        )), |(e, v)| {
            _binop(e, v)
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
    #[test]
    fn test_disjunction_expr() {
        assert_eq!(disjunction_expr("1||2 || 3"), Ok(("", BinOp { left: Box::from(BinOp { left: Box::from(Literal { literal: IntegerLiteral { value: "1".parse().unwrap() } }), kind: "||".parse().unwrap(), right: Box::from(Literal { literal: IntegerLiteral { value: "2".parse().unwrap() } }) }), kind: "||".parse().unwrap(), right: Box::from(Literal { literal: IntegerLiteral { value: "3".parse().unwrap() } }) })))
    }
}
