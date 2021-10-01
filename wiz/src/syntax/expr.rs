use crate::syntax::block::Block;
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::node::SyntaxNode;
use crate::syntax::stmt::Stmt;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::TypeName;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Name(NameExprSyntax),
    Literal(LiteralSyntax),
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
    Subscript(SubscriptSyntax),
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
    Lambda(LambdaSyntax),
    Return(ReturnSyntax),
    TypeCast(TypeCastSyntax),
}

impl SyntaxNode for Expr {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct NameExprSyntax {
    pub(crate) name_space: Vec<String>,
    pub(crate) name: String,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct CallExprSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) args: Vec<CallArg>,
    pub(crate) tailing_lambda: Option<LambdaSyntax>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<Expr>,
    pub(crate) is_vararg: bool,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct LambdaSyntax {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct SubscriptSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) idx_or_keys: Vec<Expr>,
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
        tailing_lambda: Option<LambdaSyntax>,
    },
    IndexingSuffix {
        indexes: Vec<Expr>,
    },
    NavigationSuffix {
        is_safe: bool,
        name: String,
    },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ReturnSyntax {
    pub(crate) return_keyword: TokenSyntax,
    pub(crate) value: Option<Box<Expr>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypeCastSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) is_safe: bool,
    pub(crate) type_: TypeName,
}
