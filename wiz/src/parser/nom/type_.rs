use nom::IResult;
use crate::parser::nom::lexical_structure::identifier;

pub fn type_(s: &str) -> IResult<&str, String> {
    // TODO:
    identifier(s)
}
