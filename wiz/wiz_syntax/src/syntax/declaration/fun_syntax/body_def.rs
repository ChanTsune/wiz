use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunBody {
    Block(BlockSyntax),
    Expr(ExprFunBodySyntax),
}

impl Syntax for FunBody {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            FunBody::Block(b) => FunBody::Block(b.with_leading_trivia(trivia)),
            FunBody::Expr(e) => FunBody::Expr(e.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            FunBody::Block(b) => FunBody::Block(b.with_trailing_trivia(trivia)),
            FunBody::Expr(e) => FunBody::Expr(e.with_trailing_trivia(trivia)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExprFunBodySyntax {
    pub equal: TokenSyntax,
    pub expr: Expr,
}

impl Syntax for ExprFunBodySyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            equal: self.equal.with_leading_trivia(trivia),
            expr: self.expr,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            equal: self.equal,
            expr: self.expr.with_trailing_trivia(trivia),
        }
    }
}