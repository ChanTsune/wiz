use crate::parser::wiz::lexical_structure::token;
use crate::syntax::token::TokenSyntax;
use nom::bytes::complete::tag;
use nom::{Compare, IResult, InputTake};

pub fn struct_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("struct")(s)
}

pub fn fun_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("fun")(s)
}

pub fn where_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("where")(s)
}

pub fn var_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("var")(s)
}

pub fn val_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("val")(s)
}

pub fn extension_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("extension")(s)
}

pub fn protocol_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("protocol")(s)
}

pub fn while_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("while")(s)
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

pub fn return_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("return")(s)
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

pub fn self_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("self")(s)
}

pub fn true_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("true")(s)
}

pub fn false_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("false")(s)
}

pub fn extern_keyword<I>(s: I) -> IResult<I, TokenSyntax>
where
    I: InputTake + Compare<&'static str> + ToString,
{
    token("extern")(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::check;
    use crate::parser::wiz::keywords::{
        as_keyword, deinit_keyword, else_keyword, extension_keyword, extern_keyword, false_keyword,
        for_keyword, fun_keyword, if_keyword, in_keyword, init_keyword, protocol_keyword,
        return_keyword, self_keyword, struct_keyword, true_keyword, use_keyword, val_keyword,
        var_keyword, where_keyword, while_keyword,
    };
    use crate::syntax::token::TokenSyntax;

    #[test]
    fn test_struct_keyword() {
        check("struct", struct_keyword, TokenSyntax::from("struct"));
    }

    #[test]
    fn test_fun_keyword() {
        check("fun", fun_keyword, TokenSyntax::from("fun"));
    }

    #[test]
    fn test_where_keyword() {
        check("where", where_keyword, TokenSyntax::from("where"));
    }

    #[test]
    fn test_var_keyword() {
        check("var", var_keyword, TokenSyntax::from("var"));
    }

    #[test]
    fn test_val_keyword() {
        check("val", val_keyword, TokenSyntax::from("val"));
    }

    #[test]
    fn test_extension_keyword() {
        check(
            "extension",
            extension_keyword,
            TokenSyntax::from("extension"),
        );
    }

    #[test]
    fn test_protocol_keyword() {
        check("protocol", protocol_keyword, TokenSyntax::from("protocol"));
    }

    #[test]
    fn test_while_keyword() {
        check("while", while_keyword, TokenSyntax::from("while"));
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
        check("return", return_keyword, TokenSyntax::from("return"));
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
        check("self", self_keyword, TokenSyntax::from("self"));
    }

    #[test]
    fn test_true_keyword() {
        check("true", true_keyword, TokenSyntax::from("true"));
    }

    #[test]
    fn test_false_keyword() {
        check("false", false_keyword, TokenSyntax::from("false"));
    }

    #[test]
    fn test_extern_keyword() {
        check("extern", extern_keyword, TokenSyntax::from("extern"));
    }
}
