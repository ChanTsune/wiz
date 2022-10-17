use crate::typed_decl::TypedTopLevelDecl;
use crate::typed_expr::TypedExpr;
use crate::typed_type::TypedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedTopLevelDecl),
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
    pub target: TypedExpr,
    pub value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedAssignmentAndOperation {
    pub target: TypedExpr,
    pub operator: TypedAssignmentAndOperator,
    pub value: TypedExpr,
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
    pub condition: TypedExpr,
    pub block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedForStmt {
    pub values: Vec<String>,
    pub iterator: TypedExpr,
    pub block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedBlock {
    pub body: Vec<TypedStmt>,
}

impl TypedBlock {
    pub fn type_(&self) -> Option<TypedType> {
        if let Some(TypedStmt::Expr(e)) = self.body.last() {
            e.ty.clone()
        } else {
            None
        }
    }
}
