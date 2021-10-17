use crate::parser::wiz::character::{
    alphabet, backticks, cr, digit, form_feed, space, under_score, vertical_tab,
};
use crate::syntax::trivia::{Trivia, TriviaPiece};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_until, take_while_m_n};
use nom::character::complete::{crlf, newline, tab};
use nom::combinator::{map, opt};
use nom::lib::std::ops::{Range, RangeFrom};
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::{AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake, Slice};
use std::iter::FromIterator;

pub fn whitespace0<I>(s: I) -> IResult<I, Trivia>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(many0(trivia_piece), Trivia::from)(s)
}

pub fn whitespace1<I>(s: I) -> IResult<I, Trivia>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(many1(trivia_piece),Trivia::from)(s)
}

pub fn whitespace_without_eol0<I>(s: I) -> IResult<I, Trivia>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(many0(trivia_piece_without_line_ending), Trivia::from)(s)
}

fn line_comment_start<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("//")(s)
}

fn doc_line_comment_start<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("///")(s)
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

fn _doc_line_comment<I>(input: I) -> IResult<I, String>
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
        tuple((doc_line_comment_start, many0(none_of_newline), opt(newline))),
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
    map(_line_comment,  TriviaPiece::LineComment)(s)
}

fn doc_line_comment<I>(s: I) -> IResult<I, TriviaPiece>
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
    map(_doc_line_comment, TriviaPiece::DocLineComment)(s)
}

fn block_comment_start<I>(input: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("/*")(input)
}

fn doc_block_comment_start<I>(input: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("/**")(input)
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

fn _doc_block_comment<I>(input: I) -> IResult<I, String>
where
    I: InputTake + FindSubstring<&'static str> + Compare<&'static str> + ToString + Clone,
{
    map(
        permutation((
            doc_block_comment_start,
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
    map(_block_comment, TriviaPiece::BlockComment)(input)
}

pub fn doc_block_comment<I>(input: I) -> IResult<I, TriviaPiece>
where
    I: InputTake + FindSubstring<&'static str> + Compare<&'static str> + ToString + Clone,
{
    map(_doc_block_comment, TriviaPiece::DocBlockComment)(input)
}

pub fn identifier_head<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    alt((alphabet, under_score))(s)
}

pub fn identifier_character<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    alt((alphabet, under_score, digit))(s)
}

pub fn identifier_characters<I>(s: I) -> IResult<I, String>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    map(many0(identifier_character), String::from_iter)(s)
}

pub fn identifier<I>(s: I) -> IResult<I, String>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength + Clone,
    <I as InputIter>::Item: AsChar,
{
    alt((
        map(
            tuple((
                map(backticks, |r| r.to_string()),
                map(identifier_head, |r| r.to_string()),
                identifier_characters,
                map(backticks, |r| r.to_string()),
            )),
            |(a, b, c, d)| a + &*b + &*c + &*d,
        ),
        map(tuple((identifier_head, identifier_characters)), |(a, b)| {
            a.to_string() + &*b
        }),
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

pub fn vertical_tabs<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(vertical_tab), |l| {
        TriviaPiece::VerticalTabs(l.len() as i64)
    })(s)
}

pub fn form_feeds<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(form_feed), |l| TriviaPiece::FormFeeds(l.len() as i64))(s)
}

pub fn newlines<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(newline), |l| TriviaPiece::Newlines(l.len() as i64))(s)
}

pub fn carriage_returns<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(many1(cr), |l| TriviaPiece::CarriageReturns(l.len() as i64))(s)
}

pub fn carriage_return_line_feeds<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + Clone
        + InputLength
        + InputIter
        + Compare<&'static str>,
{
    map(many1(crlf), |l| {
        TriviaPiece::CarriageReturnLineFeeds(l.len() as i64)
    })(s)
}

pub fn trivia_piece<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((
        spaces,
        tabs,
        vertical_tabs,
        form_feeds,
        carriage_return_line_feeds,
        newlines,
        carriage_returns,
        doc_line_comment,
        doc_block_comment,
        line_comment,
        block_comment,
    ))(s)
}

pub fn trivia_piece_without_line_ending<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((
        spaces,
        tabs,
        vertical_tabs,
        form_feeds,
        carriage_return_line_feeds,
        doc_line_comment,
        doc_block_comment,
        line_comment,
        block_comment,
    ))(s)
}

pub fn trivia_piece_line_ending<I>(s: I) -> IResult<I, TriviaPiece>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    alt((carriage_return_line_feeds, newlines, carriage_returns))(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::lexical_structure::{
        block_comment, carriage_return_line_feeds, carriage_returns, doc_block_comment,
        doc_line_comment, form_feeds, identifier, line_comment, newlines, spaces, tabs,
        vertical_tabs, whitespace0, whitespace1,
    };
    use crate::syntax::trivia::{Trivia, TriviaPiece};
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
    fn test_whitespace0() {
        assert_eq!(whitespace0(""), Ok(("", Trivia::new())));
        assert_eq!(
            whitespace0("\n"),
            Ok(("", Trivia::from(TriviaPiece::Newlines(1))))
        );
        assert_eq!(
            whitespace0(" \n "),
            Ok((
                "",
                Trivia::from(vec![
                    TriviaPiece::Spaces(1),
                    TriviaPiece::Newlines(1),
                    TriviaPiece::Spaces(1)
                ])
            ))
        );
        assert_eq!(
            whitespace0("        "),
            Ok(("", Trivia::from(TriviaPiece::Spaces(8))))
        );
    }

    #[test]
    fn test_whitespace1() {
        assert_eq!(
            whitespace1(" "),
            Ok(("", Trivia::from(TriviaPiece::Spaces(1))))
        );
        assert_eq!(
            whitespace1("        "),
            Ok(("", Trivia::from(TriviaPiece::Spaces(8))))
        );
    }

    #[test]
    fn test_whitespace0_with_comment() {
        assert_eq!(
            whitespace0("// code comment"),
            Ok((
                "",
                Trivia::from(TriviaPiece::LineComment(String::from("// code comment")))
            ))
        );
        assert_eq!(
            whitespace0("//"),
            Ok((
                "",
                Trivia::from(TriviaPiece::LineComment(String::from("//")))
            ))
        );
        assert_eq!(
            whitespace0("/// code comment\n"),
            Ok((
                "",
                Trivia::from(TriviaPiece::DocLineComment(String::from(
                    "/// code comment\n"
                )))
            ))
        );
        assert_eq!(
            whitespace0("/* a */"),
            Ok((
                "",
                Trivia::from(TriviaPiece::BlockComment(String::from("/* a */")))
            ))
        );
        assert_eq!(
            whitespace0("/***/"),
            Ok((
                "",
                Trivia::from(TriviaPiece::DocBlockComment(String::from("/***/")))
            ))
        );
    }

    #[test]
    fn test_whitespace1_with_comment() {
        assert_eq!(
            whitespace1("// code comment"),
            Ok((
                "",
                Trivia::from(TriviaPiece::LineComment(String::from("// code comment")))
            ))
        );
        assert_eq!(
            whitespace1("//"),
            Ok((
                "",
                Trivia::from(TriviaPiece::LineComment(String::from("//")))
            ))
        );
        assert_eq!(
            whitespace1("// code comment\n"),
            Ok((
                "",
                Trivia::from(TriviaPiece::LineComment(String::from("// code comment\n")))
            ))
        );
        assert_eq!(
            whitespace1("/* a */"),
            Ok((
                "",
                Trivia::from(TriviaPiece::BlockComment(String::from("/* a */")))
            ))
        );
        assert_eq!(
            whitespace1("/**/"),
            Ok((
                "",
                Trivia::from(TriviaPiece::BlockComment(String::from("/**/")))
            ))
        );
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
    fn test_vertical_tabs() {
        assert_eq!(
            vertical_tabs("\x0b"),
            Ok(("", TriviaPiece::VerticalTabs(1)))
        );
    }

    #[test]
    fn test_form_feeds() {
        assert_eq!(form_feeds("\x0c"), Ok(("", TriviaPiece::FormFeeds(1))));
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
    fn test_carriage_return_line_feeds() {
        assert_eq!(
            carriage_return_line_feeds("\r\n\r\n"),
            Ok(("", TriviaPiece::CarriageReturnLineFeeds(2)))
        )
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            line_comment("// code comment"),
            Ok((
                "",
                TriviaPiece::LineComment(String::from("// code comment"))
            ))
        );
        assert_eq!(
            line_comment("//"),
            Ok(("", TriviaPiece::LineComment(String::from("//"))))
        );
        assert_eq!(
            line_comment("// code comment\n"),
            Ok((
                "",
                TriviaPiece::LineComment(String::from("// code comment\n"))
            ))
        );
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
    fn test_doc_line_comment() {
        assert_eq!(
            doc_line_comment("/// doc comment\n"),
            Ok((
                "",
                TriviaPiece::DocLineComment(String::from("/// doc comment\n"))
            ))
        );
        assert_eq!(
            doc_line_comment("/// this is document comment"),
            Ok((
                "",
                TriviaPiece::DocLineComment(String::from("/// this is document comment"))
            ))
        );
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(
            block_comment("/* a */"),
            Ok(("", TriviaPiece::BlockComment(String::from("/* a */"))))
        );
        assert_eq!(
            block_comment("/**/"),
            Ok(("", TriviaPiece::BlockComment(String::from("/**/"))))
        );
        assert_eq!(
            block_comment("/*\n*/"),
            Ok(("", TriviaPiece::BlockComment(String::from("/*\n*/"))))
        );
        assert_eq!(
            block_comment("/* this is comment */"),
            Ok((
                "",
                TriviaPiece::BlockComment(String::from("/* this is comment */"))
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

    #[test]
    fn test_doc_block_comment() {
        assert_eq!(
            doc_block_comment("/** a */"),
            Ok(("", TriviaPiece::DocBlockComment(String::from("/** a */"))))
        );
        assert_eq!(
            doc_block_comment("/***/"),
            Ok(("", TriviaPiece::DocBlockComment(String::from("/***/"))))
        );
    }
}
