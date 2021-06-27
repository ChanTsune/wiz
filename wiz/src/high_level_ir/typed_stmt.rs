use std::fmt;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_decl::TypedDecl;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedDecl),
    Assignment,
    Loop,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedBlock {
    pub(crate) body: Vec<TypedStmt>
}
