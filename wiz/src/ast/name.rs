use std::fmt;
use crate::ast::node::Node;
use crate::ast::expr::Expr;

#[derive(fmt::Debug)]
pub struct Name {

}

impl Node for Name {}
impl Expr for Name {}
