use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub condition: Expr,
    pub block: BlockSyntax,
}

impl Syntax for WhileLoopSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            condition: self.condition.with_leading_trivia(trivia),
            block: self.block,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            condition: self.condition,
            block: self.block.with_leading_trivia(trivia),
        }
    }
}
