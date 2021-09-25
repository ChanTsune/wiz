use crate::ast::node::Node;
use crate::ast::token::TokenSyntax;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum LiteralSyntax {
    Integer { value: String },
    FloatingPoint { value: String },
    String { value: String },
    Boolean(TokenSyntax),
    Null,
}

impl Node for LiteralSyntax {}
