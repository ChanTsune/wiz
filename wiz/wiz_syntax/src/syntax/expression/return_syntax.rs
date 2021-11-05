use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ReturnSyntax {
    pub return_keyword: TokenSyntax,
    pub value: Option<Box<Expr>>,
}

impl Syntax for ReturnSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            return_keyword: self.return_keyword.with_leading_trivia(trivia),
            value: self.value,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.value {
            None => Self {
                return_keyword: self.return_keyword.with_trailing_trivia(trivia),
                value: None,
            },
            Some(value) => Self {
                return_keyword: self.return_keyword,
                value: Some(Box::new(value.with_trailing_trivia(trivia))),
            },
        }
    }
}
