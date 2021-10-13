use super::node::SyntaxNode;
use crate::syntax::block::BlockSyntax;
use crate::syntax::decl::Decl;
use crate::syntax::expr::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
    Assignment(AssignmentStmt),
    Loop(LoopStmt),
}

impl SyntaxNode for Stmt {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AssignmentStmt {
    Assignment(AssignmentSyntax),
    AssignmentAndOperator(AssignmentAndOperatorSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub(crate) target: Expr,
    pub(crate) value: Expr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssignmentAndOperatorSyntax {
    pub(crate) target: Expr,
    pub(crate) operator: String,
    pub(crate) value: Expr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While(WhileLoopSyntax),
    For {
        values: Vec<String>,
        iterator: Expr,
        block: BlockSyntax,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WhileLoopSyntax {
    pub(crate) condition: Expr,
    pub(crate) block: BlockSyntax,
}
