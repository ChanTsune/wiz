use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenSyntax {
    pub(crate) leading_trivia: Trivia,
    pub(crate) token: String,
    pub(crate) trailing_trivia: Trivia,
}
