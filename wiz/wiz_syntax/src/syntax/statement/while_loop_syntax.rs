use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub while_keyword: TokenSyntax,
    pub condition: Expr,
    pub block: BlockSyntax,
}

impl Syntax for WhileLoopSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            while_keyword: self.while_keyword.with_leading_trivia(trivia),
            condition: self.condition,
            block: self.block,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            while_keyword: self.while_keyword,
            condition: self.condition,
            block: self.block.with_leading_trivia(trivia),
        }
    }
}
