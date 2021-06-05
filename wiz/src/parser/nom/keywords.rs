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
