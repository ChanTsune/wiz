use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenSyntax {
    pub leading_trivia: Trivia,
    pub token: String,
    pub trailing_trivia: Trivia,
}

impl TokenSyntax {
    pub fn new(token: String) -> Self {
        Self {
            leading_trivia: Trivia::new(),
            token,
            trailing_trivia: Trivia::new(),
        }
    }
}

impl ToString for TokenSyntax {
    fn to_string(&self) -> String {
        self.leading_trivia.to_string() + &*self.token + &*self.trailing_trivia.to_string()
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
