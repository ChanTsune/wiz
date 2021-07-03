use nom::IResult;
use nom::combinator::map;
use nom::bytes::complete::tag;

pub fn fun_keyword(s: &str) -> IResult<&str, &str> {
    tag("fun")(s)
}

pub fn where_keyword(s: &str) -> IResult<&str, &str> {
    tag("where")(s)
}

pub fn var_keyword(s: &str) -> IResult<&str, &str> {
    tag("var")(s)
}

pub fn val_keyword(s: &str) -> IResult<&str, &str> {
    tag("val")(s)
}

pub fn while_keyword(s: &str) -> IResult<&str, &str> {
    tag("while")(s)
}

pub fn if_keyword(s: &str) -> IResult<&str, &str> {
    tag("if")(s)
}

pub fn else_keyword(s: &str) -> IResult<&str, &str> {
    tag("else")(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::nom::keywords::{fun_keyword, where_keyword, var_keyword, val_keyword, while_keyword, if_keyword, else_keyword};

    #[test]
    fn test_fun_keyword() {
        assert_eq!(fun_keyword("fun"), Ok(("", "fun")))
    }

    #[test]
    fn test_where_keyword() {
        assert_eq!(where_keyword("where"), Ok(("", "where")))
    }

    #[test]
    fn test_var_keyword() {
        assert_eq!(var_keyword("var"), Ok(("", "var")))
    }

    #[test]
    fn test_val_keyword() {
        assert_eq!(val_keyword("val"), Ok(("", "val")))
    }

    #[test]
    fn test_while_keyword() {
        assert_eq!(while_keyword("while"), Ok(("", "while")))
    }

    #[test]
    fn test_if_keyword() {
        assert_eq!(if_keyword("if"), Ok(("", "if")))
    }

    #[test]
    fn test_else_keyword() {
        assert_eq!(else_keyword("else"), Ok(("", "else")))
    }
}
