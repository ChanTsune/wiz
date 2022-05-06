use crate::parser::wiz::character::comma;
use crate::parser::wiz::lexical_structure::{identifier, token, whitespace0};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::{Range, RangeFrom};
use wiz_syntax::syntax::annotation::{Annotation, AnnotationsSyntax};
use wiz_syntax::syntax::token::TokenSyntax;
use wiz_syntax::syntax::Syntax;

pub fn annotations_syntax<I>(s: I) -> IResult<I, AnnotationsSyntax>
where
    I: Slice<RangeFrom<usize>>
        + Slice<Range<usize>>
        + InputIter
        + Clone
        + InputLength
        + ToString
        + InputTake
        + FindSubstring<&'static str>
        + Compare<&'static str>,
    <I as InputIter>::Item: AsChar + Copy,
{
    map(
        tuple((
            token("#["),
            many0(tuple((whitespace0, identifier, whitespace0, comma))),
            opt(tuple((whitespace0, identifier))),
            whitespace0,
            token("]"),
        )),
        |(open, v, a, tws, close)| {
            let mut annotations: Vec<_> = v
                .into_iter()
                .map(|(lws, a, rws, cma)| Annotation {
                    element: TokenSyntax::from(a).with_leading_trivia(lws),
                    trailing_comma: Some(cma.with_leading_trivia(rws)),
                })
                .collect();

            if let Some((ws, p)) = a {
                annotations.push(Annotation {
                    element: TokenSyntax::from(p).with_leading_trivia(ws),
                    trailing_comma: None,
                });
            }

            AnnotationsSyntax {
                open,
                elements: annotations,
                close: close.with_leading_trivia(tws),
            }
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::check;
    use crate::parser::wiz::annotation::annotations_syntax;
    use wiz_syntax::syntax::annotation::{Annotation, AnnotationsSyntax};
    use wiz_syntax::syntax::token::TokenSyntax;

    #[test]
    fn test_annotations() {
        check(
            "#[no_mangle]",
            annotations_syntax,
            AnnotationsSyntax {
                open: TokenSyntax::from("#["),
                elements: vec![Annotation {
                    element: TokenSyntax::from("no_mangle"),
                    trailing_comma: None,
                }],
                close: TokenSyntax::from("]"),
            },
        );
    }
}
