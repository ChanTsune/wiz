use crate::parser::wiz::character::comma;
use crate::parser::wiz::lexical_structure::{identifier, whitespace0};
use crate::syntax::annotation::{Annotation, AnnotationsSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::Syntax;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{AsChar, Compare, FindSubstring, IResult, InputIter, InputLength, InputTake, Slice};
use std::ops::{Range, RangeFrom};

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
            tag("#["),
            many0(tuple((whitespace0, identifier, whitespace0, comma))),
            whitespace0,
            opt(identifier),
            whitespace0,
            tag("]"),
        )),
        |(open, v, ws, a, tws, close): (I, _, _, _, _, I)| {
            let mut close = TokenSyntax::from(close);
            let mut annotations: Vec<_> = v
                .into_iter()
                .map(|(lws, a, rws, cma)| Annotation {
                    element: TokenSyntax::from(a).with_leading_trivia(lws),
                    trailing_comma: Some(TokenSyntax::from(cma).with_leading_trivia(rws)),
                })
                .collect();

            match a {
                None => {
                    close = close.with_leading_trivia(ws + tws);
                }
                Some(p) => {
                    annotations.push(Annotation {
                        element: TokenSyntax::from(p).with_leading_trivia(ws),
                        trailing_comma: None,
                    });
                    close = close.with_leading_trivia(tws);
                }
            };

            AnnotationsSyntax {
                open: TokenSyntax::from(open),
                elements: annotations,
                close,
            }
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::tests::check;
    use crate::parser::wiz::annotation::annotations_syntax;
    use crate::syntax::annotation::{Annotation, AnnotationsSyntax};
    use crate::syntax::token::TokenSyntax;

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
