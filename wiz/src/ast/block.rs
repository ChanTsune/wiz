use crate::ast::stmt::Stmt;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Block {
    body: Vec<Stmt>
}

impl Node for Block {

}