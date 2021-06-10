use crate::ast::node::Node;
use crate::ast::literal::Literal;
use std::fmt;
use crate::ast::type_name::TypeName;
use crate::ast::stmt::Stmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
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
        args: Vec<CallArg>,
        tailing_lambda: Option<Lambda>
    },
    If {
        // TODO
    },
    When {
        // TODO
    },
    Lambda {
        lambda: Lambda
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

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<Expr>,
    pub(crate) is_vararg: bool
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Lambda {
    pub(crate) stmts: Vec<Stmt>
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum PostfixSuffix {
    Operator {
        kind: String
    },
    TypeArgumentSuffix {
        types: Vec<TypeName>
    },
    CallSuffix {
        args: Vec<CallArg>,
        tailing_lambda: Option<Lambda>
    },
    IndexingSuffix,
    NavigationSuffix,
}
