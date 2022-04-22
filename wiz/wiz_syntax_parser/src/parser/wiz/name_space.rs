use crate::parser::wiz::lexical_structure::{identifier, token};
use wiz_syntax::syntax::name_space::{NameSpaceElementSyntax, NameSpaceSyntax};
use wiz_syntax::syntax::token::TokenSyntax;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{AsChar, Compare, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::RangeFrom;

pub fn name_space<I>(s: I) -> IResult<I, NameSpaceSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(many1(name_space_element), |elements| NameSpaceSyntax {
        leading_trivia: Default::default(),
        elements,
        trailing_trivia: Default::default(),
    })(s)
}

pub fn name_space_element<I>(s: I) -> IResult<I, NameSpaceElementSyntax>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + InputTake
        + InputLength
        + Clone
        + ToString
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
{
    map(tuple((identifier, token("::"))), |(i, separator)| {
        NameSpaceElementSyntax {
            name: TokenSyntax::from(i),
            separator,
        }
    })(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::name_space::{name_space, name_space_element};
    use wiz_syntax::syntax::name_space::{NameSpaceElementSyntax, NameSpaceSyntax};

    #[test]
    fn test_name_space_element() {
        assert_eq!(
            name_space_element("name::"),
            Ok(("", NameSpaceElementSyntax::from("name")))
        );
    }

    #[test]
    fn test_name_space() {
        assert_eq!(
            name_space("a::b::"),
            Ok(("", NameSpaceSyntax::from(vec!["a", "b"])))
        );
    }
}
