use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ForLoopSyntax {
    pub for_keyword: TokenSyntax,
    pub values: Vec<TokenSyntax>,
    pub in_keyword: TokenSyntax,
    pub iterator: Expr,
    pub block: BlockSyntax,
}

impl Syntax for ForLoopSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            for_keyword: self.for_keyword.with_leading_trivia(trivia),
            values: self.values,
            in_keyword: self.in_keyword,
            iterator: self.iterator,
            block: self.block,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            for_keyword: self.for_keyword,
            values: self.values,
            in_keyword: self.in_keyword,
            iterator: self.iterator,
            block: self.block.with_trailing_trivia(trivia),
        }
    }
}
