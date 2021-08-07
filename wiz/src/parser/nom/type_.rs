use crate::ast::type_name::{TypeName, TypeParam};
use crate::parser::nom::lexical_structure::{identifier, whitespace0};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::option::Option;

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

// <type_parameters> ::= "<" <type_parameter> ("," <type_parameter>)* ","? ">"
pub fn type_parameters(s: &str) -> IResult<&str,Vec<TypeParam>> {
    map(tuple((
        char('<'),
        whitespace0,
        type_parameter,
        whitespace0,
        many0(tuple((char(','), whitespace0, type_parameter, whitespace0))),
        whitespace0,
        opt(char(',')),
        whitespace0,
        char('>'),
        )),|(_, _, p1, _, pn, _, _, _, _)|{
        let mut type_params = vec![p1];
        type_params.append(pn.into_iter().map(|(_, _, p,_)|{p}).collect::<Vec<TypeParam>>().as_mut());
        type_params
    })(s)
}

// <type_parameter> ::= <identifier> (":", <type>)?
pub fn type_parameter(s: &str) -> IResult<&str, TypeParam> {
    map(tuple((
        identifier,
        whitespace0,
        opt(tuple((char(':'),
               whitespace0,
               type_))),
        )),|(name, _, typ)|{
        TypeParam {
            name,
            type_constraints: typ.map(|(_, _, t)|{vec![t]})
        }
    })(s)
}

mod test {
    use crate::parser::nom::type_::type_parameter;
    use crate::ast::type_name::{TypeParam, TypeName};

    #[test]
    fn test_simple_type_parameter() {
        assert_eq!(type_parameter("T"), Ok(("", TypeParam {
            name: "T".to_string(),
            type_constraints: None
        })));
    }

    #[test]
    fn test_type_parameter() {
        assert_eq!(type_parameter("T:Int"), Ok(("", TypeParam {
            name: "T".to_string(),
            type_constraints: Some(vec![TypeName{ name: "Int".to_string(), type_args: None }])
        })));
        assert_eq!(type_parameter("T :Int"), Ok(("", TypeParam {
            name: "T".to_string(),
            type_constraints: Some(vec![TypeName{ name: "Int".to_string(), type_args: None }])
        })));
        assert_eq!(type_parameter("T: Int"), Ok(("", TypeParam {
            name: "T".to_string(),
            type_constraints: Some(vec![TypeName{ name: "Int".to_string(), type_args: None }])
        })));
        assert_eq!(type_parameter("T : Int"), Ok(("", TypeParam {
            name: "T".to_string(),
            type_constraints: Some(vec![TypeName{ name: "Int".to_string(), type_args: None }])
        })));
    }
}
