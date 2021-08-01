use crate::ast::block::Block;
use crate::ast::literal::Literal;
use crate::ast::node::Node;
use crate::ast::stmt::Stmt;
use crate::ast::type_name::TypeName;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Name(NameExprSyntax),
    Literal(Literal),
    BinOp {
        left: Box<Expr>,
        kind: String,
        right: Box<Expr>,
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
    Member {
        target: Box<Expr>,
        name: String,
        is_safe: bool,
    },
    List {
        values: Vec<Expr>,
    },
    Tuple {
        values: Vec<Expr>,
    },
    Dict {
        // TODO
    },
    StringBuilder {
        // TODO
    },
    Call(CallExprSyntax),
    If {
        condition: Box<Expr>,
        body: Block,
        else_body: Option<Block>,
    },
    When {
        // TODO
    },
    Lambda {
        lambda: Lambda,
    },
    Return(ReturnSyntax),
    TypeCast {
        target: Box<Expr>,
        is_safe: bool,
        type_: TypeName,
    },
}

impl Node for Expr {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct NameExprSyntax {
    pub(crate) name: String,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct CallExprSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) args: Vec<CallArg>,
    pub(crate) tailing_lambda: Option<Lambda>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<Expr>,
    pub(crate) is_vararg: bool,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Lambda {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum PostfixSuffix {
    Operator {
        kind: String,
    },
    TypeArgumentSuffix {
        types: Vec<TypeName>,
    },
    CallSuffix {
        args: Vec<CallArg>,
        tailing_lambda: Option<Lambda>,
    },
    IndexingSuffix,
    NavigationSuffix {
        is_safe: bool,
        name: String,
    },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ReturnSyntax {
    pub(crate) value: Option<Box<Expr>>,
}
