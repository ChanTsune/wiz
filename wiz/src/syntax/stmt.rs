use super::node::Node;
use crate::syntax::block::Block;
use crate::syntax::decl::Decl;
use crate::syntax::expr::Expr;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Decl { decl: Decl },
    Expr { expr: Expr },
    Assignment(AssignmentStmt),
    Loop(LoopStmt),
}

impl Node for Stmt {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum AssignmentStmt {
    Assignment(AssignmentSyntax),
    AssignmentAndOperator(AssignmentAndOperatorSyntax),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct AssignmentSyntax {
    pub(crate) target: Expr,
    pub(crate) value: Expr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct AssignmentAndOperatorSyntax {
    pub(crate) target: Expr,
    pub(crate) operator: String,
    pub(crate) value: Expr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While {
        condition: Expr,
        block: Block,
    },
    For {
        values: Vec<String>,
        iterator: Expr,
        block: Block,
    },
}
