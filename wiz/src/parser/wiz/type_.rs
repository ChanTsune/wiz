use crate::parser::wiz::character::{comma, dot};
use crate::parser::wiz::lexical_structure::{identifier, whitespace0};
use crate::syntax::type_name::{TypeName, TypeParam};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::RangeFrom;

pub fn type_<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    alt((
        parenthesized_type,
        nullable_type,
        type_reference,
        // function_type,
    ))(s)
}

pub fn parenthesized_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((char('('), type_, char(')'))), |(_, type_, _)| type_)(s)
}

pub fn nullable_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((alt((type_reference, parenthesized_type)), char('?'))),
        |(type_name, hatena)| type_name,
    )(s)
}

pub fn type_reference<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    user_type(s)
}

pub fn user_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    map(
        tuple((simple_user_type, many0(tuple((dot, simple_user_type))))),
        |(p, chs)| {
            // TODO: use chs
            p
        },
    )(s)
}

pub fn simple_user_type<I>(s: I) -> IResult<I, TypeName>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((identifier, opt(type_arguments))), |(name, args)| {
        TypeName {
            name,
            type_args: args,
        }
    })(s)
}

// pub fn function_type(s: &str) -> IResult<&str, TypeName> {
//
// }

pub fn type_arguments<I>(s: I) -> IResult<I, Vec<TypeName>>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
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
pub fn type_parameters(s: &str) -> IResult<&str, Vec<TypeParam>> {
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
pub fn type_parameter(s: &str) -> IResult<&str, TypeParam> {
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
    use crate::parser::wiz::type_::type_parameter;
    use crate::syntax::type_name::{TypeName, TypeParam};

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
                    type_constraints: Some(TypeName {
                        name: "Int".to_string(),
                        type_args: None
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T :Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName {
                        name: "Int".to_string(),
                        type_args: None
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T: Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName {
                        name: "Int".to_string(),
                        type_args: None
                    })
                }
            ))
        );
        assert_eq!(
            type_parameter("T : Int"),
            Ok((
                "",
                TypeParam {
                    name: "T".to_string(),
                    type_constraints: Some(TypeName {
                        name: "Int".to_string(),
                        type_args: None
                    })
                }
            ))
        );
    }
}
