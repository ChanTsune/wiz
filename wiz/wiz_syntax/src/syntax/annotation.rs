use crate::syntax::token::TokenSyntax;

pub trait Annotatable {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AnnotationsSyntax {
    pub open: TokenSyntax,
    pub annotations: Vec<Annotation>,
    pub close: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Annotation {
    pub name: TokenSyntax,
    pub trailing_comma: Option<TokenSyntax>,
}
