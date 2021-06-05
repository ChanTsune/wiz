use nom::IResult;
use crate::parser::nom::lexical_structure::identifier;
use crate::ast::type_name::TypeName;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::character::complete::{char, anychar};
use nom::branch::alt;
use nom::multi::many0;

pub fn type_(s: &str) -> IResult<&str, TypeName> {
    alt((
        parenthesized_type,
        nullable_type,
        type_reference,
        // function_type,
        ))(s)
}

pub fn parenthesized_type(s: &str) -> IResult<&str, TypeName> {
    map(tuple((
        char('('),
        type_,
        char(')')
        )), |(_, type_,_)| {
        type_
    })(s)
}

pub fn nullable_type(s: &str) -> IResult<&str, TypeName> {
    map(tuple((
        alt((
            type_reference,
            parenthesized_type,
            )),
        char('?')
        )), |(type_name, hatena)| {
        type_name
    })(s)
}

pub fn type_reference(s: &str) -> IResult<&str, TypeName> {
    user_type(s)
}

pub fn user_type(s: &str) -> IResult<&str, TypeName> {
    map(tuple(
        (simple_user_type,
        many0(tuple((
            char('.'),
            simple_user_type,
            ))))
    ), |(p, chs)| {
        // TODO: use chs
        p
    })(s)
}

pub fn simple_user_type(s: &str) -> IResult<&str, TypeName> {
    map(tuple((
        identifier,
        opt(anychar) // TODO: replace to `type_arguments`
        )), |(name, args)| {
        // TODO: use args
        TypeName{ name, type_params: vec![] }
    })(s)
}

// pub fn function_type(s: &str) -> IResult<&str, TypeName> {
//
// }
