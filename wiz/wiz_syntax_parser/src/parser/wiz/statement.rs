use crate::parser::wiz::declaration::{block, decl};
use crate::parser::wiz::expression::{expr, postfix_expr, prefix_expr};
use crate::parser::wiz::keywords::{for_keyword, in_keyword, while_keyword};
use crate::parser::wiz::lexical_structure::{identifier, token, whitespace0, whitespace1};
use crate::parser::wiz::operators::{assignment_and_operator, assignment_operator};
use crate::parser::Span;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{
    error, AsChar, Compare, ExtendInto, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use std::ops::{Range, RangeFrom};
use wiz_session::ParseSession;
use wiz_syntax::syntax::expression::{Expr, NameExprSyntax, ParenthesizedExprSyntax};
use wiz_syntax::syntax::statement::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, ForLoopSyntax, LoopStmt, Stmt,
    WhileLoopSyntax,
};
use wiz_syntax::syntax::token::TokenSyntax;
use wiz_syntax::syntax::FileSyntax;
use wiz_syntax::syntax::Syntax;

pub fn decl_stmt<I>(s: I) -> IResult<I, Stmt>
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
    map(decl, Stmt::Decl)(s)
}

pub fn expr_stmt<I>(s: I) -> IResult<I, Stmt>
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
    map(expr, Stmt::Expr)(s)
}

/*
<assignment_stmt> ::= ((<directly_assignable_expr> '=') | (<assignable_expr> <assignment_and_operator>)) <expr>
*/
pub fn assignment_stmt<I>(s: I) -> IResult<I, Stmt>
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
            alt((
                tuple((directly_assignable_expr, whitespace0, assignment_operator)),
                tuple((assignable_expr, whitespace0, assignment_and_operator)),
            )),
            whitespace0,
            expr,
        )),
        |((target, ws, op), ews, value): ((_, _, I), _, _)| {
            let op = op.to_string();
            match &*op {
                "=" => Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target,
                    operator: TokenSyntax::from(op).with_leading_trivia(ws),
                    value: value.with_leading_trivia(ews),
                })),
                _ => Stmt::Assignment(AssignmentStmt::AssignmentAndOperator(
                    AssignmentAndOperatorSyntax {
                        target,
                        operator: TokenSyntax::from(op).with_leading_trivia(ws),
                        value: value.with_leading_trivia(ews),
                    },
                )),
            }
        },
    )(s)
}
/*
<directly_assignable_expr> ::= <postfix_expr> <assignable_suffix>
                             | <identifier>
                             | <parenthesized_directly_assignable_expr>
*/
pub fn directly_assignable_expr<I>(s: I) -> IResult<I, Expr>
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
        _directly_assignable_postfix_expr,
        map(identifier, |name| {
            Expr::Name(NameExprSyntax::simple(TokenSyntax::from(name)))
        }),
        map(parenthesized_directly_assignable_expr, |e| e),
    ))(s)
}

/*
<assignable_suffix> ::= <type_arguments>
  | <indexing_suffix>
  | <navigation_suffix>
*/
fn _directly_assignable_postfix_expr<I>(s: I) -> IResult<I, Expr>
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
    let (e, expr) = postfix_expr(s)?;
    match expr {
        Expr::Member { .. } => Ok((e, expr)),
        Expr::Subscript { .. } => Ok((e, expr)),
        _ => Err(Error(error::Error::new(e, error::ErrorKind::Alt))),
    }
}

/*
<assignable_expr> ::= <prefix_expr>
  | <parenthesized_assignable_expression>
*/
pub fn assignable_expr<I>(s: I) -> IResult<I, Expr>
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
    alt((prefix_expr, parenthesized_assignable_expression))(s)
}
/*
<parenthesized_assignable_expression> ::= "(" <assignable_expr> ")"
*/
pub fn parenthesized_assignable_expression<I>(s: I) -> IResult<I, Expr>
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
            token("("),
            whitespace0,
            assignable_expr,
            whitespace0,
            token(")"),
        )),
        |(open_paren, ows, e, cws, close_paren)| {
            Expr::Parenthesized(ParenthesizedExprSyntax {
                open_paren,
                expr: Box::new(e.with_leading_trivia(ows)),
                close_paren: close_paren.with_leading_trivia(cws),
            })
        },
    )(s)
}

/*
<parenthesized_directly_assignable_expr> ::= '(' <directly_assignable_expr> ')'
*/
pub fn parenthesized_directly_assignable_expr<I>(s: I) -> IResult<I, Expr>
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
            token("("),
            whitespace0,
            directly_assignable_expr,
            whitespace0,
            token(")"),
        )),
        |(open_paren, ows, e, cws, close_paren)| {
            Expr::Parenthesized(ParenthesizedExprSyntax {
                open_paren,
                expr: Box::new(e.with_leading_trivia(ows)),
                close_paren: close_paren.with_leading_trivia(cws),
            })
        },
    )(s)
}

pub fn loop_stmt<I>(s: I) -> IResult<I, Stmt>
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
    map(alt((for_stmt, while_stmt)), Stmt::Loop)(s)
}

pub fn while_stmt<I>(s: I) -> IResult<I, LoopStmt>
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
        tuple((while_keyword, whitespace1, expr, whitespace1, block)),
        |(w, ws, e, bws, b)| {
            LoopStmt::While(WhileLoopSyntax {
                while_keyword: w,
                condition: e.with_leading_trivia(ws),
                block: b.with_leading_trivia(bws),
            })
        },
    )(s)
}

pub fn for_stmt<I>(s: I) -> IResult<I, LoopStmt>
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
            for_keyword,
            whitespace1,
            identifier,
            whitespace1,
            in_keyword,
            whitespace1,
            expr,
            whitespace1,
            block,
        )),
        |(for_keyword, w, value, iw, in_keyword, itw, iterator, bws, block)| {
            LoopStmt::For(ForLoopSyntax {
                for_keyword: TokenSyntax::from(for_keyword),
                values: vec![TokenSyntax::from(value).with_leading_trivia(w)],
                in_keyword: TokenSyntax::from(in_keyword).with_leading_trivia(iw),
                iterator: iterator.with_leading_trivia(itw),
                block: block.with_leading_trivia(bws),
            })
        },
    )(s)
}

pub fn stmt<I>(s: I) -> IResult<I, Stmt>
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
    alt((decl_stmt, assignment_stmt, loop_stmt, expr_stmt))(s)
}

pub fn file<'sess, 's>(session: &'sess ParseSession, s: Span<'s>) -> IResult<Span<'s>, FileSyntax> {
    let (s, leading_trivia) = whitespace0(s)?;
    let (s, decls): (Span, _) = many0(tuple((whitespace0, decl)))(s)?;
    let (s, trailing_trivia) = whitespace0(s)?;
    Ok((
        s,
        FileSyntax {
            leading_trivia,
            body: decls
                .into_iter()
                .map(|(t, f)| f.with_leading_trivia(t))
                .collect(),
            trailing_trivia,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::{check, check_with_session};
    use crate::parser::wiz::statement::{
        assignable_expr, assignment_stmt, directly_assignable_expr, file, stmt, while_stmt,
    };
    use wiz_syntax::syntax::block::BlockSyntax;
    use wiz_syntax::syntax::expression::{
        BinaryOperationSyntax, CallArgListSyntax, CallExprSyntax, Expr, MemberSyntax,
        NameExprSyntax, ParenthesizedExprSyntax,
    };
    use wiz_syntax::syntax::literal::LiteralSyntax;
    use wiz_syntax::syntax::statement::{
        AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt,
        WhileLoopSyntax,
    };
    use wiz_syntax::syntax::token::TokenSyntax;
    use wiz_syntax::syntax::trivia::{Trivia, TriviaPiece};
    use wiz_syntax::syntax::FileSyntax;
    use wiz_syntax::syntax::Syntax;

    #[test]
    fn test_call_expr_stmt() {
        check(
            "hoge()",
            stmt,
            Stmt::Expr(Expr::Call(CallExprSyntax {
                target: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from(
                    "hoge",
                )))),
                args: Some(CallArgListSyntax::new()),
                tailing_lambda: None,
            })),
        )
    }

    #[test]
    fn test_while_stmt_with_bracket() {
        check(
            r"while (a < b) {
            a = a + 1
        }",
            while_stmt,
            LoopStmt::While(WhileLoopSyntax {
                while_keyword: TokenSyntax::from("while"),
                condition: Expr::Parenthesized(ParenthesizedExprSyntax {
                    open_paren: TokenSyntax::from("(")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    expr: Box::new(Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a")))),
                        operator: TokenSyntax::from("<")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        right: Box::new(
                            Expr::Name(NameExprSyntax::simple(TokenSyntax::from("b")))
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        ),
                    })),
                    close_paren: TokenSyntax::from(")"),
                }),
                block: BlockSyntax {
                    open: TokenSyntax::from("{")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    body: vec![
                        Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                            target: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a"))),
                            operator: TokenSyntax::from("=")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            value: Expr::BinOp(BinaryOperationSyntax {
                                left: Box::new(Expr::Name(NameExprSyntax::simple(
                                    TokenSyntax::from("a"),
                                ))),
                                operator: TokenSyntax::from("+")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                right: Box::new(
                                    Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                ),
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        }))
                        .with_leading_trivia(Trivia::from(vec![
                            TriviaPiece::Newlines(1),
                            TriviaPiece::Spaces(12),
                        ])),
                    ],
                    close: TokenSyntax::from("}").with_leading_trivia(Trivia::from(vec![
                        TriviaPiece::Newlines(1),
                        TriviaPiece::Spaces(8),
                    ])),
                },
            }),
        )
    }

    #[test]
    fn test_while_stmt() {
        check(
            r"while a.c < b {
            a = a + 1
        }",
            while_stmt,
            LoopStmt::While(WhileLoopSyntax {
                while_keyword: TokenSyntax::from("while"),
                condition: Expr::BinOp(BinaryOperationSyntax {
                    left: Box::new(
                        Expr::Member(MemberSyntax {
                            target: Box::new(Expr::Name(NameExprSyntax::simple(
                                TokenSyntax::from("a"),
                            ))),
                            name: TokenSyntax::from("c"),
                            navigation_operator: TokenSyntax::from("."),
                        })
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    ),
                    operator: TokenSyntax::from("<")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    right: Box::new(
                        Expr::Name(NameExprSyntax::simple(TokenSyntax::from("b")))
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    ),
                }),
                block: BlockSyntax {
                    open: TokenSyntax::from("{")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    body: vec![
                        Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                            target: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a"))),
                            operator: TokenSyntax::from("=")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            value: Expr::BinOp(BinaryOperationSyntax {
                                left: Box::new(Expr::Name(NameExprSyntax::simple(
                                    TokenSyntax::from("a"),
                                ))),
                                operator: TokenSyntax::from("+")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                right: Box::new(
                                    Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                ),
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        }))
                        .with_leading_trivia(Trivia::from(vec![
                            TriviaPiece::Newlines(1),
                            TriviaPiece::Spaces(12),
                        ])),
                    ],
                    close: TokenSyntax::from("}").with_leading_trivia(Trivia::from(vec![
                        TriviaPiece::Newlines(1),
                        TriviaPiece::Spaces(8),
                    ])),
                },
            }),
        )
    }

    #[test]
    fn test_directly_assignable_expr() {
        check(
            "a.b",
            directly_assignable_expr,
            Expr::Member(MemberSyntax {
                target: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a")))),
                name: TokenSyntax::from("b"),
                navigation_operator: TokenSyntax::from("."),
            }),
        )
    }

    #[test]
    fn test_assignable_expr() {
        check(
            "a.b",
            assignable_expr,
            Expr::Member(MemberSyntax {
                target: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a")))),
                name: TokenSyntax::from("b"),
                navigation_operator: TokenSyntax::from("."),
            }),
        )
    }

    #[test]
    fn test_assignment() {
        check(
            "a = b",
            assignment_stmt,
            Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                target: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a"))),
                operator: TokenSyntax::from("=")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                value: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("b")))
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            })),
        )
    }

    #[test]
    fn test_assignment_struct_field() {
        check(
            "a = b.c",
            assignment_stmt,
            Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                target: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a"))),
                operator: TokenSyntax::from("=")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                value: Expr::Member(MemberSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from("b")))),
                    name: TokenSyntax::from("c"),
                    navigation_operator: TokenSyntax::from("."),
                })
                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            })),
        )
    }

    #[test]
    fn test_assignment_to_struct_field() {
        check(
            "a.b = c",
            assignment_stmt,
            Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                target: Expr::Member(MemberSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a")))),
                    name: TokenSyntax::from("b"),
                    navigation_operator: TokenSyntax::from("."),
                }),
                operator: TokenSyntax::from("=")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                value: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("c")))
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            })),
        )
    }

    #[test]
    fn test_assignment_and_operation() {
        check(
            "a += 1",
            assignment_stmt,
            Stmt::Assignment(AssignmentStmt::AssignmentAndOperator(
                AssignmentAndOperatorSyntax {
                    target: Expr::Name(NameExprSyntax::simple(TokenSyntax::from("a"))),
                    operator: TokenSyntax::from("+=")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::from("1")))
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                },
            )),
        )
    }

    #[test]
    fn test_file_empty() {
        check_with_session(
            "",
            file,
            FileSyntax {
                leading_trivia: Default::default(),
                body: vec![],
                trailing_trivia: Default::default(),
            },
        );
        check_with_session(
            "\n",
            file,
            FileSyntax {
                leading_trivia: Trivia::from(TriviaPiece::Newlines(1)),
                body: vec![],
                trailing_trivia: Default::default(),
            },
        );
        check_with_session(
            " ",
            file,
            FileSyntax {
                leading_trivia: Trivia::from(TriviaPiece::Spaces(1)),
                body: vec![],
                trailing_trivia: Default::default(),
            },
        );
    }
}
