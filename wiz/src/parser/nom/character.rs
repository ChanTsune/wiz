use nom::IResult;
use nom::character::complete::{one_of, char};
use nom::combinator::map;

pub fn alphabet(s: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)
}

pub fn digit(s: &str) -> IResult<&str, char> {
    one_of("0123456789")(s)
}

pub fn under_score(s: &str) -> IResult<&str, char> {
    char('_')(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::nom::character::alphabet;

    #[test]
    fn test_alphabet() {
        assert_eq!(alphabet("abc"), Ok(("bc", 'a')));
    }
}
