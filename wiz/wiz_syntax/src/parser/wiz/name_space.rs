use crate::parser::wiz::lexical_structure::identifier;
use crate::syntax::name_space::{NameSpaceElementSyntax, NameSpaceSyntax};
use crate::syntax::token::TokenSyntax;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
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
    map(many0(name_space_element), |elements| NameSpaceSyntax {
        elements,
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
    map(tuple((identifier, tag("::"))), |(i, sep): (_, I)| {
        NameSpaceElementSyntax {
            name: TokenSyntax::new(i),
            separator: TokenSyntax::new(sep.to_string()),
        }
    })(s)
}
