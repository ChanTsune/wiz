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
    pub(crate) name_space: Vec<String>,
    pub(crate) name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryOperationSyntax {
    pub(crate) left: Box<Expr>,
    pub(crate) operator: TokenSyntax,
    pub(crate) right: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOperationSyntax {
    Prefix(PrefixUnaryOperationSyntax),
    Postfix(PostfixUnaryOperationSyntax),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrefixUnaryOperationSyntax {
    pub(crate) operator: TokenSyntax,
    pub(crate) target: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PostfixUnaryOperationSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) operator: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallExprSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) args: Vec<CallArg>,
    pub(crate) tailing_lambda: Option<LambdaSyntax>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CallArg {
    pub(crate) label: Option<String>,
    pub(crate) arg: Box<Expr>,
    pub(crate) is_vararg: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LambdaSyntax {
    pub(crate) stmts: Vec<Stmt>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SubscriptSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) idx_or_keys: Vec<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemberSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) name: TokenSyntax,
    pub(crate) navigation_operator: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArraySyntax {
    pub(crate) open: TokenSyntax,
    pub(crate) values: Vec<ArrayElementSyntax>,
    pub(crate) close: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArrayElementSyntax {
    pub(crate) element: Expr,
    pub(crate) trailing_comma: TokenSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
        navigation: String,
        name: String,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IfExprSyntax {
    pub(crate) condition: Box<Expr>,
    pub(crate) body: BlockSyntax,
    pub(crate) else_body: Option<BlockSyntax>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ReturnSyntax {
    pub(crate) return_keyword: TokenSyntax,
    pub(crate) value: Option<Box<Expr>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeCastSyntax {
    pub(crate) target: Box<Expr>,
    pub(crate) operator: String,
    pub(crate) type_: TypeName,
}
