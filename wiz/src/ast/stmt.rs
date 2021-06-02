use super::node::Node;
use std::fmt;
use crate::ast::decl::Decl;
use crate::ast::expr::Expr;

#[derive(fmt::Debug, Eq, PartialEq)]
pub enum Stmt {
    Decl {
        decl: Decl
    },
    Expr {
        expr: Expr
    }
}

impl Node for Stmt {

}