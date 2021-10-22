use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameSpaceSyntax {
    pub elements: Vec<NameSpaceElementSyntax>,
}

impl NameSpaceSyntax {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for NameSpaceSyntax {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for NameSpaceSyntax
where
    T: ToString,
{
    fn from(names: Vec<T>) -> Self {
        Self {
            elements: names
                .into_iter()
                .map(|n| NameSpaceElementSyntax::from(n))
                .collect(),
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
