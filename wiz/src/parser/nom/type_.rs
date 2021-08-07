use crate::ast::type_name::{TypeName, TypeParam};
use crate::parser::nom::lexical_structure::identifier;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

pub fn type_(s: &str) -> IResult<&str, TypeName> {
    alt((
        parenthesized_type,
        nullable_type,
        type_reference,
        // function_type,
    ))(s)
}

pub fn parenthesized_type(s: &str) -> IResult<&str, TypeName> {
    map(tuple((char('('), type_, char(')'))), |(_, type_, _)| type_)(s)
}

pub fn nullable_type(s: &str) -> IResult<&str, TypeName> {
    map(
        tuple((alt((type_reference, parenthesized_type)), char('?'))),
        |(type_name, hatena)| type_name,
    )(s)
}

pub fn type_reference(s: &str) -> IResult<&str, TypeName> {
    user_type(s)
}

pub fn user_type(s: &str) -> IResult<&str, TypeName> {
    map(
        tuple((
            simple_user_type,
            many0(tuple((char('.'), simple_user_type))),
        )),
        |(p, chs)| {
            // TODO: use chs
            p
        },
    )(s)
}

pub fn simple_user_type(s: &str) -> IResult<&str, TypeName> {
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

pub fn type_arguments(s: &str) -> IResult<&str, Vec<TypeName>> {
    map(
        tuple((
            char('<'),
            type_,
            many0(tuple((char(','), type_))),
            opt(char(',')),
            char('>'),
        )),
        |(_, t, ts, _, _)| {
            let mut t = vec![t];
            t.append(
                ts.into_iter()
                    .map(|(_, b)| b)
                    .collect::<Vec<TypeName>>()
                    .as_mut(),
            );
            t
        },
    )(s)
}
