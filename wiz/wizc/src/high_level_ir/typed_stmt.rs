use crate::high_level_ir::typed_decl::TypedDeclKind;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_type::TypedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedDeclKind),
    Assignment(TypedAssignmentStmt),
    Loop(TypedLoopStmt),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedAssignmentStmt {
    Assignment(TypedAssignment),
    AssignmentAndOperation(TypedAssignmentAndOperation),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedAssignment {
    pub(crate) target: TypedExpr,
    pub(crate) value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedAssignmentAndOperation {
    pub(crate) target: TypedExpr,
    pub(crate) operator: TypedAssignmentAndOperator,
    pub(crate) value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedAssignmentAndOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedLoopStmt {
    While(TypedWhileLoopStmt),
    For(TypedForStmt),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedWhileLoopStmt {
    pub(crate) condition: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedForStmt {
    pub(crate) values: Vec<String>,
    pub(crate) iterator: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
