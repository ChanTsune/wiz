use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    Integer { value: String },
    FloatingPoint { value: String },
    String { value: String },
    Boolean { value: String },
    Null,
}

impl Node for Literal {}
