use super::node::Node;
use crate::ast::block::Block;
use crate::ast::decl::Decl;
use crate::ast::expr::Expr;
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
pub struct AssignmentStmt {
    pub(crate) target: String,
    pub(crate) value: Expr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While {
        condition: Expr,
        block: Block,
    },
    DoWhile {
        condition: Expr,
        block: Block,
    },
    For {
        values: Vec<String>,
        iterator: Expr,
        block: Block,
    },
}
