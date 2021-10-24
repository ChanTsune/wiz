use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ForLoopSyntax {
    pub for_keyword: TokenSyntax,
    pub values: Vec<TokenSyntax>,
    pub in_keyword: TokenSyntax,
    pub iterator: Expr,
    pub block: BlockSyntax,
}
