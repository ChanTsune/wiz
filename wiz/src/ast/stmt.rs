use super::node::Node;
use std::fmt;
use crate::ast::decl::Decl;
use crate::ast::expr::Expr;
use crate::ast::block::Block;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Stmt {
    Decl {
        decl: Decl
    },
    Expr {
        expr: Expr
    },
    Assignment(AssignmentStmt),
    Loop(LoopStmt)
}

impl Node for Stmt {

}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct AssignmentStmt {
    pub(crate) target: String,
    pub(crate) value: Expr
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LoopStmt {
    While {
        condition: Expr,
        block: Block
    },
    DoWhile {
        condition: Expr,
        block: Block
    },
    For {
        values: Vec<String>,
        iterator: Expr,
        block: Block
    }
}
