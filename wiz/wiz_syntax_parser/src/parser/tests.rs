use crate::parser::Span;
use nom::IResult;
use std::fmt::Debug;
use wiz_session::ParseSession;

pub fn check<'a, T, P>(source: &'a str, parser: P, excepted: T)
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

pub fn check_with_session<'a, T, P>(source: &'a str, parser: P, excepted: T)
where
    P: Fn(&ParseSession, Span<'a>) -> IResult<Span<'a>, T>,
    T: Debug + PartialEq,
    IResult<Span<'a>, T>: Debug,
{
    let session = ParseSession::default();
    let result = parser(&session, Span::<'a>::from(source));
    if let Ok((_, actual)) = result {
        assert_eq!(actual, excepted);
    } else {
        panic!("{:?}", result)
    }
}
