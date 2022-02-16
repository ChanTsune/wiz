use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ModifiersSyntax {
    pub modifiers: Vec<TokenSyntax>,
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
