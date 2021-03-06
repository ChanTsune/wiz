use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TokenSyntax {
    pub leading_trivia: Trivia,
    token: String,
    pub trailing_trivia: Trivia,
}

impl TokenSyntax {
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

impl<T> From<T> for TokenSyntax
where
    T: ToString,
{
    fn from(token: T) -> Self {
        Self {
            leading_trivia: Trivia::default(),
            token: token.to_string(),
            trailing_trivia: Trivia::default(),
        }
    }
}

impl Syntax for TokenSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: self.leading_trivia + trivia,
            token: self.token,
            trailing_trivia: self.trailing_trivia,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: self.leading_trivia,
            token: self.token,
            trailing_trivia: self.trailing_trivia + trivia,
        }
    }
}
