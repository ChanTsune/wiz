use crate::middle_level_ir::ml_decl::MLDecl;
use crate::middle_level_ir::ml_expr::MLExpr;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLStmt {
    Expr(MLExpr),
    Decl(MLDecl),
    Assignment(MLAssignmentStmt),
    Loop(MLLoopStmt),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLAssignmentStmt {
    pub(crate) target: String,
    pub(crate) value: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLLoopStmt {
    pub(crate) condition: MLExpr,
    pub(crate) block: MLBlock,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLBlock {
    pub(crate) body: Vec<MLStmt>,
}
