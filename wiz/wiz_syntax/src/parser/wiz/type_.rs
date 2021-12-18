use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::lexical_structure::{identifier, whitespace0};
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{
    DecoratedTypeName, SimpleTypeName, TypeArgumentElementSyntax, TypeArgumentListSyntax,
    TypeConstraintSyntax, TypeName, TypeNameSpaceElementSyntax, TypeParam,
    TypeParameterElementSyntax, TypeParameterListSyntax, UserTypeName,
};
use crate::syntax::Syntax;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
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
    map(tuple((char('('), type_, char(')'))), |(_, type_, _)| type_)(s)
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
            alt((char('*'), ampersand)),
            alt((type_reference, parenthesized_type)),
        )),
        |(p, type_name)| DecoratedTypeName {
            decoration: TokenSyntax::from(p),
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
            many0(tuple((simple_user_type, tag("::")))),
            simple_user_type,
        )),
        |(name_space, type_name)| {
            if name_space.is_empty() {
                TypeName::Simple(type_name)
            } else {
                TypeName::NameSpaced(Box::new(UserTypeName {
                    name_space: name_space
                        .into_iter()
                        .map(|(simple_type, sep)| TypeNameSpaceElementSyntax {
                            simple_type,
                            sep: TokenSyntax::from(sep),
                        })
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
            char('<'),
            many0(tuple((whitespace0, type_, whitespace0, comma))),
            whitespace0,
            opt(type_),
            whitespace0,
            char('>'),
        )),
        |(open, t, ws, typ, tws, close)| {
            let mut close = TokenSyntax::from(close);
            let mut elements: Vec<_> = t
                .into_iter()
                .map(|(lws, tp, rws, com)| TypeArgumentElementSyntax {
                    element: tp.with_leading_trivia(lws),
                    trailing_comma: Some(TokenSyntax::from(com).with_leading_trivia(rws)),
                })
                .collect();
            match typ {
                None => {
                    close = close.with_leading_trivia(ws + tws);
                }
                Some(p) => {
                    elements.push(TypeArgumentElementSyntax {
                        element: p.with_leading_trivia(ws),
                        trailing_comma: None,
                    });
                    close = close.with_leading_trivia(tws);
                }
            };
            TypeArgumentListSyntax {
                open: TokenSyntax::from(open),
                elements,
                close,
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
            char('<'),
            many0(tuple((whitespace0, type_parameter, whitespace0, comma))),
            whitespace0,
            opt(type_parameter),
            whitespace0,
            char('>'),
        )),
        |(open, params, ws, param, tws, close)| {
            let mut close = TokenSyntax::from(close);
            let mut elements: Vec<_> = params
                .into_iter()
                .map(|(lws, tp, rws, com)| TypeParameterElementSyntax {
                    element: tp.with_leading_trivia(lws),
                    trailing_comma: Some(TokenSyntax::from(com).with_leading_trivia(rws)),
                })
                .collect();
            match param {
                None => {
                    close = close.with_leading_trivia(ws + tws);
                }
                Some(p) => {
                    elements.push(TypeParameterElementSyntax {
                        element: p.with_leading_trivia(ws),
                        trailing_comma: None,
                    });
                    close = close.with_leading_trivia(tws);
                }
            };
            TypeParameterListSyntax {
                open: TokenSyntax::from(open),
                elements,
                close,
            }
        },
    )(s)
}

// <type_parameter> ::= <identifier> (":", <type>)?
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
            opt(tuple((whitespace0, char(':'), whitespace0, type_))),
        )),
        |(name, typ)| TypeParam {
            name: TokenSyntax::from(name),
            type_constraint: typ.map(|(lws, c, ws, t)| TypeConstraintSyntax {
                sep: TokenSyntax::from(c)
                    .with_leading_trivia(lws)
                    .with_trailing_trivia(ws),
                constraint: t,
            }),
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::type_::{decorated_type, type_parameter, type_parameters, user_type};
    use crate::syntax::token::TokenSyntax;
    use crate::syntax::trivia::{Trivia, TriviaPiece};
    use crate::syntax::type_name::{
        DecoratedTypeName, SimpleTypeName, TypeConstraintSyntax, TypeName,
        TypeNameSpaceElementSyntax, TypeParam, TypeParameterElementSyntax, TypeParameterListSyntax,
        UserTypeName,
    };
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
        assert_eq!(
            type_parameter("T:Int"),
            Ok((
                "",
                TypeParam {
                    name: TokenSyntax::from("T"),
                    type_constraint: Some(TypeConstraintSyntax {
                        sep: TokenSyntax::from(":"),
                        constraint: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int"),
                            type_args: None
                        })
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T :Int"),
            Ok((
                "",
                TypeParam {
                    name: TokenSyntax::from("T"),
                    type_constraint: Some(TypeConstraintSyntax {
                        sep: TokenSyntax::from(":")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        constraint: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int"),
                            type_args: None
                        })
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T: Int"),
            Ok((
                "",
                TypeParam {
                    name: TokenSyntax::from("T"),
                    type_constraint: Some(TypeConstraintSyntax {
                        sep: TokenSyntax::from(":")
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        constraint: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int"),
                            type_args: None
                        })
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T : Int"),
            Ok((
                "",
                TypeParam {
                    name: TokenSyntax::from("T"),
                    type_constraint: Some(TypeConstraintSyntax {
                        sep: TokenSyntax::from(":")
                            .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                        constraint: TypeName::Simple(SimpleTypeName {
                            name: TokenSyntax::from("Int"),
                            type_args: None
                        })
                    })
                }
            ))
        );
    }

    #[test]
    fn test_type_parameters_single() {
        assert_eq!(
            type_parameters("<T: A >"),
            Ok((
                "",
                TypeParameterListSyntax {
                    open: TokenSyntax::from("<"),
                    elements: vec![TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T"),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":")
                                    .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("A"),
                                    type_args: None
                                })
                            })
                        },
                        trailing_comma: None
                    }],
                    close: TokenSyntax::from(">")
                        .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                }
            ))
        );
    }

    #[test]
    fn test_type_parameters_single_with_trailing_comma() {
        assert_eq!(
            type_parameters("<T: A,>"),
            Ok((
                "",
                TypeParameterListSyntax {
                    open: TokenSyntax::from("<"),
                    elements: vec![TypeParameterElementSyntax {
                        element: TypeParam {
                            name: TokenSyntax::from("T"),
                            type_constraint: Some(TypeConstraintSyntax {
                                sep: TokenSyntax::from(":")
                                    .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                constraint: TypeName::Simple(SimpleTypeName {
                                    name: TokenSyntax::from("A"),
                                    type_args: None
                                })
                            })
                        },
                        trailing_comma: Some(TokenSyntax::from(","))
                    }],
                    close: TokenSyntax::from(">")
                }
            ))
        );
    }

    #[test]
    fn test_type_parameters_multi() {
        assert_eq!(
            type_parameters("<T: A, U :B>"),
            Ok((
                "",
                TypeParameterListSyntax {
                    open: TokenSyntax::from("<"),
                    elements: vec![
                        TypeParameterElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T"),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("A"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(TokenSyntax::from(","))
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
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: None
                        }
                    ],
                    close: TokenSyntax::from(">")
                }
            ))
        );
    }

    #[test]
    fn test_type_parameters_multi_with_trailing_comma() {
        assert_eq!(
            type_parameters("<T: A, U :B ,>"),
            Ok((
                "",
                TypeParameterListSyntax {
                    open: TokenSyntax::from("<"),
                    elements: vec![
                        TypeParameterElementSyntax {
                            element: TypeParam {
                                name: TokenSyntax::from("T"),
                                type_constraint: Some(TypeConstraintSyntax {
                                    sep: TokenSyntax::from(":")
                                        .with_trailing_trivia(Trivia::from(TriviaPiece::Spaces(1))),
                                    constraint: TypeName::Simple(SimpleTypeName {
                                        name: TokenSyntax::from("A"),
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(TokenSyntax::from(","))
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
                                        type_args: None
                                    })
                                })
                            },
                            trailing_comma: Some(
                                TokenSyntax::from(",")
                                    .with_leading_trivia(Trivia::from(TriviaPiece::Spaces(1)))
                            )
                        }
                    ],
                    close: TokenSyntax::from(">")
                }
            ))
        );
    }
}
