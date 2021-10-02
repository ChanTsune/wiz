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


#[cfg(test)]
mod tests {
    use crate::parser::wiz::operators::{assignment_operator, member_access_operator, prefix_operator};

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
}
