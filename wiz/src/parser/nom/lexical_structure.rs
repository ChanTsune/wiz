use nom::IResult;
use nom::character::complete::{space0, space1, alpha1, one_of, char};
use nom::branch::alt;
use crate::parser::nom::character::{alphabet, digit, under_score};
use nom::combinator::{opt, map};
use nom::sequence::tuple;

pub fn whitespace0(s: &str) -> IResult<&str, &str> {
    space0(s)
}

pub fn whitespace1(s: &str) -> IResult<&str, &str> {
    space1(s)
}

pub fn identifier_head(s: &str) -> IResult<&str, char> {
    alt((alphabet,
            under_score
    ))(s)
}
pub fn identifier_character(s: &str) -> IResult<&str, char> {
    alt((
        alphabet,
        under_score,
        digit,
    ))(s)
}

pub fn identifier_characters(s: &str) -> IResult<&str, String> {
    map(tuple((
            identifier_character,
            opt(identifier_characters)
        )),
        |(c, ops)| {
            c.to_string() + &*ops.unwrap_or("".to_string())
        }
    )(s)
}

pub fn identifier(s: &str) -> IResult<&str, String> {
    alt((
        map(
            tuple((
                map(char('`'), |r| { r.to_string() }),
                map(identifier_head, |r| { r.to_string() }),
                opt(identifier_characters),
                map(char('`'), |r| { r.to_string() }),
            )), |(a,b,c,d)| {
                a + &*b + &*c.unwrap_or("".to_string()) + &*d
            }
        ),
        map(
            tuple((
                identifier_head,
                opt(identifier_characters)
            )), |(a, b)| {
                a.to_string() + &*b.unwrap_or("".to_string())
            }
        )
    ))(s)
}
