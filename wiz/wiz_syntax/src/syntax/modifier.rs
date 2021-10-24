use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ModifiersSyntax {
    pub modifiers: Vec<TokenSyntax>,
}

impl ModifiersSyntax {
    pub fn new() -> Self {
        Self { modifiers: vec![] }
    }
}

impl Default for ModifiersSyntax {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for ModifiersSyntax
where
    T: ToString,
{
    fn from(modifiers: Vec<T>) -> Self {
        Self {
            modifiers: modifiers.into_iter().map(TokenSyntax::from).collect(),
        }
    }
}
