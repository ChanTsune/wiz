use nom::bytes::complete::take_while_m_n;
use nom::character::complete::char;
use nom::combinator::map;
use nom::{AsChar, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::RangeFrom;

pub fn not_double_quote_or_back_slash<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(
        take_while_m_n(1, 1, |c: <I as InputIter>::Item| {
            let c = c.as_char();
            c != '"' && c != '\\'
        }),
        |p: I| p.iter_elements().next().unwrap().as_char(),
    )(s)
}

pub fn alphabet<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(
        take_while_m_n(1, 1, |c: <I as InputIter>::Item| c.is_alpha()),
        |p: I| p.iter_elements().next().unwrap().as_char(),
    )(s)
}

pub fn digit<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
    <I as InputIter>::Item: AsChar,
{
    map(
        take_while_m_n(1, 1, |c: <I as InputIter>::Item| c.is_dec_digit()),
        |p: I| p.iter_elements().next().unwrap().as_char(),
    )(s)
}

pub fn under_score<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('_')(s)
}

pub fn vertical_tab<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('\x0b')(s)
}

pub fn form_feed<I>(s: I) -> IResult<I, char>
    where
        I: Slice<RangeFrom<usize>> + InputIter,
        <I as InputIter>::Item: AsChar,
{
    char('\x0c')(s)
}

pub fn double_quote<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('"')(s)
}

pub fn backticks<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('`')(s)
}

pub fn dot<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('.')(s)
}

pub fn comma<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char(',')(s)
}

pub fn space<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char(' ')(s)
}

pub fn cr<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('\r')(s)
}

pub fn ampersand<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('&')(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::character::{alphabet, ampersand, backticks, comma, cr, digit, dot, double_quote, form_feed, not_double_quote_or_back_slash, space, under_score, vertical_tab};

    #[test]
    fn test_alphabet() {
        assert_eq!(alphabet("abc"), Ok(("bc", 'a')));
    }

    #[test]
    fn test_digit() {
        assert_eq!(digit("12"), Ok(("2", '1')))
    }

    #[test]
    fn test_under_score() {
        assert_eq!(under_score("_"), Ok(("", '_')))
    }

    #[test]
    fn test_vertical_tab() {
        assert_eq!(vertical_tab("\x0b"), Ok(("", '\x0b')))
    }

    #[test]
    fn test_form_feed() {
        assert_eq!(form_feed("\x0c"), Ok(("", '\x0c')))
    }

    #[test]
    fn test_double_quote() {
        assert_eq!(double_quote("\""), Ok(("", '"')));
    }

    #[test]
    fn test_backticks() {
        assert_eq!(backticks("`"), Ok(("", '`')));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot("."), Ok(("", '.')))
    }

    #[test]
    fn test_comma() {
        assert_eq!(comma(","), Ok(("", ',')));
    }

    #[test]
    fn test_space() {
        assert_eq!(space(" "), Ok(("", ' ')))
    }

    #[test]
    fn test_cr() {
        assert_eq!(cr("\r"), Ok(("", '\r')))
    }

    #[test]
    fn test_ampersand() {
        assert_eq!(ampersand("&"), Ok(("", '&')));
    }

    #[test]
    fn test_not_double_quote_or_back_slash() {
        assert_eq!(not_double_quote_or_back_slash("1"), Ok(("", '1')));
    }
}
