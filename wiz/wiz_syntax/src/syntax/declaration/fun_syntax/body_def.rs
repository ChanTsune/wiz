use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::trivia::Trivia;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunBody {
    Block(BlockSyntax),
    Expr(Expr),
}

impl Syntax for FunBody {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            FunBody::Block(b) => FunBody::Block(b.with_leading_trivia(trivia)),
            FunBody::Expr(e) => FunBody::Expr(e.with_leading_trivia(trivia))
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            FunBody::Block(b) => FunBody::Block(b.with_trailing_trivia(trivia)),
            FunBody::Expr(e) => FunBody::Expr(e.with_trailing_trivia(trivia))
        }
    }
}
