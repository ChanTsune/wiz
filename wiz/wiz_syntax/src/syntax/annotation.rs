use crate::syntax::list::ElementSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

pub trait Annotatable {
    fn with_annotation(self, a: AnnotationsSyntax) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AnnotationsSyntax {
    pub open: TokenSyntax,
    pub annotations: Vec<Annotation>,
    pub close: TokenSyntax,
}

impl Syntax for AnnotationsSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open.with_leading_trivia(trivia),
            annotations: self.annotations,
            close: self.close,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open,
            annotations: self.annotations,
            close: self.close.with_trailing_trivia(trivia),
        }
    }
}

pub type Annotation = ElementSyntax<TokenSyntax>;
