use nom::bytes::complete::tag;
use nom::IResult;

pub fn struct_keyword(s: &str) -> IResult<&str, &str> {
    tag("struct")(s)
}

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

pub fn return_keyword(s: &str) -> IResult<&str, &str> {
    tag("return")(s)
}

pub fn init_keyword(s: &str) -> IResult<&str, &str> {
    tag("init")(s)
}

pub fn use_keyword(s: &str) -> IResult<&str, &str> {
    tag("use")(s)
}

pub fn as_keyword(s: &str) -> IResult<&str, &str> {
    tag("as")(s)
}

pub fn self_keyword(s: &str) -> IResult<&str, &str> {
    tag("self")(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::keywords::{
        as_keyword, else_keyword, fun_keyword, if_keyword, init_keyword, return_keyword,
        self_keyword, struct_keyword, use_keyword, val_keyword, var_keyword, where_keyword,
        while_keyword,
    };

    #[test]
    fn test_struct_keyword() {
        assert_eq!(struct_keyword("struct"), Ok(("", "struct")))
    }

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

    #[test]
    fn test_return_keyword() {
        assert_eq!(return_keyword("return"), Ok(("", "return")))
    }

    #[test]
    fn test_init_keyword() {
        assert_eq!(init_keyword("init"), Ok(("", "init")))
    }

    #[test]
    fn test_use_keyword() {
        assert_eq!(use_keyword("use"), Ok(("", "use")))
    }

    #[test]
    fn test_as_keyword() {
        assert_eq!(as_keyword("as"), Ok(("", "as")))
    }

    #[test]
    fn test_self_keyword() {
        assert_eq!(self_keyword("self"), Ok(("", "self")))
    }
}
