use crate::parser::Span;
use nom::IResult;
use std::fmt::Debug;

pub(crate) fn check<'a, T, P>(source: &'a str, parser: P, excepted: T)
where
    P: Fn(Span<'a>) -> IResult<Span<'a>, T>,
    T: Debug + PartialEq,
    IResult<Span<'a>, T>: Debug,
{
    let result = parser(Span::<'a>::from(source));
    if let Ok((_, actual)) = result {
        assert_eq!(actual, excepted);
    } else {
        panic!("{:?}", result)
    }
}
