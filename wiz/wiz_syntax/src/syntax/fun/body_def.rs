use crate::syntax::block::BlockSyntax;
use crate::syntax::expr::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunBody {
    Block { block: BlockSyntax },
    Expr { expr: Expr },
}
