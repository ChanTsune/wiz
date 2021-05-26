use std::fmt;
use crate::ast::expr::Expr;
use crate::ast::node::Node;

#[derive(fmt::Debug)]
struct BinOp {
    left: Box<dyn Expr>,
    kind: Box<str>,
    right: Box<dyn Expr>
}

impl Node for BinOp {

}
impl Expr for BinOp {

}