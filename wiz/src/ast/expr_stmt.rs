use std::fmt;
use crate::ast::node::Node;
use crate::ast::stmt::Stmt;

#[derive(fmt::Debug)]
pub struct ExprStmt {

}

impl Node for ExprStmt {}
impl Stmt for ExprStmt {}