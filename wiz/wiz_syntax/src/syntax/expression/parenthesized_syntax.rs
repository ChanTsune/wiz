use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParenthesizedExprSyntax {
    pub open_paren: TokenSyntax,
    pub expr: Box<Expr>,
    pub close_paren: TokenSyntax,
}

impl Syntax for ParenthesizedExprSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            open_paren: self.open_paren.with_leading_trivia(trivia),
            expr: self.expr,
            close_paren: self.close_paren,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            open_paren: self.open_paren,
            expr: self.expr,
            close_paren: self.close_paren.with_trailing_trivia(trivia),
        }
    }
}
