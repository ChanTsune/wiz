mod return_statement;

use crate::middle_level_ir::expr::{MLBlock, MLExpr};
use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_decl::MLVar;
use crate::middle_level_ir::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLStmt {
    Expr(MLExpr),
    Var(MLVar),
    Assignment(MLAssignmentStmt),
    Loop(MLLoopStmt),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLAssignmentStmt {
    pub(crate) target: MLExpr,
    pub(crate) value: MLExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLLoopStmt {
    pub(crate) condition: MLExpr,
    pub(crate) block: MLBlock,
}

impl MLNode for MLStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLStmt::Expr(e) => e.fmt(f),
            MLStmt::Var(d) => d.fmt(f),
            MLStmt::Assignment(a) => a.fmt(f),
            MLStmt::Loop(l) => l.fmt(f),
        }
    }
}

impl MLNode for MLAssignmentStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_str(" = ")?;
        self.value.fmt(f)
    }
}

impl MLNode for MLLoopStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("while (")?;
        self.condition.fmt(f)?;
        f.write_str(") ")?;
        self.block.fmt(f)
    }
}
