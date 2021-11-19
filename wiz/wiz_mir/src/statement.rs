mod return_statement;

pub use self::return_statement::MLReturn;
use crate::expr::{MLBlock, MLExpr};
use crate::format::Formatter;
use crate::ml_decl::MLVar;
use crate::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLStmt {
    Expr(MLExpr),
    Var(MLVar),
    Assignment(MLAssignmentStmt),
    Loop(MLLoopStmt),
    Return(MLReturn),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLAssignmentStmt {
    pub target: MLExpr,
    pub value: MLExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLLoopStmt {
    pub condition: MLExpr,
    pub block: MLBlock,
}

impl MLNode for MLStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLStmt::Expr(e) => e.fmt(f),
            MLStmt::Var(d) => d.fmt(f),
            MLStmt::Assignment(a) => a.fmt(f),
            MLStmt::Loop(l) => l.fmt(f),
            MLStmt::Return(r) => r.fmt(f),
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
