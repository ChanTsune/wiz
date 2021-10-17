use crate::syntax::block::BlockSyntax;
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::node::SyntaxNode;
use crate::syntax::stmt::Stmt;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Name(NameExprSyntax),
    Literal(LiteralSyntax),
    BinOp(BinaryOperationSyntax),
    UnaryOp(UnaryOperationSyntax),
    Subscript(SubscriptSyntax),
    Member(MemberSyntax),
    Array(ArraySyntax),
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
    If(IfExprSyntax),
    When {
        // TODO
    },
    Lambda(LambdaSyntax),
    Return(ReturnSyntax),
    TypeCast(TypeCastSyntax),
}

impl SyntaxNode for Expr {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameExprSyntax {
    pub name_space: Vec<String>,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryOperationSyntax {
    pub left: Box<Expr>,
    pub operator: TokenSyntax,
    pub right: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOperationSyntax {
    Prefix(PrefixUnaryOperationSyntax),
    Postfix(PostfixUnaryOperationSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrefixUnaryOperationSyntax {
    pub operator: TokenSyntax,
    pub target: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PostfixUnaryOperationSyntax {
    pub target: Box<Expr>,
    pub operator: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallExprSyntax {
    pub target: Box<Expr>,
    pub args: Vec<CallArg>,
    pub tailing_lambda: Option<LambdaSyntax>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub label: Option<String>,
    pub arg: Box<Expr>,
    pub is_vararg: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LambdaSyntax {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SubscriptSyntax {
    pub target: Box<Expr>,
    pub idx_or_keys: Vec<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemberSyntax {
    pub target: Box<Expr>,
    pub name: TokenSyntax,
    pub navigation_operator: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArraySyntax {
    pub open: TokenSyntax,
    pub values: Vec<ArrayElementSyntax>,
    pub close: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArrayElementSyntax {
    pub element: Expr,
    pub trailing_comma: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PostfixSuffix {
    Operator(String),
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
        navigation: String,
        name: String,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IfExprSyntax {
    pub condition: Box<Expr>,
    pub body: BlockSyntax,
    pub else_body: Option<BlockSyntax>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ReturnSyntax {
    pub return_keyword: TokenSyntax,
    pub value: Option<Box<Expr>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeCastSyntax {
    pub target: Box<Expr>,
    pub operator: String,
    pub type_: TypeName,
}
