use crate::syntax::block::BlockSyntax;
use crate::syntax::expression::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub condition: Expr,
    pub block: BlockSyntax,
}
