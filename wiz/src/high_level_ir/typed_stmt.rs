use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedDecl),
    Assignment(TypedAssignmentStmt),
    Loop(TypedLoopStmt),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedAssignmentStmt {
    Assignment(TypedAssignment)
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedAssignment {
    pub(crate) target: TypedExpr,
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

impl TypedBlock {
    pub fn type_(&self) -> Option<TypedType> {
        if let Some(stmt) = self.body.last() {
            match stmt {
                TypedStmt::Expr(e) => e.type_(),
                _ => None,
            }
        } else {
            None
        }
    }
}
