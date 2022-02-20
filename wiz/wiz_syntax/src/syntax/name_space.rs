use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct NameSpaceSyntax {
    pub leading_trivia: Trivia,
    pub elements: Vec<NameSpaceElementSyntax>,
    pub trailing_trivia: Trivia,
}

impl Syntax for NameSpaceSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: trivia,
            elements: self.elements,
            trailing_trivia: self.trailing_trivia,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: self.leading_trivia,
            elements: self.elements,
            trailing_trivia: trivia,
        }
    }
}

impl<T> From<Vec<T>> for NameSpaceSyntax
where
    T: ToString,
{
    fn from(names: Vec<T>) -> Self {
        Self {
            leading_trivia: Default::default(),
            elements: names
                .into_iter()
                .map(NameSpaceElementSyntax::from)
                .collect(),
            trailing_trivia: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameSpaceElementSyntax {
    pub name: TokenSyntax,
    pub separator: TokenSyntax,
}

impl<T> From<T> for NameSpaceElementSyntax
where
    T: ToString,
{
    fn from(name: T) -> Self {
        Self {
            name: TokenSyntax::from(name),
            separator: TokenSyntax::from("::"),
        }
    }
}
