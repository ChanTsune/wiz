use crate::parser::Parser;
use wiz_mir_syntax::span::Span;
use wiz_mir_syntax::syntax::File;
use wiz_mir_syntax::token::TokenStream;

#[test]
fn test_parse() {
    let mut parser = Parser::from(TokenStream::default());
    let file = parser.parse();
    assert_eq!(
        file,
        Ok(File {
            attrs: vec![],
            items: vec![],
            span: Span::new(0, 0)
        })
    )
}
