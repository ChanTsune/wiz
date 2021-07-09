use crate::middle_level_ir::ml_decl::MLDecl;
use crate::middle_level_ir::ml_expr::MLExpr;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLStmt {
    Expr(MLExpr),
    Decl(MLDecl),
    Assignment,
    Loop,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLBlock {
    pub(crate) body: Vec<MLStmt>,
}
