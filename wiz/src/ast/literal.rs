use crate::ast::node::Node;
use std::fmt;
use crate::ast::token::TokenSyntax;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LiteralSyntax {
    Integer { value: String },
    FloatingPoint { value: String },
    String { value: String },
    Boolean(TokenSyntax),
    Null,
}

impl Node for LiteralSyntax {}
