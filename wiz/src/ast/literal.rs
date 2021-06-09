use crate::ast::expr::Expr;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Literal {
    IntegerLiteral {
        value: String
    },
    FloatingPointLiteral {
        value: String
    },
    StringLiteral {
        value: String
    },
    NullLiteral,
}

impl Node for Literal {}
