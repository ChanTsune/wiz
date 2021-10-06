use crate::syntax::token::TokenSyntax;

pub(crate) trait Annotatable {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AnnotationsSyntax {
    pub(crate) open: TokenSyntax,
    pub(crate) annotations: Vec<Annotation>,
    pub(crate) close: TokenSyntax
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Annotation {
    pub(crate) name: TokenSyntax,
    pub(crate) trailing_comma: TokenSyntax
}
