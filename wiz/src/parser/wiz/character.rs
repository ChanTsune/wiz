use nom::character::complete::{char, newline};
use nom::{AsChar, IResult, InputIter, Slice, InputTake, InputLength};
use std::ops::RangeFrom;
use nom::bytes::complete::take_while_m_n;
use nom::combinator::map;

pub fn alphabet<I>(s: I) -> IResult<I, char>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
        <I as InputIter>::Item: AsChar,
{
    map(take_while_m_n(1, 1, |c:<I as InputIter>::Item| {
        c.is_alpha()
    }),|p:I|{
        p.iter_elements().next().unwrap().as_char()
    })(s)
}

pub fn digit<I>(s: I) -> IResult<I, char>
    where
        I: Slice<RangeFrom<usize>> + InputIter + InputTake + InputLength,
        <I as InputIter>::Item: AsChar,
{
    map(take_while_m_n(1, 1, |c:<I as InputIter>::Item| {
        c.is_dec_digit()
    }),|p:I|{
        p.iter_elements().next().unwrap().as_char()
    })(s)
}

pub fn under_score<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('_')(s)
}

pub fn space<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char(' ')(s)
}

pub fn eol<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    newline(s)
}

pub fn cr<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('\r')(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::character::{alphabet, cr, digit, eol, space, under_score};

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
    fn test_space() {
        assert_eq!(space(" "), Ok(("", ' ')))
    }

    #[test]
    fn test_eol() {
        assert_eq!(eol("\n"), Ok(("", '\n')))
    }

    #[test]
    fn test_cr() {
        assert_eq!(cr("\r"), Ok(("", '\r')))
    }
}
