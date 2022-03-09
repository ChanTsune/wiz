use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::lexical_structure::{identifier, token, whitespace0};
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{
    ArrayTypeSyntax, DecoratedTypeName, ParenthesizedTypeName, SimpleTypeName,
    TypeArgumentElementSyntax, TypeArgumentListSyntax, TypeConstraintSyntax, TypeName,
    TypeNameSpaceElementSyntax, TypeParam, TypeParameterElementSyntax, TypeParameterListSyntax,
    UserTypeName,
};
use crate::syntax::Syntax;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::{AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::{Range, RangeFrom};

pub fn type_<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((
        parenthesized_type,
        map(decorated_type, |t| TypeName::Decorated(Box::new(t))),
        type_reference,
        // function_type,
    ))(s)
}

pub fn parenthesized_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((token("("), whitespace0, type_, whitespace0, token(")"))),
        |(open_paren, ows, type_, cws, close_paren)| {
            TypeName::Parenthesized(ParenthesizedTypeName {
                open_paren,
                type_name: Box::new(type_.with_leading_trivia(ows)),
                close_paren: close_paren.with_trailing_trivia(cws),
            })
        },
    )(s)
}

pub fn decorated_type<I>(s: I) -> IResult<I, DecoratedTypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            alt((token("*"), ampersand)),
            alt((type_reference, parenthesized_type)),
        )),
        |(p, type_name)| DecoratedTypeName {
            decoration: p,
            type_: type_name,
        },
    )(s)
}

pub fn type_reference<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    user_type(s)
}

pub fn user_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            many0(tuple((simple_user_type, token("::")))),
            simple_user_type,
        )),
        |(name_space, type_name)| {
            if name_space.is_empty() {
                TypeName::Simple(type_name)
            } else {
                TypeName::NameSpaced(Box::new(UserTypeName {
                    name_space: name_space
                        .into_iter()
                        .map(|(simple_type, sep)| TypeNameSpaceElementSyntax { simple_type, sep })
                        .collect(),
                    type_name,
                }))
            }
        },
    )(s)
}

pub fn simple_user_type<I>(s: I) -> IResult<I, SimpleTypeName>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(tuple((identifier, opt(type_arguments))), |(name, args)| {
        SimpleTypeName {
            name: TokenSyntax::from(name),
            type_args: args,
        }
    })(s)
}

// pub fn function_type(s: &str) -> IResult<&str, TypeName> {
//
// }

pub fn type_arguments<I>(s: I) -> IResult<I, TypeArgumentListSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            token("<"),
            many0(tuple((whitespace0, type_, whitespace0, comma))),
            opt(tuple((whitespace0, type_))),
            whitespace0,
            token(">"),
        )),
        |(open, t, typ, tws, close)| {
            let mut elements: Vec<_> = t
                .into_iter()
                .map(|(lws, tp, rws, com)| TypeArgumentElementSyntax {
                    element: tp.with_leading_trivia(lws),
                    trailing_comma: Some(com.with_leading_trivia(rws)),
                })
                .collect();
            if let Some((ws, p)) = typ {
                elements.push(TypeArgumentElementSyntax {
                    element: p.with_leading_trivia(ws),
                    trailing_comma: None,
                });
            };
            TypeArgumentListSyntax {
                open,
                elements,
                close: close.with_leading_trivia(tws),
            }
        },
    )(s)
}

// <type_parameters> ::= "<" <type_parameter> ("," <type_parameter>)* ","? ">"
pub fn type_parameters<I>(s: I) -> IResult<I, TypeParameterListSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>
        + FindSubstring<&'static str>
        + ToString
        + Slice<Range<usize>>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            token("<"),
            many0(tuple((whitespace0, type_parameter, whitespace0, comma))),
            opt(tuple((whitespace0, type_parameter))),
            whitespace0,
            token(">"),
        )),
        |(open, params, param, tws, close)| {
            let mut elements: Vec<_> = params
                .into_iter()
                .map(|(lws, tp, rws, com)| TypeParameterElementSyntax {
                    element: tp.with_leading_trivia(lws),
                    trailing_comma: Some(com.with_leading_trivia(rws)),
                })
                .collect();
            if let Some((ws, p)) = param {
                elements.push(TypeParameterElementSyntax {
                    element: p.with_leading_trivia(ws),
                    trailing_comma: None,
                });
            };
            TypeParameterListSyntax {
                open,
                elements,
                close: close.with_leading_trivia(tws),
            }
        },
    )(s)
}

// <type_parameter> ::= <identifier> <type_constraint>?
pub fn type_parameter<I>(s: I) -> IResult<I, TypeParam>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>
        + FindSubstring<&'static str>
        + ToString
        + Slice<Range<usize>>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            identifier,
            opt(tuple((whitespace0, type_constraint_syntax))),
        )),
        |(name, typ)| TypeParam {
            name: TokenSyntax::from(name),
            type_constraint: typ.map(|(ws, t)| t.with_leading_trivia(ws)),
        },
    )(s)
}

// <type_constraint> ::= ":" <type>
pub fn type_constraint_syntax<I>(s: I) -> IResult<I, TypeConstraintSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + Compare<&'static str>
        + FindSubstring<&'static str>
        + ToString
        + Slice<Range<usize>>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(tuple((token(":"), whitespace0, type_)), |(sep, lws, t)| {
        TypeConstraintSyntax {
            sep,
            constraint: t.with_leading_trivia(lws),
        }
    })(s)
}

pub fn array_type_syntax<I>(s: I) -> IResult<I, ArrayTypeSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            token("["),
            whitespace0,
            type_,
            whitespace0,
            token(";"),
            whitespace0,
            many1(alt((char('0'),char('1'),char('2'),char('3'),char('4'),char('5'),char('6'),char('7'),char('8'),char('9')))),
            whitespace0,
            token("]"),
        )),
        |(open, ws1, typ, ws2, semi, ws3, size, ws4, close)| ArrayTypeSyntax {
            open,
            type_: typ.with_leading_trivia(ws1),
            semicolon: semi.with_leading_trivia(ws2),
            size: TokenSyntax::from(size.into_iter().collect::<String>()).with_leading_trivia(ws3),
            close: close.with_leading_trivia(ws4),
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::check;
    use crate::parser::wiz::type_::{array_type_syntax, decorated_type, type_parameter, type_parameters, user_type};
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::{Trivia, TriviaPiece};
    use crate::syntax::type_name::{ArrayTypeSyntax, DecoratedTypeName, SimpleTypeName, TypeConstraintSyntax, TypeName, TypeNameSpaceElementSyntax, TypeParam, TypeParameterElementSyntax, TypeParameterListSyntax, UserTypeName};
    use crate::syntax::Syntax;

    #[test]
    fn test_name_spaced_type() {
        assert_eq!(
            user_type("std::builtin::String"),
            Ok((
                "",
                TypeName::NameSpaced(Box::new(UserTypeName {
                    name_space: vec![
                        TypeNameSpaceElementSyntax::from("std"),
                        TypeNameSpaceElementSyntax::from("builtin")
                    ],
                    type_name: SimpleTypeName {
                        name: TokenSyntax::from("String"),
                        type_args: None
                    }
                }))
            ))
        );
    }

    #[test]
    fn test_pointer_type() {
        assert_eq!(
            decorated_type("*T"),
            Ok((
                "",
                DecoratedTypeName {
                    decoration: TokenSyntax::from("*"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("T"),
                        type_args: None
                    })
                }
            ))
        );
    }

    #[test]
    fn test_reference_type() {
        assert_eq!(
            decorated_type("&T"),
            Ok((
                "",
                DecoratedTypeName {
                    decoration: TokenSyntax::from("&"),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("T"),
                        type_args: None
                    })
                }
            ))
        );
    }

    #[test]
    fn test_simple_type_parameter() {
        assert_eq!(
            type_parameter("T"),
            Ok((
                "",
                TypeParam {
                    name: TokenSyntax::from("T"),
                    type_constraint: None
                }
            ))
        );
    }

    #[test]
    fn test_type_parameter() {
        check(
            "T:Int",
            type_parameter,
            TypeParam {
                name: TokenSyntax::from("T"),
                type_constraint: Some(TypeConstraintSyntax {
                    sep: TokenSyntax::from(":"),
                    constraint: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None,
                    }),
                }),
            },
        );
        check(
            "T :Int",
            type_parameter,
            TypeParam {
                name: TokenSyntax::from("T"),
                type_constraint: Some(TypeConstraintSyntax {
                    sep: TokenSyntax::from(":")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    constraint: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None,
                    }),
                }),
            },
        );
        check(
            "T: Int",
            type_parameter,
            TypeParam {
                name: TokenSyntax::from("T"),
                type_constraint: Some(TypeConstraintSyntax {
                    sep: TokenSyntax::from(":"),
                    constraint: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
            },
        );
        check(
            "T : Int",
            type_parameter,
            TypeParam {
                name: TokenSyntax::from("T"),
                type_constraint: Some(TypeConstraintSyntax {
                    sep: TokenSyntax::from(":")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                    constraint: TypeName::Simple(SimpleTypeName {
                        name: TokenSyntax::from("Int"),
                        type_args: None,
                    })
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                }),
            },
        );
    }

    #[test]
    fn test_type_parameters_single() {
        check(
            "<T: A >",
            type_parameters,
            TypeParameterListSyntax {
                open: TokenSyntax::from("<"),
                elements: vec![TypeParameterElementSyntax {
                    element: TypeParam {
                        name: TokenSyntax::from("T"),
                        type_constraint: Some(TypeConstraintSyntax {
                            sep: TokenSyntax::from(":"),
                            constraint: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("A"),
                                type_args: None,
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        }),
                    },
                    trailing_comma: None,
                }],
                close: TokenSyntax::from(">")
                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
            },
        );
    }

    #[test]
    fn test_type_parameters_single_with_trailing_comma() {
        check(
            "<T: A,>",
            type_parameters,
            TypeParameterListSyntax {
                open: TokenSyntax::from("<"),
                elements: vec![TypeParameterElementSyntax {
                    element: TypeParam {
                        name: TokenSyntax::from("T"),
                        type_constraint: Some(TypeConstraintSyntax {
                            sep: TokenSyntax::from(":"),
                            constraint: TypeName::Simple(SimpleTypeName {
                                name: TokenSyntax::from("A"),
                                type_args: None,
                            })
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        }),
                    },
                    trailing_comma: Some(TokenSyntax::from(",")),
                }],
                close: TokenSyntax::from(">"),
            },
        );
    }

    #[test]
    fn test_type_parameters_multi() {
        check(
            "<T: A, U :B>",
            type_parameters,
            TypeParameterListSyntax {
                open: TokenSyntax::from("<"),
                elements: vec![
                    TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T"),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("A"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: Some(TokenSyntax::from(",")),
                    },
                    TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("U")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("B"),
                                    type_args: None,
                                }),
                            }),
                        },
                        trailing_comma: None,
                    },
                ],
                close: TokenSyntax::from(">"),
            },
        );
    }

    #[test]
    fn test_type_parameters_multi_with_trailing_comma() {
        check(
            "<T: A, U :B ,>",
            type_parameters,
            TypeParameterListSyntax {
                open: TokenSyntax::from("<"),
                elements: vec![
                    TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T"),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":"),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("A"),
                                    type_args: None,
                                })
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            }),
                        },
                        trailing_comma: Some(TokenSyntax::from(",")),
                    },
                    TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("U")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("B"),
                                    type_args: None,
                                }),
                            }),
                        },
                        trailing_comma: Some(
                            TokenSyntax::from(",")
                                .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        ),
                    },
                ],
                close: TokenSyntax::from(">"),
            },
        );
    }

    #[test]
    fn test_array_type_syntax() {
        check("[Int64;12]", array_type_syntax, ArrayTypeSyntax {
            open: TokenSyntax::from("["),
            type_: TypeName::Simple(SimpleTypeName::from("Int64")),
            semicolon: TokenSyntax::from(";"),
            size: TokenSyntax::from("12"),
            close: TokenSyntax::from("]"),
        });
    }
}
