use nom::{IResult, InputTakeAtPosition, AsChar};
use nom::character::complete::{space0, space1, alpha1, one_of, char};
use nom::branch::alt;
use crate::parser::nom::character::{alphabet, digit, under_score};
use nom::combinator::{opt, map};
use nom::sequence::tuple;
use nom::error::{ParseError, ErrorKind};

pub fn whitespace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !c.is_whitespace()
    })
}

pub fn whitespace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(|item| {
        let c = item.as_char();
        !c.is_whitespace()
    }, ErrorKind::Space)
}

pub fn whitespace_without_eol0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
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
    input.split_at_position1_complete(|item| {
        let c = item.as_char();
        !c.is_whitespace() || (c == '\n')
    }, ErrorKind::Space)
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

#[cfg(test)]
mod tests {
    use crate::parser::nom::lexical_structure::identifier;
    use nom::error::ErrorKind;
    use nom::error;
    use nom::Err;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("hello"), Ok(("", "hello".to_string())));
        assert_eq!(identifier("`hello`"), Ok(("", "`hello`".to_string())));
        assert_eq!(identifier("1"), Err(Err::Error(error::Error { input: "1", code: ErrorKind::Char })));
        assert_eq!(identifier("1ab"), Err(Err::Error(error::Error { input: "1ab", code: ErrorKind::Char })));
        assert_eq!(identifier("`1ab`"), Err(Err::Error(error::Error { input: "`1ab`", code: ErrorKind::Char })));
    }
}
