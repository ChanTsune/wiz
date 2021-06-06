use nom::IResult;
use nom::combinator::map;
use nom::bytes::complete::tag;

pub fn fun_keyword(s: &str) -> IResult<&str, String> {
    map(tag("fun"), |a:&str|{
        a.to_string()
    })(s)
}

pub fn where_keyword(s: &str) -> IResult<&str, String> {
    map(tag("where"), |a:&str| {
        a.to_string()
    })(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::nom::keywords::{fun_keyword, where_keyword};

    #[test]
    fn test_fun_keyword() {
        assert_eq!(fun_keyword("fun"), Ok(("", "fun".to_string())))
    }

    #[test]
    fn test_where_keyword() {
        assert_eq!(where_keyword("where"), Ok(("", "where".to_string())))
    }
}
