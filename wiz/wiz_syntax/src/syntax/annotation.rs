use crate::syntax::list::{ElementSyntax, ListSyntax};
use crate::syntax::token::TokenSyntax;

pub trait Annotatable {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self;
}

pub type AnnotationsSyntax = ListSyntax<TokenSyntax>;

pub type Annotation = ElementSyntax<TokenSyntax>;
