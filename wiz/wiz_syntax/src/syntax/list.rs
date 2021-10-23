use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ListSyntax<E> where E: Syntax {
    pub open: TokenSyntax,
    pub elements: Vec<ElementSyntax<E>>,
    pub close: TokenSyntax,
}

impl<E> Syntax for ListSyntax<E> where E: Syntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open.with_leading_trivia(trivia),
            elements: self.elements,
            close: self.close
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open,
            elements: self.elements,
            close: self.close.with_trailing_trivia(trivia)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ElementSyntax<E> where E: Syntax {
    pub element: E,
    pub trailing_comma: Option<TokenSyntax>,
}

impl<E> Syntax for ElementSyntax<E> where E: Syntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            element: self.element.with_leading_trivia(trivia),
            trailing_comma: self.trailing_comma
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.trailing_comma {
            None =>
                Self {
                    element: self.element.with_trailing_trivia(trivia),
                    trailing_comma: None
                },
            Some(trailing_comma) =>
                Self {
                    element: self.element,
                    trailing_comma: Some(trailing_comma.with_trailing_trivia(trivia))
                },
        }
    }
}
