use crate::syntax::block::Block;
use crate::syntax::expr::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunBody {
    Block { block: Block },
    Expr { expr: Expr },
}
