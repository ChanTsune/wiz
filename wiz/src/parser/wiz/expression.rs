use crate::parser::wiz::character::{comma, dot, double_quote};
use crate::parser::wiz::declaration::block;
use crate::parser::wiz::keywords::{
    as_keyword, else_keyword, false_keyword, if_keyword, in_keyword, return_keyword, true_keyword,
};
use crate::parser::wiz::lexical_structure::{
    identifier, whitespace0, whitespace1, whitespace_without_eol0,
};
use crate::parser::wiz::operators::member_access_operator;
use crate::parser::wiz::statement::stmts;
use crate::parser::wiz::type_::{type_, type_arguments};
use crate::syntax::block::Block;
use crate::syntax::expr::{
    CallArg, CallExprSyntax, Expr, LambdaSyntax, NameExprSyntax, PostfixSuffix, ReturnSyntax,
    SubscriptSyntax,
};
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::stmt::Stmt;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;
use nom::branch::{alt, permutation};
use nom::bytes::complete::take_until;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{
    AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Slice,
};
use std::ops::RangeFrom;

pub fn integer_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: InputTake + ToString + InputLength + InputIter + Clone + InputTakeAtPosition,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(digit1, |n: I| LiteralSyntax::Integer {
        value: n.to_string(),
    })(s)
}

pub fn floating_point_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: InputTake
        + Compare<&'static str>
        + Slice<RangeFrom<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTakeAtPosition,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(tuple((digit1, dot, digit1)), |(i, d, f): (I, char, I)| {
        LiteralSyntax::FloatingPoint(TokenSyntax::new(
            i.to_string() + &*d.to_string() + &*f.to_string(),
        ))
    })(s)
}

pub fn string_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: InputTake
        + Compare<&'static str>
        + Clone
        + FindSubstring<&'static str>
        + Slice<RangeFrom<usize>>
        + InputIter
        + ToString,
    <I as InputIter>::Item: AsChar,
{
    map(
        permutation((double_quote, take_until("\""), double_quote)),
        |(a, b, c): (char, I, char)| LiteralSyntax::String {
            open_quote: TokenSyntax::new(a.to_string()),
            value: b.to_string(),
            close_quote: TokenSyntax::new(c.to_string()),
        },
    )(s)
}

pub fn boolean_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: InputTake + Compare<&'static str> + Clone + ToString,
{
    map(alt((true_keyword, false_keyword)), |b: I| {
        LiteralSyntax::Boolean(TokenSyntax {
            leading_trivia: Trivia::new(),
            token: b.to_string(),
            trailing_trivia: (Trivia::new()),
        })
    })(s)
}

pub fn prefix_operator(s: &str) -> IResult<&str, String> {
    map(
        alt((char('+'), char('-'), char('!'), char('*'), char('&'))),
        |c| c.to_string(),
    )(s)
}

pub fn literal_expr(s: &str) -> IResult<&str, Expr> {
    map(
        alt((
            boolean_literal,
            floating_point_literal,
            integer_literal,
            string_literal,
        )),
        |l| Expr::Literal(l),
    )(s)
}

pub fn name_expr(s: &str) -> IResult<&str, Expr> {
    map(identifier, |name| Expr::Name(NameExprSyntax { name }))(s)
}

pub fn parenthesized_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((char('('), whitespace0, expr, whitespace0, char(')'))),
        |(_, _, expr, _, _)| expr,
    )(s)
}

pub fn return_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((return_keyword, whitespace1, opt(expr))),
        |(_, _, e)| {
            Expr::Return(ReturnSyntax {
                value: e.map(|i| Box::new(i)),
            })
        },
    )(s)
}

pub fn primary_expr(s: &str) -> IResult<&str, Expr> {
    alt((
        return_expr,
        if_expr,
        name_expr,
        literal_expr,
        parenthesized_expr,
    ))(s)
}
/*
<if> ::= "if" <expr> <block> ("else" (<block> | <if>))?
*/
pub fn if_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            if_keyword,
            whitespace1,
            expr,
            whitespace0,
            block,
            opt(map(
                tuple((
                    whitespace0,
                    else_keyword,
                    whitespace0,
                    alt((
                        block,
                        map(if_expr, |ib| Block {
                            body: vec![Stmt::Expr { expr: ib }],
                        }),
                    )),
                )),
                |(_, _, _, e)| e,
            )),
        )),
        |(_, _, condition, _, body, else_body)| Expr::If {
            condition: Box::new(condition),
            body,
            else_body,
        },
    )(s)
}

/*
<postfix_expr> ::= <primary_expr> <postfix_suffix>*
*/
pub fn postfix_expr(s: &str) -> IResult<&str, Expr> {
    map(_postfix_expr, |(e, suffixes)| {
        let mut e = e;
        for suffix in suffixes {
            e = match suffix {
                // TODO: impl
                PostfixSuffix::Operator { .. } => e,
                PostfixSuffix::TypeArgumentSuffix { .. } => e,
                PostfixSuffix::CallSuffix {
                    args,
                    tailing_lambda,
                } => Expr::Call(CallExprSyntax {
                    target: Box::new(e),
                    args,
                    tailing_lambda,
                }),
                PostfixSuffix::IndexingSuffix { indexes } => Expr::Subscript(SubscriptSyntax {
                    target: Box::new(e),
                    idx_or_keys: indexes,
                }),
                PostfixSuffix::NavigationSuffix { is_safe, name } => Expr::Member {
                    target: Box::new(e),
                    name,
                    is_safe,
                },
            }
        }
        e
    })(s)
}

pub fn _postfix_expr(s: &str) -> IResult<&str, (Expr, Vec<PostfixSuffix>)> {
    tuple((primary_expr, many0(postfix_suffix)))(s)
}

/*
<postfix_suffix> ::= <postfix_operator>
                   | <type_arguments>
                   | <call_suffix>
                   | <indexing_suffix>
                   | <navigation_suffix>
*/
pub fn postfix_suffix(s: &str) -> IResult<&str, PostfixSuffix> {
    alt((
        map(postfix_operator, |s| PostfixSuffix::Operator { kind: s }),
        map(type_arguments, |type_names| {
            PostfixSuffix::TypeArgumentSuffix { types: type_names }
        }),
        call_suffix,
        indexing_suffix,
        navigation_suffix,
    ))(s)
}

/*
<navigation_suffix> ::= <member_access_operator> <identifier>
*/
pub fn navigation_suffix(s: &str) -> IResult<&str, PostfixSuffix> {
    map(tuple((member_access_operator, identifier)), |(op, name)| {
        PostfixSuffix::NavigationSuffix {
            is_safe: op == "?.",
            name,
        }
    })(s)
}

pub fn postfix_operator(s: &str) -> IResult<&str, String> {
    map(char('!'), |c| c.to_string())(s)
}

// <indexing_suffix> ::= "[" <expr> ("," <expr>)* ","? "]"
pub fn indexing_suffix(s: &str) -> IResult<&str, PostfixSuffix> {
    map(
        tuple((
            char('['),
            whitespace0,
            expr,
            whitespace0,
            many0(tuple((comma, whitespace0, expr))),
            whitespace0,
            opt(comma),
            whitespace0,
            char(']'),
        )),
        |(_, _, ex, _, exs, _, _, _, _)| PostfixSuffix::IndexingSuffix {
            indexes: vec![ex]
                .into_iter()
                .chain(exs.into_iter().map(|(_, _, e)| e))
                .collect(),
        },
    )(s)
}

pub fn prefix_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((opt(prefix_operator), postfix_expr)),
        |(op, postfix)| match op {
            Some(op) => Expr::UnaryOp {
                target: Box::new(postfix),
                prefix: true,
                kind: op,
            },
            None => postfix,
        },
    )(s)
}

fn _binop(e: Expr, v: Vec<(Trivia, String, Trivia, Expr)>) -> Expr {
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
    map(tuple((char('&'), char('&'))), |(a, b)| {
        a.to_string() + &*b.to_string()
    })(s)
}

/*
<conjunction_expr> ::= <equality_expr> ("&&" <equality_expr>)*
*/
pub fn conjunction_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            equality_expr,
            many0(tuple((
                whitespace_without_eol0,
                conjunction_operator,
                whitespace0,
                equality_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
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
                whitespace_without_eol0,
                equality_operator,
                whitespace0,
                comparison_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

/*
<equality_operator> ::= "==" | "!="
*/
pub fn equality_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('='), char('='))), |(a, b)| {
            a.to_string() + &*b.to_string()
        }),
        map(tuple((char('!'), char('='))), |(a, b)| {
            a.to_string() + &*b.to_string()
        }),
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
                whitespace_without_eol0,
                comparison_operator,
                whitespace0,
                generic_call_like_comparison_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

/*
<comparison_operator> ::= "<"  | ">"  | "<="  | ">="
*/
pub fn comparison_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('<'), char('='))), |(a, b)| {
            a.to_string() + &*b.to_string()
        }),
        map(tuple((char('>'), char('='))), |(a, b)| {
            a.to_string() + &*b.to_string()
        }),
        map(char('<'), |a| a.to_string()),
        map(char('>'), |a| a.to_string()),
    ))(s)
}
/*
<call_suffix> ::= <type_arguments>? ((<value_arguments>? <annotated_lambda>) | <value_arguments>)
*/
pub fn call_suffix(s: &str) -> IResult<&str, PostfixSuffix> {
    map(
        tuple((
            opt(type_arguments),
            alt((
                map(
                    tuple((opt(value_arguments), annotated_lambda)),
                    |(args, l)| (args, Option::Some(l)),
                ),
                map(value_arguments, |v| (Option::Some(v), Option::None)),
            )),
        )),
        |(ta, (args, tl))| PostfixSuffix::CallSuffix {
            args: args.unwrap_or(vec![]),
            tailing_lambda: tl,
        },
    )(s)
}
/*
<value_arguments> ::= "(" (<value_argument> ("," <value_argument>)* ","?)? ")"
*/
pub fn value_arguments(s: &str) -> IResult<&str, Vec<CallArg>> {
    map(
        tuple((
            char('('),
            opt(tuple((
                value_argument,
                many0(tuple((comma, value_argument))),
                opt(comma),
            ))),
            char(')'),
        )),
        |(_, args_t, _)| {
            let mut args = vec![];
            match args_t {
                Some((a, ags, _)) => {
                    args.insert(args.len(), a);
                    for (_, ar) in ags {
                        args.insert(args.len(), ar);
                    }
                }
                None => {}
            };
            args
        },
    )(s)
}
/*
<value_argument> ::= (<identifier> ":")? "*"? <expr>
*/
pub fn value_argument(s: &str) -> IResult<&str, CallArg> {
    map(
        tuple((
            whitespace0,
            opt(tuple((identifier, whitespace0, char(':'), whitespace0))),
            opt(char('*')),
            expr,
        )),
        |(_, arg_label, is_vararg, arg)| CallArg {
            label: arg_label.map(|(label, _, _, _)| label),
            arg: Box::new(arg),
            is_vararg: match is_vararg {
                None => false,
                Some(_) => true,
            },
        },
    )(s)
}
/*
<annotated_lambda> ::= <label>? <lambda_literal>
*/
pub fn annotated_lambda(s: &str) -> IResult<&str, LambdaSyntax> {
    map(
        tuple((
            opt(label), // TODO: label
            lambda_literal,
        )),
        |(l, lmd)| lmd,
    )(s)
}

pub fn lambda_literal(s: &str) -> IResult<&str, LambdaSyntax> {
    map(tuple((char('{'), stmts, char('}'))), |(_, stms, _)| {
        LambdaSyntax { stmts: stms }
    })(s)
}

pub fn label(s: &str) -> IResult<&str, char> {
    // TODO: Impl
    char(' ')(s)
}

/*
<generic_call_like_comparison_expr> ::= <infix_operation_expr> <call_suffix>*
*/
pub fn generic_call_like_comparison_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((infix_operation_expr, many0(call_suffix))),
        |(e, calls)| {
            // TODO: use calls
            e
        },
    )(s)
}

/*
<infix_operation_expr> ::= <elvis_expr> ((<in_operator> <elvis_expr>) | (<is_operator> <type>))*
*/
pub fn infix_operation_expr(s: &str) -> IResult<&str, Expr> {
    enum P {
        IN { op: String, expr: Expr },
        IS { op: String, type_: TypeName },
    }
    map(
        tuple((
            elvis_expr,
            many0(alt((
                map(
                    tuple((whitespace1, in_operator, whitespace1, elvis_expr)),
                    |(_, op, _, expr)| P::IN { op, expr },
                ),
                map(
                    tuple((whitespace1, is_operator, whitespace1, type_)),
                    |(_, op, _, type_)| P::IS { op, type_ },
                ),
            ))),
        )),
        |(op, v)| {
            let mut bin_op = op;
            for p in v {
                match p {
                    P::IS { op, type_ } => {
                        bin_op = Expr::TypeCast {
                            target: Box::new(bin_op),
                            is_safe: op.ends_with("?"),
                            type_,
                        }
                    }
                    P::IN { op, expr } => {
                        bin_op = Expr::BinOp {
                            left: Box::new(bin_op),
                            kind: op,
                            right: Box::new(expr),
                        }
                    }
                }
            }
            bin_op
        },
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
                whitespace_without_eol0,
                elvis_operator,
                whitespace0,
                infix_function_call_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

pub fn elvis_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char(':'), char('?'))), |(a, b)| {
        a.to_string() + &*b.to_string()
    })(s)
}

/*
<infix_function_call_expr> ::= <range_expr> (<identifier> <range_expr>)*
*/
pub fn infix_function_call_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            range_expr,
            many0(tuple((
                whitespace_without_eol0,
                identifier,
                whitespace0,
                range_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
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
                whitespace_without_eol0,
                range_operator,
                whitespace0,
                additive_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

/*
<range_operator> ::= "..." || "..<"
*/
pub fn range_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('.'), char('.'), char('.'))), |(a, b, c)| {
            a.to_string() + &*b.to_string() + &*c.to_string()
        }),
        map(tuple((char('.'), char('.'), char('<'))), |(a, b, c)| {
            a.to_string() + &*b.to_string() + &*c.to_string()
        }),
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
                whitespace_without_eol0,
                additive_operator,
                whitespace0,
                multiplicative_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

/*
<additive_operator> ::= "+" | "-"
*/
pub fn additive_operator(s: &str) -> IResult<&str, String> {
    map(alt((char('+'), char('-'))), |c| c.to_string())(s)
}

/*
<multiplicative_expr> ::= <as_expr> (<multiplicative_operator> <as_expr>)*
*/
pub fn multiplicative_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            as_expr,
            many0(tuple((
                whitespace_without_eol0,
                multiplicative_operator,
                whitespace0,
                as_expr,
            ))),
        )),
        |(op, v)| _binop(op, v),
    )(s)
}

/*
<multiplicative_operator> ::= "*" | "/" | "%"
*/
pub fn multiplicative_operator(s: &str) -> IResult<&str, String> {
    map(alt((char('*'), char('/'), char('%'))), |c| c.to_string())(s)
}

/*
<as_expr> ::= <prefix_expr> (<as_operator> <type>)*
*/
pub fn as_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            prefix_expr,
            many0(tuple((whitespace1, as_operator, whitespace1, type_))),
        )),
        |(e, v)| {
            let mut bin_op = e;
            for (_, op, _, typ) in v {
                bin_op = Expr::TypeCast {
                    target: Box::new(bin_op),
                    is_safe: op.ends_with("?"),
                    type_: typ,
                }
            }
            bin_op
        },
    )(s)
}

pub fn as_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((as_keyword, char('?'))), |(a, b): (&str, _)| {
            a.to_string() + &*b.to_string()
        }),
        map(as_keyword, |a: &str| a.to_string()),
    ))(s)
}

pub fn in_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((in_keyword, char('!'))), |(a, b): (&str, _)| {
            a.to_string() + &*b.to_string()
        }),
        map(in_keyword, |a: &str| a.to_string()),
    ))(s)
}

pub fn is_operator(s: &str) -> IResult<&str, String> {
    alt((
        map(tuple((char('i'), char('s'), char('!'))), |(a, b, c)| {
            a.to_string() + &*b.to_string() + &*c.to_string()
        }),
        map(tuple((char('i'), char('s'))), |(a, b)| {
            a.to_string() + &*b.to_string()
        }),
    ))(s)
}

pub fn disjunction_operator(s: &str) -> IResult<&str, String> {
    map(tuple((char('|'), char('|'))), |(a, b)| {
        a.to_string() + &*b.to_string()
    })(s)
}

pub fn disjunction_expr(s: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            conjunction_expr,
            many0(tuple((
                whitespace_without_eol0,
                disjunction_operator,
                whitespace0,
                conjunction_expr,
            ))),
        )),
        |(e, v)| _binop(e, v),
    )(s)
}

pub fn expr(s: &str) -> IResult<&str, Expr> {
    disjunction_expr(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::expression::{
        boolean_literal, conjunction_expr, disjunction_expr, equality_expr, expr,
        floating_point_literal, indexing_suffix, integer_literal, literal_expr, postfix_suffix,
        return_expr, string_literal, value_arguments,
    };
    use crate::syntax::block::Block;
    use crate::syntax::expr::Expr::{BinOp, If};
    use crate::syntax::expr::{
        CallArg, CallExprSyntax, Expr, NameExprSyntax, PostfixSuffix, ReturnSyntax,
    };
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::Trivia;

    #[test]
    fn test_integer_literal() {
        assert_eq!(
            integer_literal("1"),
            Ok((
                "",
                LiteralSyntax::Integer {
                    value: "1".to_string()
                }
            ))
        );
        assert_eq!(
            integer_literal("12"),
            Ok((
                "",
                LiteralSyntax::Integer {
                    value: "12".to_string()
                }
            ))
        );
    }

    #[test]
    fn test_floating_point_literal() {
        assert_eq!(
            floating_point_literal("1.0"),
            Ok((
                "",
                LiteralSyntax::FloatingPoint(TokenSyntax::new("1.0".to_string()))
            ))
        );
        assert_eq!(
            floating_point_literal("12.0"),
            Ok((
                "",
                LiteralSyntax::FloatingPoint(TokenSyntax::new("12.0".to_string()))
            ))
        );
        assert_eq!(
            floating_point_literal("13847.03478"),
            Ok((
                "",
                LiteralSyntax::FloatingPoint(TokenSyntax::new("13847.03478".to_string()))
            ))
        );
    }

    #[test]
    fn test_number_literal() {
        assert_eq!(
            literal_expr("1.1"),
            Ok((
                "",
                Expr::Literal(LiteralSyntax::FloatingPoint(TokenSyntax::new(
                    String::from("1.1")
                )))
            ))
        );
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            string_literal("\"\""),
            Ok((
                "",
                LiteralSyntax::String {
                    open_quote: TokenSyntax::new('"'.to_string()),
                    value: "".to_string(),
                    close_quote: TokenSyntax::new('"'.to_string())
                }
            ))
        );
    }

    #[test]
    fn test_boolean_literal() {
        assert_eq!(
            boolean_literal("true"),
            Ok((
                "",
                LiteralSyntax::Boolean(TokenSyntax {
                    leading_trivia: Trivia::new(),
                    token: "true".to_string(),
                    trailing_trivia: Trivia::new()
                })
            ))
        )
    }

    #[test]
    fn test_disjunction_expr() {
        assert_eq!(
            disjunction_expr("1||2 || 3"),
            Ok((
                "",
                BinOp {
                    left: Box::from(BinOp {
                        left: Box::from(Expr::Literal(LiteralSyntax::Integer {
                            value: "1".parse().unwrap()
                        })),
                        kind: "||".parse().unwrap(),
                        right: Box::from(Expr::Literal(LiteralSyntax::Integer {
                            value: "2".parse().unwrap()
                        }))
                    }),
                    kind: "||".parse().unwrap(),
                    right: Box::from(Expr::Literal(LiteralSyntax::Integer {
                        value: "3".parse().unwrap()
                    }))
                }
            ))
        )
    }

    #[test]
    fn test_conjunction_expr() {
        assert_eq!(
            conjunction_expr(
                r"1 &&
            2 && 3"
            ),
            Ok((
                "",
                BinOp {
                    left: Box::from(BinOp {
                        left: Box::from(Expr::Literal(LiteralSyntax::Integer {
                            value: "1".parse().unwrap()
                        })),
                        kind: "&&".parse().unwrap(),
                        right: Box::from(Expr::Literal(LiteralSyntax::Integer {
                            value: "2".parse().unwrap()
                        }))
                    }),
                    kind: "&&".parse().unwrap(),
                    right: Box::from(Expr::Literal(LiteralSyntax::Integer {
                        value: "3".parse().unwrap()
                    }))
                }
            ))
        )
    }

    #[test]
    fn test_equality_expr() {
        assert_eq!(
            equality_expr(
                r"1
            && 2"
            ),
            Ok((
                "\n            && 2",
                Expr::Literal(LiteralSyntax::Integer {
                    value: "1".parse().unwrap()
                })
            ))
        )
    }

    #[test]
    fn test_value_arguments_no_args() {
        assert_eq!(value_arguments("()"), Ok(("", vec![])))
    }

    #[test]
    fn test_value_arguments_no_labeled_args() {
        assert_eq!(
            value_arguments("(\"Hello, World\")"),
            Ok((
                "",
                vec![CallArg {
                    label: None,
                    arg: Box::from(Expr::Literal(LiteralSyntax::String {
                        open_quote: TokenSyntax::new('"'.to_string()),
                        value: "Hello, World".parse().unwrap(),
                        close_quote: TokenSyntax::new('"'.to_string()),
                    })),
                    is_vararg: false
                }]
            ))
        )
    }

    #[test]
    fn test_postfix_suffix_call() {
        assert_eq!(
            postfix_suffix("()"),
            Ok((
                "",
                PostfixSuffix::CallSuffix {
                    args: vec![],
                    tailing_lambda: None
                }
            ))
        )
    }

    #[test]
    fn test_call_expr_no_args() {
        assert_eq!(
            expr("puts()"),
            Ok((
                "",
                Expr::Call(CallExprSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name: "puts".parse().unwrap()
                    })),
                    args: vec![],
                    tailing_lambda: None,
                })
            ))
        );
    }

    #[test]
    fn test_call_expr() {
        assert_eq!(
            expr("puts(\"Hello, World\")"),
            Ok((
                "",
                Expr::Call(CallExprSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name: "puts".parse().unwrap()
                    })),
                    args: vec![CallArg {
                        label: None,
                        arg: Box::from(Expr::Literal(LiteralSyntax::String {
                            open_quote: TokenSyntax::new('"'.to_string()),
                            value: "Hello, World".parse().unwrap(),
                            close_quote: TokenSyntax::new('"'.to_string()),
                        })),
                        is_vararg: false
                    }],
                    tailing_lambda: None,
                })
            ))
        );
    }

    #[test]
    fn test_call_expr_with_label() {
        assert_eq!(
            expr("puts(string: \"Hello, World\")"),
            Ok((
                "",
                Expr::Call(CallExprSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name: "puts".parse().unwrap()
                    })),
                    args: vec![CallArg {
                        label: Some(String::from("string")),
                        arg: Box::from(Expr::Literal(LiteralSyntax::String {
                            open_quote: TokenSyntax::new('"'.to_string()),
                            value: "Hello, World".parse().unwrap(),
                            close_quote: TokenSyntax::new('"'.to_string())
                        })),
                        is_vararg: false
                    }],
                    tailing_lambda: None,
                })
            ))
        );
    }

    #[test]
    fn test_if_expr() {
        assert_eq!(
            expr(r"if a { }"),
            Ok((
                "",
                If {
                    condition: Box::new(Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    })),
                    body: Block { body: vec![] },
                    else_body: None
                }
            ))
        )
    }

    #[test]
    fn test_if_expr_with_else() {
        assert_eq!(
            expr(r"if a { } else { }"),
            Ok((
                "",
                If {
                    condition: Box::new(Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    })),
                    body: Block { body: vec![] },
                    else_body: Some(Block { body: vec![] })
                }
            ))
        )
    }

    #[test]
    fn test_return() {
        assert_eq!(
            return_expr("return name"),
            Ok((
                "",
                Expr::Return(ReturnSyntax {
                    value: Some(Box::new(Expr::Name(NameExprSyntax {
                        name: "name".to_string()
                    })))
                })
            ))
        )
    }

    #[test]
    fn test_struct_member() {
        assert_eq!(
            expr("a.b"),
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
    fn test_index_suffix() {
        assert_eq!(
            indexing_suffix("[a]"),
            Ok((
                "",
                PostfixSuffix::IndexingSuffix {
                    indexes: vec![Expr::Name(NameExprSyntax {
                        name: "a".to_string()
                    }),]
                }
            ))
        );
        assert_eq!(
            indexing_suffix("[a, b]"),
            Ok((
                "",
                PostfixSuffix::IndexingSuffix {
                    indexes: vec![
                        Expr::Name(NameExprSyntax {
                            name: "a".to_string()
                        }),
                        Expr::Name(NameExprSyntax {
                            name: "b".to_string()
                        }),
                    ]
                }
            ))
        );
        assert_eq!(
            indexing_suffix("[a, b, ]"),
            Ok((
                "",
                PostfixSuffix::IndexingSuffix {
                    indexes: vec![
                        Expr::Name(NameExprSyntax {
                            name: "a".to_string()
                        }),
                        Expr::Name(NameExprSyntax {
                            name: "b".to_string()
                        }),
                    ]
                }
            ))
        );
    }
}
