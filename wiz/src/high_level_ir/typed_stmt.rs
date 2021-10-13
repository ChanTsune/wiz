use crate::high_level_ir::typed_decl::TypedDecl;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedStmt {
    Expr(TypedExpr),
    Decl(TypedDecl),
    Assignment(TypedAssignmentStmt),
    Loop(TypedLoopStmt),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedAssignmentStmt {
    Assignment(TypedAssignment),
    AssignmentAndOperation(TypedAssignmentAndOperation),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedAssignment {
    pub(crate) target: TypedExpr,
    pub(crate) value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedAssignmentAndOperation {
    pub(crate) target: TypedExpr,
    pub(crate) operator: TypedAssignmentAndOperator,
    pub(crate) value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedAssignmentAndOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedLoopStmt {
    While(TypedWhileLoopStmt),
    For(TypedForStmt),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedWhileLoopStmt {
    pub(crate) condition: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedForStmt {
    pub(crate) values: Vec<String>,
    pub(crate) iterator: TypedExpr,
    pub(crate) block: TypedBlock,
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
