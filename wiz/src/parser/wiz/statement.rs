use crate::parser::wiz::declaration::{block, decl};
use crate::parser::wiz::expression::{expr, postfix_expr, prefix_expr};
use crate::parser::wiz::keywords::while_keyword;
use crate::parser::wiz::lexical_structure::{identifier, whitespace0, whitespace1};
use crate::parser::wiz::operators::{assignment_and_operator, assignment_operator};
use crate::syntax::expr::{Expr, NameExprSyntax};
use crate::syntax::file::FileSyntax;
use crate::syntax::stmt::{
    AssignmentAndOperatorSyntax, AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt,
};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err::Error;
use nom::{
    error, AsChar, Compare, CompareResult, ExtendInto, FindSubstring, IResult, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
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
    map(decl, |d| Stmt::Decl(d))(s)
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
    map(expr, |e| Stmt::Expr(e))(s)
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
        |((target, _, op), _, value): ((_, _, I), _, _)| {
            let r = op.compare("");
            match r {
                CompareResult::Ok => {
                    Stmt::Assignment(AssignmentStmt::Assignment(AssignmentSyntax {
                        target,
                        value,
                    }))
                }
                CompareResult::Incomplete => Stmt::Assignment(
                    AssignmentStmt::AssignmentAndOperator(AssignmentAndOperatorSyntax {
                        target,
                        operator: op.to_string(),
                        value,
                    }),
                ),
                CompareResult::Error => {
                    panic!()
                }
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
                name_space: vec![],
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
    map(alt((while_stmt, while_stmt)), |l| Stmt::Loop(l))(s)
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
        |(_, _, e, _, b)| LoopStmt::While {
            condition: e,
            block: b,
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
    use crate::syntax::block::Block;
    use crate::syntax::expr::{Expr, NameExprSyntax};
    use crate::syntax::file::FileSyntax;
    use crate::syntax::literal::LiteralSyntax;
    use crate::syntax::stmt::{AssignmentStmt, AssignmentSyntax, LoopStmt, Stmt};
    use crate::syntax::token::TokenSyntax;

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
                LoopStmt::While {
                    condition: Expr::BinOp {
                        left: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "a".to_string()
                        })),
                        kind: "<".to_string(),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "b".to_string()
                        }))
                    },
                    block: Block {
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name_space: vec![],
                                    name: "a".to_string()
                                }),
                                value: Expr::BinOp {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name_space: vec![],
                                        name: "a".to_string()
                                    })),
                                    kind: "+".to_string(),
                                    right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("1".to_string())
                                    )))
                                }
                            }
                        ))]
                    }
                }
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
                LoopStmt::While {
                    condition: Expr::BinOp {
                        left: Box::new(Expr::Member {
                            target: Box::new(Expr::Name(NameExprSyntax {
                                name_space: vec![],
                                name: "a".to_string()
                            })),
                            name: "c".to_string(),
                            navigation_operator: ".".to_string()
                        }),
                        kind: "<".to_string(),
                        right: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "b".to_string()
                        }))
                    },
                    block: Block {
                        body: vec![Stmt::Assignment(AssignmentStmt::Assignment(
                            AssignmentSyntax {
                                target: Expr::Name(NameExprSyntax {
                                    name_space: vec![],
                                    name: "a".to_string()
                                }),
                                value: Expr::BinOp {
                                    left: Box::new(Expr::Name(NameExprSyntax {
                                        name_space: vec![],
                                        name: "a".to_string()
                                    })),
                                    kind: "+".to_string(),
                                    right: Box::new(Expr::Literal(LiteralSyntax::Integer(
                                        TokenSyntax::new("1".to_string())
                                    )))
                                }
                            }
                        ))]
                    }
                }
            ))
        )
    }

    #[test]
    fn test_directly_assignable_expr() {
        assert_eq!(
            directly_assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "a".to_string()
                    })),
                    name: "b".to_string(),
                    navigation_operator: ".".to_string()
                }
            ))
        )
    }

    #[test]
    fn test_assignable_expr() {
        assert_eq!(
            assignable_expr("a.b"),
            Ok((
                "",
                Expr::Member {
                    target: Box::new(Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "a".to_string()
                    })),
                    name: "b".to_string(),
                    navigation_operator: ".".to_string()
                }
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
                        name_space: vec![],
                        name: "a".to_string()
                    }),
                    value: Expr::Name(NameExprSyntax {
                        name_space: vec![],
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
                        name_space: vec![],
                        name: "a".to_string()
                    }),
                    value: Expr::Member {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "b".to_string()
                        })),
                        name: "c".to_string(),
                        navigation_operator: ".".to_string()
                    }
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
                    target: Expr::Member {
                        target: Box::new(Expr::Name(NameExprSyntax {
                            name_space: vec![],
                            name: "a".to_string()
                        })),
                        name: "b".to_string(),
                        navigation_operator: ".".to_string()
                    },
                    value: Expr::Name(NameExprSyntax {
                        name_space: vec![],
                        name: "c".to_string()
                    }),
                }))
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
