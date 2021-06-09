use crate::ast::block::Block;
use crate::ast::expr::Expr;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum FunBody {
    Block{
        block: Block
    },
    Expr {
        expr: Expr
    }
}