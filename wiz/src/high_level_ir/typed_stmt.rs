use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_expr::TypedExpr;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedDecl),
    Assignment(TypedAssignmentStmt),
    Loop(TypedLoopStmt),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedAssignmentStmt {
    pub(crate) target: String,
    pub(crate) value: TypedExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedLoopStmt {
    While(TypedWhileLoopStmt),
    For(TypedForStmt),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedWhileLoopStmt {
    pub(crate) condition: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedForStmt {
    pub(crate) values: Vec<String>,
    pub(crate) iterator: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedBlock {
    pub(crate) body: Vec<TypedStmt>,
}
