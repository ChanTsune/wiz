use crate::syntax::node::Node;
use crate::syntax::token::TokenSyntax;
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
