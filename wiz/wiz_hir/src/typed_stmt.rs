use crate::typed_decl::TypedDecl;
use crate::typed_expr::TypedExprKind;
use crate::typed_type::TypedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedStmt {
    Expr(TypedExprKind),
    Decl(TypedDecl),
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
    pub target: TypedExprKind,
    pub value: TypedExprKind,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedAssignmentAndOperation {
    pub target: TypedExprKind,
    pub operator: TypedAssignmentAndOperator,
    pub value: TypedExprKind,
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
    pub condition: TypedExprKind,
    pub block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedForStmt {
    pub values: Vec<String>,
    pub iterator: TypedExprKind,
    pub block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedBlock {
    pub body: Vec<TypedStmt>,
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
