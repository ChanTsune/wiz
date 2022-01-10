use nom::bytes::complete::tag;
use nom::{Compare, IResult, InputTake};

pub fn struct_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("struct")(s)
}

pub fn fun_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("fun")(s)
}

pub fn where_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("where")(s)
}

pub fn var_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("var")(s)
}

pub fn val_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("val")(s)
}

pub fn extension_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("extension")(s)
}

pub fn protocol_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("protocol")(s)
}

pub fn while_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("while")(s)
}

pub fn for_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("for")(s)
}

pub fn if_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("if")(s)
}

pub fn else_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("else")(s)
}

pub fn return_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("return")(s)
}

pub fn init_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("init")(s)
}

pub fn deinit_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("deinit")(s)
}

pub fn use_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("use")(s)
}

pub fn as_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("as")(s)
}

pub fn in_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("in")(s)
}

pub fn self_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("self")(s)
}

pub fn true_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("true")(s)
}

pub fn false_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("false")(s)
}

pub fn extern_keyword<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("extern")(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::keywords::{
        as_keyword, deinit_keyword, else_keyword, extension_keyword, extern_keyword, false_keyword,
        for_keyword, fun_keyword, if_keyword, in_keyword, init_keyword, protocol_keyword,
        return_keyword, self_keyword, struct_keyword, true_keyword, use_keyword, val_keyword,
        var_keyword, where_keyword, while_keyword,
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
    fn test_extension_keyword() {
        assert_eq!(extension_keyword("extension"), Ok(("", "extension")))
    }

    #[test]
    fn test_protocol_keyword() {
        assert_eq!(protocol_keyword("protocol"), Ok(("", "protocol")))
    }

    #[test]
    fn test_while_keyword() {
        assert_eq!(while_keyword("while"), Ok(("", "while")))
    }

    #[test]
    fn test_for_keyword() {
        assert_eq!(for_keyword("for"), Ok(("", "for")))
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
    fn test_deinit_keyword() {
        assert_eq!(deinit_keyword("deinit"), Ok(("", "deinit")))
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
    fn test_in_keyword() {
        assert_eq!(in_keyword("in"), Ok(("", "in")))
    }

    #[test]
    fn test_self_keyword() {
        assert_eq!(self_keyword("self"), Ok(("", "self")))
    }

    #[test]
    fn test_true_keyword() {
        assert_eq!(true_keyword("true"), Ok(("", "true")))
    }

    #[test]
    fn test_false_keyword() {
        assert_eq!(false_keyword("false"), Ok(("", "false")))
    }

    #[test]
    fn test_extern_keyword() {
        assert_eq!(extern_keyword("extern"), Ok(("", "extern")));
    }
}
