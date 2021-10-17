use crate::parser::wiz::character::{ampersand, comma};
use crate::parser::wiz::lexical_structure::{identifier, whitespace0};
use crate::parser::wiz::name_space::name_space;
use crate::syntax::type_name::{
    DecoratedTypeName, NameSpacedTypeName, SimpleTypeName, TypeName, TypeParam,
};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::{Range, RangeFrom};

pub fn type_<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    alt((
        parenthesized_type,
        pointer_type,
        reference_type,
        type_reference,
        // function_type,
    ))(s)
}

pub fn parenthesized_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((char('('), type_, char(')'))), |(_, type_, _)| type_)(s)
}

pub fn pointer_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((char('*'), alt((type_reference, parenthesized_type)))),
        |(p, type_name)| {
            TypeName::Decorated(Box::new(DecoratedTypeName {
                decoration: p.to_string(),
                type_: type_name,
            }))
        },
    )(s)
}

pub fn reference_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((ampersand, alt((type_reference, parenthesized_type)))),
        |(a, type_name)| {
            TypeName::Decorated(Box::new(DecoratedTypeName {
                decoration: a.to_string(),
                type_: type_name,
            }))
        },
    )(s)
}

pub fn type_reference<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    user_type(s)
}

pub fn user_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((name_space, simple_user_type)),
        |(name_space, type_name)| {
            if name_space.is_empty() {
                type_name
            } else {
                TypeName::NameSpaced(Box::new(NameSpacedTypeName {
                    name_space,
                    type_name,
                }))
            }
        },
    )(s)
}

pub fn simple_user_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((identifier, opt(type_arguments))), |(name, args)| {
        TypeName::Simple(SimpleTypeName {
            name,
            type_args: args,
        })
    })(s)
}

// pub fn function_type(s: &str) -> IResult<&str, TypeName> {
//
// }

pub fn type_arguments<I>(s: I) -> IResult<I, Vec<TypeName>>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((
            char('<'),
            type_,
            many0(tuple((comma, type_))),
            opt(comma),
            char('>'),
        )),
        |(_, t, ts, _, _)| {
            vec![t]
                .into_iter()
                .chain(ts.into_iter().map(|(_, b)| b))
                .collect()
        },
    )(s)
}

// <type_parameters> ::= "<" <type_parameter> ("," <type_parameter>)* ","? ">"
pub fn type_parameters<I>(s: I) -> IResult<I, Vec<TypeParam>>
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
            whitespace0,
            type_parameter,
            whitespace0,
            many0(tuple((comma, whitespace0, type_parameter, whitespace0))),
            whitespace0,
            opt(comma),
            whitespace0,
            char('>'),
        )),
        |(_, _, p1, _, pn, _, _, _, _)| {
            vec![p1]
                .into_iter()
                .chain(pn.into_iter().map(|(_, _, p, _)| p))
                .collect()
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
            whitespace0,
            opt(tuple((char(':'), whitespace0, type_))),
        )),
        |(name, _, typ)| TypeParam {
            name,
            type_constraints: typ.map(|(_, _, t)| t),
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::type_::{pointer_type, reference_type, type_parameter};
    use crate::syntax::type_name::{DecoratedTypeName, SimpleTypeName, TypeName, TypeParam};

    #[test]
    fn test_pointer_type() {
        assert_eq!(
            pointer_type("*T"),
            Ok((
                "",
                TypeName::Decorated(Box::new(DecoratedTypeName {
                    decoration: "*".to_string(),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: "T".to_string(),
                        type_args: None
                    })
                }))
            ))
        );
    }

    #[test]
    fn test_reference_type() {
        assert_eq!(
            reference_type("&T"),
            Ok((
                "",
                TypeName::Decorated(Box::new(DecoratedTypeName {
                    decoration: "&".to_string(),
                    type_: TypeName::Simple(SimpleTypeName {
                        name: "T".to_string(),
                        type_args: None
                    })
                }))
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
                    name: "T".to_string(),
                    type_constraints: None
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
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Int".to_string(),
                        type_args: None
                    }))
                }
            ))
        );
        assert_eq!(
            type_parameter("T :Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Int".to_string(),
                        type_args: None
                    }))
                }
            ))
        );
        assert_eq!(
            type_parameter("T: Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Int".to_string(),
                        type_args: None
                    }))
                }
            ))
        );
        assert_eq!(
            type_parameter("T : Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName::Simple(SimpleTypeName {
                        name: "Int".to_string(),
                        type_args: None
                    }))
                }
            ))
        );
    }
}
