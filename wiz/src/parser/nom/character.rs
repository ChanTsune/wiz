use nom::character::complete::{char, one_of};
use nom::{AsChar, IResult, InputIter, Slice};
use std::ops::RangeFrom;

pub fn alphabet(s: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)
}

pub fn digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789")(s)
}

pub fn under_score<I>(s: I) -> IResult<I, char>
where
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    char('_')(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::nom::character::{alphabet, digit, under_score};

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
}
