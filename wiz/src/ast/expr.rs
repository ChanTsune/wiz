use crate::ast::node::Node;
use crate::ast::literal::Literal;
use std::fmt;
use crate::ast::type_name::TypeName;

#[derive(fmt::Debug, Eq, PartialEq)]
pub enum Expr {
    Name {
        name: String
    },
    Literal {
        literal: Literal
    },
    BinOp {
        left: Box<Expr>,
        kind: String,
        right: Box<Expr>
    },
    UnaryOp {
        target: Box<Expr>,
        prefix: bool,
        kind: String,
    },
    Subscript {
        target: Box<Expr>,
        idx_or_key: Box<Expr>,
    },
    List {
        values: Vec<Expr>
    },
    Tuple {
        values: Vec<Expr>
    },
    Dict {
        // TODO
    },
    StringBuilder {
        // TODO
    },
    Call {
        target: Box<Expr>,
        // TODO
    },
    If {
        // TODO
    },
    When {
        // TODO
    },
    Lambda {
        // TODO
    },
    Return {
        // TODO
    },
    TypeCast {
        target: Box<Expr>,
        is_safe: bool,
        type_: TypeName
    }
}

impl Node for Expr {

}
