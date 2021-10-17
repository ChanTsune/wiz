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

pub fn annotations<I>(s: I) -> IResult<I, AnnotationsSyntax>
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
            whitespace0,
            identifier,
            whitespace0,
            many0(tuple((comma, whitespace0, identifier, whitespace0))),
            opt(comma),
            whitespace0,
            tag("]"),
        )),
        |(open, ows, e, ws, v, c, cws, close): (I, _, _, _, _, _, _, I)| {
            let mut cmas = vec![];
            let mut lwss = vec![ws];
            let mut expers = vec![e];
            let mut twss = vec![];
            for (cma, tws, e, lws) in v.into_iter() {
                cmas.push(cma);
                twss.push(tws);
                expers.push(e);
                lwss.push(lws);
            }
            match c {
                None => {}
                Some(c) => cmas.push(c),
            }
            let mut elements = vec![];
            for (idx, e) in expers.into_iter().enumerate() {
                let mut trailing_comma = TokenSyntax::new(match cmas.get(idx) {
                    None => String::new(),
                    Some(c) => c.to_string(),
                });
                match lwss.get(idx) {
                    None => {}
                    Some(e) => {
                        trailing_comma = trailing_comma.with_leading_trivia(e.clone());
                    }
                };
                match twss.get(idx) {
                    None => {}
                    Some(e) => {
                        trailing_comma = trailing_comma.with_trailing_trivia(e.clone());
                    }
                }
                elements.push(Annotation {
                    name: TokenSyntax::new(e.to_string()),
                    trailing_comma,
                })
            }
            AnnotationsSyntax {
                open: TokenSyntax::new(open.to_string()).with_trailing_trivia(ows),
                annotations: elements,
                close: TokenSyntax::new(close.to_string()).with_leading_trivia(cws),
            }
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use crate::parser::wiz::annotation::annotations;
    use crate::syntax::annotation::{Annotation, AnnotationsSyntax};
    use crate::syntax::token::TokenSyntax;

    #[test]
    fn test_annotations() {
        assert_eq!(
            annotations("#[no_mangle]"),
            Ok((
                "",
                AnnotationsSyntax {
                    open: TokenSyntax::new("#[".to_string()),
                    annotations: vec![Annotation {
                        name: TokenSyntax::new("no_mangle".to_string()),
                        trailing_comma: TokenSyntax::new("".to_string()),
                    }],
                    close: TokenSyntax::new("]".to_string()),
                }
            ))
        );
    }
}
