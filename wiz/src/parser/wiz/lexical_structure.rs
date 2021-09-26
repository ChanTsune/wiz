use crate::parser::wiz::character::{alphabet, cr, digit, eol, space, under_score};
use crate::syntax::trivia::TriviaPiece;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_until, take_while_m_n};
use nom::character::complete::{char, newline, tab};
use nom::combinator::{map, opt};
use nom::error::{ErrorKind, ParseError};
use nom::lib::std::ops::RangeFrom;
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::{
    AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Slice,
};
use std::iter::FromIterator;

pub fn whitespace0(s: &str) -> IResult<&str, String> {
    map(
        tuple((_whitespace0, opt(comment), _whitespace0)),
        |(w1, c, w2)| String::from(w1) + &*c.unwrap_or_default() + w2,
    )(s)
}

pub fn whitespace1(s: &str) -> IResult<&str, String> {
    alt((
        map(
            alt((
                tuple((_whitespace0, opt(comment), _whitespace1)),
                tuple((_whitespace1, opt(comment), _whitespace0)),
            )),
            |(w1, c, w2)| String::from(w1) + &*c.unwrap_or_default() + w2,
        ),
        comment,
    ))(s)
}
pub fn whitespace_without_eol0(s: &str) -> IResult<&str, String> {
    map(
        tuple((
            _whitespace_without_eol0,
            opt(comment),
            _whitespace_without_eol0,
        )),
        |(w1, c, w2)| String::from(w1) + &*c.unwrap_or_default() + w2,
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

fn line_comment_start<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("//")(s)
}

fn none_of_newline<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(
        take_while_m_n(1, 1, |c: <I as InputIter>::Item| c.as_char() != '\n'),
        |p: I| p.iter_elements().next().unwrap().as_char(),
    )(s)
}

fn _line_comment<I>(input: I) -> IResult<I, String>
where
    I: InputTake
        + Compare<&'static str>
        + Clone
        + InputLength
        + InputIter
        + Slice<RangeFrom<usize>>
        + ToString,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((line_comment_start, many0(none_of_newline), opt(newline))),
        |(s, c, e): (I, _, _)| {
            s.to_string() + &*String::from_iter(c) + &*e.map(|c| c.to_string()).unwrap_or_default()
        },
    )(input)
}

fn line_comment<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: InputTake
        + Compare<&'static str>
        + Clone
        + InputLength
        + InputIter
        + Slice<RangeFrom<usize>>
        + ToString,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(_line_comment, |c| TriviaPiece::LineComment(c))(s)
}

fn block_comment_start<I>(input: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("/*")(input)
}

fn block_comment_end<I>(input: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("*/")(input)
}

fn take_until_block_comment_end<I>(input: I) -> IResult<I, I>
where
    I: InputTake + FindSubstring<&'static str>,
{
    take_until("*/")(input)
}

fn _block_comment<I>(input: I) -> IResult<I, String>
    where
        I: InputTake + FindSubstring<&'static str> + Compare<&'static str> + ToString + Clone,
{
    map(
        permutation((
            block_comment_start,
            take_until_block_comment_end,
            block_comment_end,
        )),
        |(a, b, c): (I, I, I)| a.to_string() + &*b.to_string() + &*c.to_string(),
    )(input)
}

pub fn block_comment<I>(input: I) -> IResult<I, TriviaPiece>
    where
        I: InputTake + FindSubstring<&'static str> + Compare<&'static str> + ToString + Clone,
{
    map(_block_comment, |s|{
        TriviaPiece::BlockComment(s)
    })(input)
}
pub fn comment(input: &str) -> IResult<&str, String> {
    alt((_line_comment, _block_comment))(input)
}

pub fn identifier_head(s: &str) -> IResult<&str, char> {
    alt((alphabet, under_score))(s)
}
pub fn identifier_character(s: &str) -> IResult<&str, char> {
    alt((alphabet, under_score, digit))(s)
}

pub fn identifier_characters(s: &str) -> IResult<&str, String> {
    map(many0(identifier_character), |c| String::from_iter(c))(s)
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
            |(a, b, c, d)| a + &*b + &*c.unwrap_or_default() + &*d,
        ),
        map(
            tuple((identifier_head, opt(identifier_characters))),
            |(a, b)| a.to_string() + &*b.unwrap_or_default(),
        ),
    ))(s)
}

pub fn spaces<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(space), |l| TriviaPiece::Spaces(l.len() as i64))(s)
}

pub fn tabs<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(tab), |l| TriviaPiece::Tabs(l.len() as i64))(s)
}

pub fn newlines<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(eol), |l| TriviaPiece::Newlines(l.len() as i64))(s)
}

pub fn carriage_returns<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(cr), |l| TriviaPiece::CarriageReturns(l.len() as i64))(s)
}

pub fn trivia_piece<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength + ToString
    + InputTake + FindSubstring<& 'static str> + Compare<& 'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((
        spaces,
        tabs,
        newlines,
        carriage_returns,
        line_comment,
        block_comment,
        ))(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::lexical_structure::{carriage_returns, comment, identifier, line_comment, newlines, spaces, tabs, whitespace0, whitespace1, block_comment};
    use crate::syntax::trivia::TriviaPiece;
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
        );
        assert_eq!(comment("//"), Ok(("", String::from("//"))));
        assert_eq!(
            comment("// code comment\n"),
            Ok(("", String::from("// code comment\n")))
        );
    }

    #[test]
    fn test_inline_comment() {
        assert_eq!(comment("/* a */"), Ok(("", String::from("/* a */"))));
        assert_eq!(comment("/**/"), Ok(("", String::from("/**/"))));
        assert_eq!(comment("/*\n*/"), Ok(("", String::from("/*\n*/"))));
    }

    #[test]
    fn test_whitespace0() {
        assert_eq!(whitespace0(""), Ok(("", String::from(""))));
        assert_eq!(whitespace0("\n"), Ok(("", String::from("\n"))));
        assert_eq!(whitespace0(" \n "), Ok(("", String::from(" \n "))));
        assert_eq!(whitespace0("        "), Ok(("", String::from("        "))));
    }

    #[test]
    fn test_whitespace1() {
        assert_eq!(whitespace1(" "), Ok(("", String::from(" "))));
        assert_eq!(whitespace1("        "), Ok(("", String::from("        "))))
    }

    #[test]
    fn test_whitespace0_with_comment() {
        assert_eq!(
            whitespace0("// code comment"),
            Ok(("", String::from("// code comment")))
        );
        assert_eq!(whitespace0("//"), Ok(("", String::from("//"))));
        assert_eq!(
            whitespace0("// code comment\n"),
            Ok(("", String::from("// code comment\n")))
        );
        assert_eq!(whitespace0("/* a */"), Ok(("", String::from("/* a */"))));
        assert_eq!(whitespace0("/**/"), Ok(("", String::from("/**/"))));
    }

    #[test]
    fn test_whitespace1_with_comment() {
        assert_eq!(
            whitespace1("// code comment"),
            Ok(("", String::from("// code comment")))
        );
        assert_eq!(whitespace1("//"), Ok(("", String::from("//"))));
        assert_eq!(
            whitespace1("// code comment\n"),
            Ok(("", String::from("// code comment\n")))
        );
        assert_eq!(whitespace1("/* a */"), Ok(("", String::from("/* a */"))));
        assert_eq!(whitespace1("/**/"), Ok(("", String::from("/**/"))));
    }

    #[test]
    fn test_spaces() {
        assert_eq!(spaces(" "), Ok(("", TriviaPiece::Spaces(1))))
    }

    #[test]
    fn test_tabs() {
        assert_eq!(tabs("\t"), Ok(("", TriviaPiece::Tabs(1))))
    }

    #[test]
    fn test_newlines() {
        assert_eq!(newlines("\n"), Ok(("", TriviaPiece::Newlines(1))))
    }

    #[test]
    fn test_carriage_returns() {
        assert_eq!(
            carriage_returns("\r"),
            Ok(("", TriviaPiece::CarriageReturns(1)))
        )
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            line_comment("// this is comment"),
            Ok((
                "",
                TriviaPiece::LineComment(String::from("// this is comment"))
            ))
        );
        assert_eq!(
            line_comment("// this is comment\n"),
            Ok((
                "",
                TriviaPiece::LineComment(String::from("// this is comment\n"))
            ))
        );
        assert_eq!(
            line_comment("//comment\nnot comment\n"),
            Ok((
                "not comment\n",
                TriviaPiece::LineComment(String::from("//comment\n"))
            ))
        );
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(
            block_comment("/* this is comment */"),
            Ok((
                "",
                TriviaPiece::BlockComment(String::from("/* this is comment */"))
            ))
        );
        assert_eq!(
            block_comment("/**/"),
            Ok((
                "",
                TriviaPiece::BlockComment(String::from("/**/"))
            ))
        );
        assert_eq!(
            block_comment("/*/comment\n*/not comment\n"),
            Ok((
                "not comment\n",
                TriviaPiece::BlockComment(String::from("/*/comment\n*/"))
            ))
        );
    }
}
