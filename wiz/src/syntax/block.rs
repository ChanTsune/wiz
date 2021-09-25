use crate::syntax::node::SyntaxNode;
use crate::syntax::stmt::Stmt;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Block {
    pub(crate) body: Vec<Stmt>,
}

impl SyntaxNode for Block {}
