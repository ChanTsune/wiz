use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{Compare, IResult, InputTake};

fn simple_member_access_operator<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag(".")(s)
}

fn safe_member_access_operator<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("?.")(s)
}

pub fn member_access_operator<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str> + Clone,
{
    alt((simple_member_access_operator, safe_member_access_operator))(s)
}

pub fn assignment_operator<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str>,
{
    tag("=")(s)
}

pub fn prefix_operator<I>(s: I) -> IResult<I, I>
where
    I: InputTake + Compare<&'static str> + Clone,
{
    alt((tag("+"), tag("-"), tag("!"), tag("*"), tag("&")))(s)
}

pub fn postfix_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str>,
{
    tag("!")(s)
}


pub fn conjunction_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str>,
{
    tag("&&")(s)
}

pub fn disjunction_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str>,
{
    tag("||")(s)
}


/*
<equality_operator> ::= "==" | "!="
*/
pub fn equality_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str> + Clone,
{
    alt((tag("=="),
        tag("!="),
    ))(s)
}

/*
<comparison_operator> ::= "<"  | ">"  | "<="  | ">="
*/
pub fn comparison_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str> + Clone,
{
    alt((
        tag("<="),
        tag(">="),
        tag("<"),
        tag(">"),
    ))(s)
}

/*
<range_operator> ::= "..." || "..<"
*/
pub fn range_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str> + Clone,
{
    alt((
        tag("..."),
        tag("..<")
    ))(s)
}

/*
<additive_operator> ::= "+" | "-"
*/
pub fn additive_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str> + Clone,
{
    alt((tag("+"), tag("-")))(s)
}

/*
<multiplicative_operator> ::= "*" | "/" | "%"
*/
pub fn multiplicative_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str> + Clone,
{
    alt((tag("*"), tag("/"), tag("%")))(s)
}


pub fn elvis_operator<I>(s: I) -> IResult<I, I>
    where
        I: InputTake + Compare<&'static str>
{
    tag(":?")(s)
}


#[cfg(test)]
mod tests {
    use crate::parser::wiz::operators::{assignment_operator, member_access_operator, prefix_operator, conjunction_operator, equality_operator, disjunction_operator, postfix_operator, comparison_operator, elvis_operator, range_operator, additive_operator, multiplicative_operator};

    #[test]
    fn test_member_access_operator() {
        assert_eq!(member_access_operator("."), Ok(("", ".")));
        assert_eq!(member_access_operator("?."), Ok(("", "?.")));
    }

    #[test]
    fn test_assignment_operator() {
        assert_eq!(assignment_operator("="), Ok(("", "=")));
    }

    #[test]
    fn test_prefix_operator() {
        assert_eq!(prefix_operator("+"), Ok(("", "+")));
        assert_eq!(prefix_operator("-"), Ok(("", "-")));
        assert_eq!(prefix_operator("!"), Ok(("", "!")));
        assert_eq!(prefix_operator("*"), Ok(("", "*")));
        assert_eq!(prefix_operator("&"), Ok(("", "&")));
    }

    #[test]
    fn test_postfix_operator() {
        assert_eq!(postfix_operator("!"), Ok(("","!")));
    }

    #[test]
    fn test_conjunction_operator() {
        assert_eq!(conjunction_operator("&&"), Ok(("", "&&")));
    }

    #[test]
    fn test_disjunction_operator() {
        assert_eq!(disjunction_operator("||"), Ok(("", "||")));
    }

    #[test]
    fn test_equality_operator() {
        assert_eq!(equality_operator("=="), Ok(("", "==")));
        assert_eq!(equality_operator("!="), Ok(("", "!=")));
    }

    #[test]
    fn test_comparison_operator() {
        assert_eq!(comparison_operator("<="), Ok(("", "<=")));
        assert_eq!(comparison_operator(">="), Ok(("", ">=")));
        assert_eq!(comparison_operator("<"), Ok(("", "<")));
        assert_eq!(comparison_operator(">"), Ok(("", ">")));
    }

    #[test]
    fn test_range_operator() {
        assert_eq!(range_operator("..."), Ok(("", "...")));
        assert_eq!(range_operator("..<"), Ok(("", "..<")));
    }

    #[test]
    fn test_additive_operator() {
        assert_eq!(additive_operator("+"), Ok(("", "+")));
        assert_eq!(additive_operator("-"), Ok(("", "-")));
    }

    #[test]
    fn test_multiplicative_operator() {
        assert_eq!(multiplicative_operator("*"), Ok(("", "*")));
        assert_eq!(multiplicative_operator("/"), Ok(("", "/")));
        assert_eq!(multiplicative_operator("%"), Ok(("", "%")));
    }

    #[test]
    fn test_elvis_operator() {
        assert_eq!(elvis_operator(":?"), Ok(("", ":?")));
    }
}
