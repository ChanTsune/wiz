use crate::parser::nom::character::{alphabet, digit, under_score};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, opt};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, IResult, InputTakeAtPosition};
use std::iter::FromIterator;

pub fn whitespace0(s: &str) -> IResult<&str, String> {
    map(
        tuple((_whitespace0, opt(comment), _whitespace0)),
        |(w1, c, w2)| String::from(w1) + &*c.unwrap_or(String::new()) + w2,
    )(s)
}

pub fn whitespace1(s: &str) -> IResult<&str, String> {
    map(
        alt((
            tuple((_whitespace0, opt(comment), _whitespace1)),
            tuple((_whitespace1, opt(comment), _whitespace0)),
        )),
        |(w1, c, w2)| String::from(w1) + &*c.unwrap_or(String::new()) + w2,
    )(s)
}
pub fn whitespace_without_eol0(s: &str) -> IResult<&str, String> {
    map(
        tuple((
            _whitespace_without_eol0,
            opt(comment),
            _whitespace_without_eol0,
        )),
        |(w1, c, w2)| String::from(w1) + &*c.unwrap_or(String::new()) + w2,
    )(s)
}

fn _whitespace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !c.is_whitespace()
    })
}

fn _whitespace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_whitespace()
        },
        ErrorKind::Space,
    )
}

fn _whitespace_without_eol0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !c.is_whitespace() || (c == '\n')
    })
}

pub fn whitespace_without_eol1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_whitespace() || (c == '\n')
        },
        ErrorKind::Space,
    )
}

pub fn line_comment_start(s: &str) -> IResult<&str, &str> {
    tag("//")(s)
}

pub fn line_comment(input: &str) -> IResult<&str, String> {
    map(
        tuple((line_comment_start, many0(anychar), opt(char('\n')))),
        |(s, c, e)| {
            s.to_string()
                + &*String::from_iter(c)
                + &*e.map(|c| c.to_string()).unwrap_or("".parse().unwrap())
        },
    )(input)
}

fn inline_comment_start(input: &str) -> IResult<&str, &str> {
    tag("/*")(input)
}

fn inline_comment_end(input: &str) -> IResult<&str, &str> {
    tag("*/")(input)
}

pub fn inline_comment(input: &str) -> IResult<&str, String> {
    map(
        permutation((inline_comment_start, is_not("*/"), inline_comment_end)),
        |(a, b, c)| a.to_string() + b + c,
    )(input)
}

pub fn comment(input: &str) -> IResult<&str, String> {
    alt((line_comment, inline_comment))(input)
}

pub fn identifier_head(s: &str) -> IResult<&str, char> {
    alt((alphabet, under_score))(s)
}
pub fn identifier_character(s: &str) -> IResult<&str, char> {
    alt((alphabet, under_score, digit))(s)
}

pub fn identifier_characters(s: &str) -> IResult<&str, String> {
    map(
        tuple((identifier_character, opt(identifier_characters))),
        |(c, ops)| c.to_string() + &*ops.unwrap_or("".to_string()),
    )(s)
}

pub fn identifier(s: &str) -> IResult<&str, String> {
    alt((
        map(
            tuple((
                map(char('`'), |r| r.to_string()),
                map(identifier_head, |r| r.to_string()),
                opt(identifier_characters),
                map(char('`'), |r| r.to_string()),
            )),
            |(a, b, c, d)| a + &*b + &*c.unwrap_or("".to_string()) + &*d,
        ),
        map(
            tuple((identifier_head, opt(identifier_characters))),
            |(a, b)| a.to_string() + &*b.unwrap_or("".to_string()),
        ),
    ))(s)
}

pub fn eol(s: &str) -> IResult<&str, char> {
    char('\n')(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::nom::lexical_structure::{comment, eol, identifier};
    use nom::error;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("hello"), Ok(("", "hello".to_string())));
        assert_eq!(identifier("`hello`"), Ok(("", "`hello`".to_string())));
        assert_eq!(
            identifier("1"),
            Err(Err::Error(error::Error {
                input: "1",
                code: ErrorKind::Char
            }))
        );
        assert_eq!(
            identifier("1ab"),
            Err(Err::Error(error::Error {
                input: "1ab",
                code: ErrorKind::Char
            }))
        );
        assert_eq!(
            identifier("`1ab`"),
            Err(Err::Error(error::Error {
                input: "`1ab`",
                code: ErrorKind::Char
            }))
        );
    }
    #[test]
    fn test_comment() {
        assert_eq!(
            comment("// code comment"),
            Ok(("", String::from("// code comment")))
        )
    }
    #[test]
    fn test_inline_comment() {
        assert_eq!(comment("/* a */"), Ok(("", String::from("/* a */"))))
    }

    #[test]
    fn test_eol() {
        assert_eq!(eol("\n"), Ok(("", '\n')))
    }
}
