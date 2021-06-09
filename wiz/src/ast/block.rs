use crate::ast::stmt::Stmt;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub(crate) body: Vec<Stmt>
}

impl Node for Block {

}