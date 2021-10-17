use crate::parser::wiz::character::{comma, dot, double_quote, not_double_quote_or_back_slash};
use crate::parser::wiz::declaration::block;
use crate::parser::wiz::keywords::{
    else_keyword, false_keyword, if_keyword, return_keyword, true_keyword,
};
use crate::parser::wiz::lexical_structure::{
    identifier, whitespace0, whitespace1, whitespace_without_eol0,
};
use crate::parser::wiz::operators::{
    additive_operator, as_operator, comparison_operator, conjunction_operator,
    disjunction_operator, elvis_operator, equality_operator, in_operator, is_operator,
    member_access_operator, multiplicative_operator, postfix_operator, prefix_operator,
    range_operator,
};
use crate::parser::wiz::statement::stmts;
use crate::parser::wiz::type_::{type_, type_arguments};
use crate::syntax::block::BlockSyntax;
use crate::syntax::expr::{
    ArrayElementSyntax, ArraySyntax, BinaryOperationSyntax, CallArg, CallExprSyntax, Expr,
    IfExprSyntax, LambdaSyntax, MemberSyntax, NameExprSyntax, PostfixSuffix,
    PostfixUnaryOperationSyntax, PrefixUnaryOperationSyntax, ReturnSyntax, SubscriptSyntax,
    TypeCastSyntax, UnaryOperationSyntax,
};
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::stmt::Stmt;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;
use crate::syntax::Syntax;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, tag, take_until, take_while_m_n};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt, value};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{
    AsChar, Compare, ExtendInto, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::ops::{Range, RangeFrom};

pub fn integer_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: InputTake + ToString + InputLength + InputIter + Clone + InputTakeAtPosition,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(digit1, |n: I| {
        LiteralSyntax::Integer(TokenSyntax::new(n.to_string()))
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

pub fn raw_string_literal<I>(s: I) -> IResult<I, LiteralSyntax>
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
        permutation((char('r'), double_quote, take_until("\""), double_quote)),
        |(r, a, b, c): (char, char, I, char)| LiteralSyntax::String {
            open_quote: TokenSyntax::new(r.to_string() + &*a.to_string()),
            value: b.to_string(),
            close_quote: TokenSyntax::new(c.to_string()),
        },
    )(s)
}

pub fn string_literal<I>(s: I) -> IResult<I, LiteralSyntax>
where
    I: Clone
        + Offset
        + InputLength
        + InputTake
        + InputTakeAtPosition
        + Slice<RangeFrom<usize>>
        + InputIter
        + ToString
        + ExtendInto<Item = char, Extender = String>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            double_quote,
            escaped_transform(
                not_double_quote_or_back_slash,
                '\\',
                alt((
                    value('\\', char('\\')),
                    value('\"', char('\"')),
                    value('\'', char('\'')),
                    value('\r', char('r')),
                    value('\n', char('n')),
                    value('\t', char('t')),
                    map(
                        permutation((
                            char('u'),
                            take_while_m_n(4, 4, |c: <I as InputIter>::Item| c.is_hex_digit()),
                        )),
                        |(_, code): (char, I)| -> char {
                            decode_utf16(vec![u16::from_str_radix(&*code.to_string(), 16).unwrap()])
                                .next()
                                .unwrap()
                                .unwrap_or(REPLACEMENT_CHARACTER)
                        },
                    ),
                )),
            ),
            double_quote,
        )),
        |(a, s, b)| LiteralSyntax::String {
            open_quote: TokenSyntax::new(a.to_string()),
            value: s,
            close_quote: TokenSyntax::new(b.to_string()),
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
            trailing_trivia: Trivia::new(),
        })
    })(s)
}

pub fn literal_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Clone
        + Offset
        + InputLength
        + InputTake
        + InputTakeAtPosition
        + Slice<RangeFrom<usize>>
        + InputIter
        + ToString
        + ExtendInto<Item = char, Extender = String>
        + Compare<&'static str>
        + FindSubstring<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        alt((
            boolean_literal,
            floating_point_literal,
            integer_literal,
            string_literal,
            raw_string_literal,
        )),
        Expr::Literal,
    )(s)
}

pub fn name_space<I>(s: I) -> IResult<I, Vec<String>>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(many0(tuple((identifier, tag("::")))), |ns| {
        ns.into_iter().map(|(i, _)| i).collect()
    })(s)
}

pub fn name_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((name_space, identifier)), |(ns, name)| {
        Expr::Name(NameExprSyntax {
            name_space: ns,
            name,
        })
    })(s)
}

pub fn parenthesized_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((char('('), whitespace0, expr, whitespace0, char(')'))),
        |(_, _, expr, _, _)| expr,
    )(s)
}

pub fn return_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((return_keyword, whitespace1, opt(expr))),
        |(r, ws, e): (I, _, _)| {
            Expr::Return(ReturnSyntax {
                return_keyword: TokenSyntax::new(r.to_string()).with_trailing_trivia(ws),
                value: e.map(Box::new),
            })
        },
    )(s)
}

pub fn array_elements<I>(s: I) -> IResult<I, Vec<ArrayElementSyntax>>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        opt(tuple((
            expr,
            whitespace0,
            many0(tuple((comma, whitespace0, expr, whitespace0))),
            opt(comma),
        ))),
        |i| match i {
            None => {
                vec![]
            }
            Some((e, ws, v, c)) => {
                let mut cmas = vec![];
                let mut lwss = vec![ws];
                let mut expers = vec![e];
                let mut twss = vec![];
                for (cma, tws, e, lws) in v.into_iter() {
                    cmas.push(cma);
                    twss.push(tws);
                    expers.push(e);
                    lwss.push(lws);
                }
                match c {
                    None => {}
                    Some(c) => cmas.push(c),
                }
                let mut elements = vec![];
                for (idx, e) in expers.into_iter().enumerate() {
                    let mut trailing_comma = TokenSyntax::new(match cmas.get(idx) {
                        None => String::new(),
                        Some(c) => c.to_string(),
                    });
                    match lwss.get(idx) {
                        None => {}
                        Some(e) => {
                            trailing_comma = trailing_comma.with_leading_trivia(e.clone());
                        }
                    };
                    match twss.get(idx) {
                        None => {}
                        Some(e) => {
                            trailing_comma = trailing_comma.with_trailing_trivia(e.clone());
                        }
                    }
                    elements.push(ArrayElementSyntax {
                        element: e,
                        trailing_comma,
                    })
                }
                elements
            }
        },
    )(s)
}

pub fn array_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((tag("["), whitespace0, array_elements, whitespace0, tag("]"))),
        |(open, ows, elements, cws, close): (I, _, _, _, I)| {
            Expr::Array(ArraySyntax {
                open: TokenSyntax::new(open.to_string()).with_trailing_trivia(ows),
                values: elements,
                close: TokenSyntax::new(close.to_string()).with_leading_trivia(cws),
            })
        },
    )(s)
}

pub fn primary_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    alt((
        return_expr,
        if_expr,
        array_expr,
        name_expr,
        literal_expr,
        parenthesized_expr,
    ))(s)
}
/*
<if> ::= "if" <expr> <block> ("else" (<block> | <if>))?
*/
pub fn if_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
                        map(if_expr, |ib| BlockSyntax {
                            open: TokenSyntax::new("".to_string()),
                            body: vec![Stmt::Expr(ib)],
                            close: TokenSyntax::new("".to_string()),
                        }),
                    )),
                )),
                |(_, _, _, e)| e,
            )),
        )),
        |(_, _, condition, _, body, else_body)| {
            Expr::If(IfExprSyntax {
                condition: Box::new(condition),
                body,
                else_body,
            })
        },
    )(s)
}

/*
<postfix_expr> ::= <primary_expr> <postfix_suffix>*
*/
pub fn postfix_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(_postfix_expr, |(e, suffixes)| {
        let mut e = e;
        for suffix in suffixes {
            e = match suffix {
                // TODO: impl
                PostfixSuffix::Operator { kind } => {
                    Expr::UnaryOp(UnaryOperationSyntax::Postfix(PostfixUnaryOperationSyntax {
                        target: Box::new(e),
                        operator: TokenSyntax::new(kind),
                    }))
                }
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
                PostfixSuffix::NavigationSuffix { navigation, name } => {
                    Expr::Member(MemberSyntax {
                        target: Box::new(e),
                        name: TokenSyntax::new(name),
                        navigation_operator: TokenSyntax::new(navigation),
                    })
                }
            }
        }
        e
    })(s)
}

pub fn _postfix_expr<I>(s: I) -> IResult<I, (Expr, Vec<PostfixSuffix>)>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + Offset
        + InputLength
        + ToString
        + InputTake
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    tuple((primary_expr, many0(postfix_suffix)))(s)
}

/*
<postfix_suffix> ::= <postfix_operator>
                   | <type_arguments>
                   | <call_suffix>
                   | <indexing_suffix>
                   | <navigation_suffix>
*/
pub fn postfix_suffix<I>(s: I) -> IResult<I, PostfixSuffix>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    alt((
        map(postfix_operator, |s: I| PostfixSuffix::Operator {
            kind: s.to_string(),
        }),
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
pub fn navigation_suffix<I>(s: I) -> IResult<I, PostfixSuffix>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((member_access_operator, identifier)),
        |(op, name): (I, _)| PostfixSuffix::NavigationSuffix {
            navigation: op.to_string(),
            name,
        },
    )(s)
}

// <indexing_suffix> ::= "[" <expr> ("," <expr>)* ","? "]"
pub fn indexing_suffix<I>(s: I) -> IResult<I, PostfixSuffix>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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

pub fn prefix_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((opt(prefix_operator), postfix_expr)),
        |(op, postfix): (Option<I>, _)| match op {
            Some(op) => Expr::UnaryOp(UnaryOperationSyntax::Prefix(PrefixUnaryOperationSyntax {
                target: Box::new(postfix),
                operator: TokenSyntax::new(op.to_string()),
            })),
            None => postfix,
        },
    )(s)
}

fn _binop<T>(e: Expr, v: Vec<(Trivia, T, Trivia, Expr)>) -> Expr
where
    T: ToString,
{
    let mut bin_op = e;
    for (lws, op, rws, ex) in v {
        bin_op = Expr::BinOp(BinaryOperationSyntax {
            left: Box::new(bin_op),
            operator: TokenSyntax::new(op.to_string())
                .with_leading_trivia(lws)
                .with_trailing_trivia(rws),
            right: Box::new(ex),
        })
    }
    bin_op
}

/*
<conjunction_expr> ::= <equality_expr> ("&&" <equality_expr>)*
*/
pub fn conjunction_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
pub fn equality_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
<comparison_expr> ::= <generic_call_like_comparison_expr> (<comparison_operator> <generic_call_like_comparison_expr>)*
*/
pub fn comparison_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
<call_suffix> ::= <type_arguments>? ((<value_arguments>? <annotated_lambda>) | <value_arguments>)
*/
pub fn call_suffix<I>(s: I) -> IResult<I, PostfixSuffix>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
            args: args.unwrap_or_default(),
            tailing_lambda: tl,
        },
    )(s)
}
/*
<value_arguments> ::= "(" (<value_argument> ("," <value_argument>)* ","?)? ")"
*/
pub fn value_arguments<I>(s: I) -> IResult<I, Vec<CallArg>>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
            if let Some((a, ags, _)) = args_t {
                args = args
                    .into_iter()
                    .chain(vec![a])
                    .chain(ags.into_iter().map(|(_, ar)| ar))
                    .collect();
            };
            args
        },
    )(s)
}
/*
<value_argument> ::= (<identifier> ":")? "*"? <expr>
*/
pub fn value_argument<I>(s: I) -> IResult<I, CallArg>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
            is_vararg: is_vararg.is_some(),
        },
    )(s)
}
/*
<annotated_lambda> ::= <label>? <lambda_literal>
*/
pub fn annotated_lambda<I>(s: I) -> IResult<I, LambdaSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            opt(label), // TODO: label
            lambda_literal,
        )),
        |(l, lmd)| lmd,
    )(s)
}

pub fn lambda_literal<I>(s: I) -> IResult<I, LambdaSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(tuple((char('{'), stmts, char('}'))), |(_, stms, _)| {
        LambdaSyntax { stmts: stms }
    })(s)
}

pub fn label<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    // TODO: Impl
    char(' ')(s)
}

/*
<generic_call_like_comparison_expr> ::= <infix_operation_expr> <call_suffix>*
*/
pub fn generic_call_like_comparison_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
pub fn infix_operation_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
                    |(_, op, _, expr): (_, I, _, _)| P::IN {
                        op: op.to_string(),
                        expr,
                    },
                ),
                map(
                    tuple((whitespace1, is_operator, whitespace1, type_)),
                    |(_, op, _, type_): (_, I, _, _)| P::IS {
                        op: op.to_string(),
                        type_,
                    },
                ),
            ))),
        )),
        |(op, v)| {
            let mut bin_op = op;
            for p in v {
                match p {
                    P::IS { op, type_ } => {
                        bin_op = Expr::TypeCast(TypeCastSyntax {
                            target: Box::new(bin_op),
                            operator: op,
                            type_,
                        })
                    }
                    P::IN { op, expr } => {
                        bin_op = Expr::BinOp(BinaryOperationSyntax {
                            left: Box::new(bin_op),
                            operator: TokenSyntax::new(op),
                            right: Box::new(expr),
                        })
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
pub fn elvis_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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

/*
<infix_function_call_expr> ::= <range_expr> (<identifier> <range_expr>)*
*/
pub fn infix_function_call_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
pub fn range_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
<additive_expr> ::= <multiplicative_expr> (<additive_operator> <multiplicative_expr>)*
*/
pub fn additive_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
<multiplicative_expr> ::= <as_expr> (<multiplicative_operator> <as_expr>)*
*/
pub fn multiplicative_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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
<as_expr> ::= <prefix_expr> (<as_operator> <type>)*
*/
pub fn as_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        tuple((
            prefix_expr,
            many0(tuple((whitespace1, as_operator, whitespace1, type_))),
        )),
        |(e, v): (_, Vec<(_, I, _, _)>)| {
            let mut bin_op = e;
            for (_, op, _, typ) in v {
                bin_op = Expr::TypeCast(TypeCastSyntax {
                    target: Box::new(bin_op),
                    operator: op.to_string(),
                    type_: typ,
                })
            }
            bin_op
        },
    )(s)
}

pub fn disjunction_expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
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

pub fn expr<I>(s: I) -> IResult<I, Expr>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + Offset
        + InputTakeAtPosition
        + ExtendInto<Item = char, Extender = String>
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    disjunction_expr(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::expression::{
        array_expr, boolean_literal, conjunction_expr, disjunction_expr, equality_expr, expr,
        floating_point_literal, if_expr, indexing_suffix, integer_literal, literal_expr, name_expr,
        postfix_suffix, raw_string_literal, return_expr, string_literal, value_arguments,
    };
    use crate::syntax::block::BlockSyntax;
    use crate::syntax::decl::{Decl, VarSyntax};
    use crate::syntax::expr::{
        ArrayElementSyntax, ArraySyntax, BinaryOperationSyntax, CallArg, CallExprSyntax, Expr,
        IfExprSyntax, MemberSyntax, NameExprSyntax, PostfixSuffix, ReturnSyntax,
    };
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::stmt::Stmt;
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::{Trivia, TriviaPiece};
    use crate::syntax::Syntax;

    #[test]
    fn test_integer_literal() {
        assert_eq!(
            integer_literal("1"),
            Ok((
                "",
                LiteralSyntax::Integer(TokenSyntax::new("1".to_string()))
            ))
        );
        assert_eq!(
            integer_literal("12"),
            Ok((
                "",
                LiteralSyntax::Integer(TokenSyntax::new("12".to_string()))
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
    fn test_raw_string_literal() {
        assert_eq!(
            raw_string_literal("r\"\""),
            Ok((
                "",
                LiteralSyntax::String {
                    open_quote: TokenSyntax::new(r#"r""#.to_string()),
                    value: "".to_string(),
                    close_quote: TokenSyntax::new('"'.to_string())
                }
            ))
        );
        assert_eq!(
            raw_string_literal("r\"\\\\\""),
            Ok((
                "",
                LiteralSyntax::String {
                    open_quote: TokenSyntax::new(r#"r""#.to_string()),
                    value: "\\\\".to_string(),
                    close_quote: TokenSyntax::new('"'.to_string())
                }
            ))
        );
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            string_literal("\"s\\t\\ri\\ng\\\\\""),
            Ok((
                "",
                LiteralSyntax::String {
                    open_quote: TokenSyntax::new('"'.to_string()),
                    value: "s\t\ri\ng\\".to_string(),
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
    fn test_name_expr() {
        assert_eq!(
            name_expr("std::builtin::println"),
            Ok((
                "",
                Expr::Name(NameExprSyntax {
                    name_space: vec![String::from("std"), String::from("builtin")],
                    name: "println".to_string()
                })
            ))
        )
    }

    #[test]
    fn test_array_expr() {
        assert_eq!(
            array_expr("[a]"),
            Ok((
                "",
                Expr::Array(ArraySyntax {
                    open: TokenSyntax::new("[".to_string()),
                    values: vec![ArrayElementSyntax {
                        element: Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "a".to_string()
                        }),
                        trailing_comma: TokenSyntax::new("".to_string())
                    }],
                    close: TokenSyntax::new("]".to_string())
                })
            ))
        );
        assert_eq!(
            array_expr("[a, b]"),
            Ok((
                "",
                Expr::Array(ArraySyntax {
                    open: TokenSyntax::new("[".to_string()),
                    values: vec![
                        ArrayElementSyntax {
                            element: Expr::Name(NameExprSyntax {
                                name_space: vec![],
                                name: "a".to_string()
                            }),
                            trailing_comma: TokenSyntax::new(",".to_string())
                                .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        },
                        ArrayElementSyntax {
                            element: Expr::Name(NameExprSyntax {
                                name_space: vec![],
                                name: "b".to_string()
                            }),
                            trailing_comma: TokenSyntax::new("".to_string())
                        }
                    ],
                    close: TokenSyntax::new("]".to_string())
                })
            ))
        );
        assert_eq!(
            array_expr("[a,]"),
            Ok((
                "",
                Expr::Array(ArraySyntax {
                    open: TokenSyntax::new("[".to_string()),
                    values: vec![ArrayElementSyntax {
                        element: Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "a".to_string()
                        }),
                        trailing_comma: TokenSyntax::new(",".to_string())
                    }],
                    close: TokenSyntax::new("]".to_string())
                })
            ))
        );
        assert_eq!(
            array_expr("[a, b, ]"),
            Ok((
                "",
                Expr::Array(ArraySyntax {
                    open: TokenSyntax::new("[".to_string()),
                    values: vec![
                        ArrayElementSyntax {
                            element: Expr::Name(NameExprSyntax {
                                name_space: vec![],
                                name: "a".to_string()
                            }),
                            trailing_comma: TokenSyntax::new(",".to_string())
                                .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        },
                        ArrayElementSyntax {
                            element: Expr::Name(NameExprSyntax {
                                name_space: vec![],
                                name: "b".to_string()
                            }),
                            trailing_comma: TokenSyntax::new(",".to_string())
                        }
                    ],
                    close: TokenSyntax::new("]".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                })
            ))
        );
    }

    #[test]
    fn test_disjunction_expr() {
        assert_eq!(
            disjunction_expr("1||2 || 3"),
            Ok((
                "",
                Expr::BinOp(BinaryOperationSyntax {
                    left: Box::from(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "1".to_string()
                        )))),
                        operator: TokenSyntax::new("||".to_string()),
                        right: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "2".to_string()
                        ))))
                    })),
                    operator: TokenSyntax::new("||".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    right: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                        "3".to_string()
                    ))))
                })
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
                Expr::BinOp(BinaryOperationSyntax {
                    left: Box::from(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "1".to_string()
                        )))),
                        operator: TokenSyntax::new("&&".to_string())
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(vec![
                                TriviaPiece::Newlines(1),
                                TriviaPiece::Spaces(12)
                            ])),
                        right: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "2".to_string()
                        ))))
                    })),
                    operator: TokenSyntax::new("&&".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    right: Box::from(Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                        "3".to_string()
                    ))))
                })
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
                Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new("1".to_string())))
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
                        name_space: vec![],
                        name: "puts".to_string()
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
                        name_space: vec![],
                        name: "puts".to_string()
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
            expr(r#"puts(string: "Hello, World")"#),
            Ok((
                "",
                Expr::Call(CallExprSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "puts".to_string()
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
                Expr::If(IfExprSyntax {
                    condition: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "a".to_string()
                    })),
                    body: BlockSyntax {
                        open: TokenSyntax::new("{".to_string())
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        body: vec![],
                        close: TokenSyntax::new("}".to_string())
                    },
                    else_body: None
                })
            ))
        )
    }

    #[test]
    fn test_if_expr_with_else() {
        assert_eq!(
            if_expr(
                r"if capacity <= length {
            val newCapacity = if capacity == 0 { 4 } else { capacity * 2 }
        }"
            ),
            Ok((
                "",
                Expr::If(IfExprSyntax {
                    condition: Box::new(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "capacity".to_string()
                        })),
                        operator: TokenSyntax::new("<=".to_string())
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "length".to_string()
                        }))
                    })),
                    body: BlockSyntax {
                        open: TokenSyntax::new("{".to_string()).with_trailing_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(12)]
                        )),
                        body: vec![Stmt::Decl(Decl::Var(VarSyntax {
                            annotations: None,
                            is_mut: false,
                            name: "newCapacity".to_string(),
                            type_: None,
                            value: Expr::If(IfExprSyntax {
                                condition: Box::new(Expr::BinOp(BinaryOperationSyntax {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name_space: vec![],
                                        name: "capacity".to_string()
                                    })),
                                    operator: TokenSyntax::new("==".to_string())
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("0".to_string())
                                    )))
                                })),
                                body: BlockSyntax {
                                    open: TokenSyntax::new("{".to_string())
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    body: vec![Stmt::Expr(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("4".to_string())
                                    )))],
                                    close: TokenSyntax::new("}".to_string())
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                },
                                else_body: Some(BlockSyntax {
                                    open: TokenSyntax::new("{".to_string())
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    body: vec![Stmt::Expr(Expr::BinOp(BinaryOperationSyntax {
                                        left: Box::new(Expr::Name(NameExprSyntax {
                                            name_space: vec![],
                                            name: "capacity".to_string()
                                        })),
                                        operator: TokenSyntax::new("*".to_string())
                                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(
                                                1
                                            )))
                                            .with_trailing_trivia(Trivia::from(
                                                TriviaPiece::Spaces(1)
                                            )),
                                        right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                            TokenSyntax::new("2".to_string())
                                        )))
                                    }))],
                                    close: TokenSyntax::new("}".to_string())
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                })
                            })
                        }))],
                        close: TokenSyntax::new("}".to_string()).with_leading_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(8)]
                        ))
                    },
                    else_body: None
                })
            ))
        )
    }

    #[test]
    fn test_if_expr_with_else_empty() {
        assert_eq!(
            expr(r"if a { } else { }"),
            Ok((
                "",
                Expr::If(IfExprSyntax {
                    condition: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "a".to_string()
                    })),
                    body: BlockSyntax {
                        open: TokenSyntax::new("{".to_string())
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        body: vec![],
                        close: TokenSyntax::new("}".to_string())
                    },
                    else_body: Some(BlockSyntax {
                        open: TokenSyntax::new("{".to_string())
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        body: vec![],
                        close: TokenSyntax::new("}".to_string())
                    })
                })
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
                    return_keyword: TokenSyntax::new(String::from("return"))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    value: Some(Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
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
                Expr::Member(MemberSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "a".to_string()
                    })),
                    name: TokenSyntax::new("b".to_string()),
                    navigation_operator: TokenSyntax::new(".".to_string())
                })
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
                        name_space: vec![],
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
                            name_space: vec![],
                            name: "a".to_string()
                        }),
                        Expr::Name(NameExprSyntax {
                            name_space: vec![],
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
                            name_space: vec![],
                            name: "a".to_string()
                        }),
                        Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "b".to_string()
                        }),
                    ]
                }
            ))
        );
    }
}
