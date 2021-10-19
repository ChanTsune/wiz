use crate::parser::wiz::declaration::{block, decl};
use crate::parser::wiz::expression::{expr, postfix_expr, prefix_expr};
use crate::parser::wiz::keywords::{for_keyword, in_keyword, while_keyword};
use crate::parser::wiz::lexical_structure::{identifier, whitespace0, whitespace1};
use crate::parser::wiz::operators::{assignment_and_operator, assignment_operator};
use crate::syntax::expr::{Expr, NameExprSyntax};
use crate::syntax::file::FileSyntax;
use crate::syntax::stmt::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt, WhileLoopSyntax,
};
use crate::syntax::token::TokenSyntax;
use crate::syntax::Syntax;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{
    error, AsChar, Compare, ExtendInto, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, Slice,
};
use std::ops::{Range, RangeFrom};

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
                    operator: TokenSyntax::new(op.to_string())
                        .with_leading_trivia(ws)
                        .with_trailing_trivia(ews),
                    value,
                })),
                _ => Stmt::Assignment(AssignmentStmt::AssignmentAndOperator(
                    AssignmentAndOperatorSyntax {
                        target,
                        operator: TokenSyntax::new(op.to_string())
                            .with_leading_trivia(ws)
                            .with_trailing_trivia(ews),
                        value,
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
            Expr::Name(NameExprSyntax {
                name_space: Default::default(),
                name,
            })
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
        Expr::Member { .. } => IResult::Ok((e, expr)),
        Expr::Subscript { .. } => IResult::Ok((e, expr)),
        _ => IResult::Err(Error(error::Error::new(e, error::ErrorKind::Alt))),
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
        tuple((char('('), assignable_expr, char(')'))),
        |(_, e, _)| e,
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
        tuple((char('('), directly_assignable_expr, char(')'))),
        |(_, e, _)| e,
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
        |(_, _, e, _, b)| {
            LoopStmt::While(WhileLoopSyntax {
                condition: e,
                block: b,
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
        |(_, _, value, _, _, _, iterator, _, block)| LoopStmt::For {
            iterator,
            values: vec![value],
            block,
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
    map(
        tuple((
            whitespace0,
            alt((decl_stmt, assignment_stmt, loop_stmt, expr_stmt)),
        )),
        |(ws, stm)| stm,
    )(s)
}

pub fn stmts<I>(s: I) -> IResult<I, Vec<Stmt>>
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
    many0(stmt)(s)
}

pub fn file<I>(s: I) -> IResult<I, FileSyntax>
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
        tuple((whitespace0, many0(tuple((whitespace0, decl))), whitespace0)),
        |(_, decls, _)| FileSyntax {
            body: decls.into_iter().map(|(_, f)| f).collect(),
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::statement::{
        assignable_expr, assignment_stmt, directly_assignable_expr, file, while_stmt,
    };
    use crate::syntax::block::BlockSyntax;
    use crate::syntax::expr::{BinaryOperationSyntax, Expr, MemberSyntax, NameExprSyntax};
    use crate::syntax::file::FileSyntax;
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::stmt::{
        AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt,
        WhileLoopSyntax,
    };
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::{Trivia, TriviaPiece};
    use crate::syntax::Syntax;

    #[test]
    fn test_while_stmt_with_bracket() {
        assert_eq!(
            while_stmt(
                r"while (a < b) {
            a = a + 1
        }"
            ),
            Ok((
                "",
                LoopStmt::While(WhileLoopSyntax {
                    condition: Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "a".to_string()
                        })),
                        operator: TokenSyntax::new("<".to_string())
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "b".to_string()
                        }))
                    }),
                    block: BlockSyntax {
                        open: TokenSyntax::new("{".to_string()).with_trailing_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(12)]
                        )),
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name_space: Default::default(),
                                    name: "a".to_string()
                                }),
                                operator: TokenSyntax::new("=".to_string())
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                    .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                value: Expr::BinOp(BinaryOperationSyntax {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name_space: Default::default(),
                                        name: "a".to_string()
                                    })),
                                    operator: TokenSyntax::new("+".to_string())
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("1".to_string())
                                    )))
                                })
                            }
                        ))],
                        close: TokenSyntax::new("}".to_string()).with_leading_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(8)]
                        ))
                    }
                })
            ))
        )
    }

    #[test]
    fn test_while_stmt() {
        assert_eq!(
            while_stmt(
                r"while a.c < b {
            a = a + 1
        }"
            ),
            Ok((
                "",
                LoopStmt::While(WhileLoopSyntax {
                    condition: Expr::BinOp(BinaryOperationSyntax {
                        left: Box::new(Expr::Member(MemberSyntax {
                            target: Box::new(Expr::Name(NameExprSyntax {
                                name_space: Default::default(),
                                name: "a".to_string()
                            })),
                            name: TokenSyntax::new("c".to_string()),
                            navigation_operator: TokenSyntax::new(".".to_string())
                        })),
                        operator: TokenSyntax::new("<".to_string())
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "b".to_string()
                        }))
                    }),
                    block: BlockSyntax {
                        open: TokenSyntax::new("{".to_string()).with_trailing_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(12)]
                        )),
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name_space: Default::default(),
                                    name: "a".to_string()
                                }),
                                operator: TokenSyntax::new("=".to_string())
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                    .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                value: Expr::BinOp(BinaryOperationSyntax {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name_space: Default::default(),
                                        name: "a".to_string()
                                    })),
                                    operator: TokenSyntax::new("+".to_string())
                                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("1".to_string())
                                    )))
                                })
                            }
                        ))],
                        close: TokenSyntax::new("}".to_string()).with_leading_trivia(Trivia::from(
                            vec![TriviaPiece::Newlines(1), TriviaPiece::Spaces(8)]
                        ))
                    }
                })
            ))
        )
    }

    #[test]
    fn test_directly_assignable_expr() {
        assert_eq!(
            directly_assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member(MemberSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "a".to_string()
                    })),
                    name: TokenSyntax::new("b".to_string()),
                    navigation_operator: TokenSyntax::new(".".to_string())
                })
            ))
        )
    }

    #[test]
    fn test_assignable_expr() {
        assert_eq!(
            assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member(MemberSyntax {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "a".to_string()
                    })),
                    name: TokenSyntax::new("b".to_string()),
                    navigation_operator: TokenSyntax::new(".".to_string())
                })
            ))
        )
    }

    #[test]
    fn test_assignment() {
        assert_eq!(
            assignment_stmt("a = b"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "a".to_string()
                    }),
                    operator: TokenSyntax::new("=".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    value: Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "b".to_string()
                    })
                }))
            ))
        )
    }

    #[test]
    fn test_assignment_struct_field() {
        assert_eq!(
            assignment_stmt("a = b.c"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "a".to_string()
                    }),
                    operator: TokenSyntax::new("=".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    value: Expr::Member(MemberSyntax {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "b".to_string()
                        })),
                        name: TokenSyntax::new("c".to_string()),
                        navigation_operator: TokenSyntax::new(".".to_string())
                    })
                }))
            ))
        )
    }

    #[test]
    fn test_assignment_to_struct_field() {
        assert_eq!(
            assignment_stmt("a.b = c"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                    target: Expr::Member(MemberSyntax {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "a".to_string()
                        })),
                        name: TokenSyntax::new("b".to_string()),
                        navigation_operator: TokenSyntax::new(".".to_string())
                    }),
                    operator: TokenSyntax::new("=".to_string())
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    value: Expr::Name(NameExprSyntax {
                        name_space: Default::default(),
                        name: "c".to_string()
                    }),
                }))
            ))
        )
    }

    #[test]
    fn test_assignment_and_operation() {
        assert_eq!(
            assignment_stmt("a += 1"),
            Ok((
                "",
                Stmt::Assignment(AssignmentStmt::AssignmentAndOperator(
                    AssignmentAndOperatorSyntax {
                        target: Expr::Name(NameExprSyntax {
                            name_space: Default::default(),
                            name: "a".to_string()
                        }),
                        operator: TokenSyntax::new("+=".to_string())
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        value: Expr::Literal(LiteralSyntax::Integer(TokenSyntax::new(
                            "1".to_string()
                        )))
                    }
                ))
            ))
        )
    }

    #[test]
    fn test_file_empty() {
        assert_eq!(file(""), Ok(("", FileSyntax { body: vec![] })));
        assert_eq!(file("\n"), Ok(("", FileSyntax { body: vec![] })));
        assert_eq!(file(" "), Ok(("", FileSyntax { body: vec![] })));
    }
}
