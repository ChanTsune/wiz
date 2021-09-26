use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

pub fn simple_member_access_operator(s: &str) -> IResult<&str, &str> {
    tag(".")(s)
}

pub fn safe_member_access_operator(s: &str) -> IResult<&str, &str> {
    tag("?.")(s)
}

pub fn member_access_operator(s: &str) -> IResult<&str, &str> {
    alt((simple_member_access_operator, safe_member_access_operator))(s)
}

pub fn assignment_operator(s: &str) -> IResult<&str, &str> {
    tag("=")(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::operators::member_access_operator;

    #[test]
    fn test_member_access_operator() {
        assert_eq!(member_access_operator("."), Ok(("", ".")));
        assert_eq!(member_access_operator("?."), Ok(("", "?.")));
    }
}
