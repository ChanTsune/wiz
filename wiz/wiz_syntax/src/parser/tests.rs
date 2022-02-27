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

#[test]
fn test_get_location() {
    use super::*;
    let location = Location::new(1, 0);
    assert_eq!(location.offset(), 1);
    assert_eq!(location.line(), 0);
}

mod get_line_offset {
    use crate::parser::*;

    #[test]
    fn one_line() {
        let location = Location::new(1, 1);
        assert_eq!(get_line_offset("a\n2", &location), 1);
    }

    #[test]
    fn one_line_second() {
        let location = Location::new(3, 2);
        assert_eq!(get_line_offset("a\n2", &location), 1);
    }

    #[test]
    fn test_three_lines() {
        let location = Location::new(7, 4);
        assert_eq!(get_line_offset("a\n2\n\n33", &location), 2);
    }
}
