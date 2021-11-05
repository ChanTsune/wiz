mod array_syntax;
mod binary_operation_syntax;
mod call_syntax;
mod member_syntax;
mod name_syntax;
mod type_cast_syntax;
mod subscript_syntax;

use crate::syntax::block::BlockSyntax;
pub use crate::syntax::expression::array_syntax::{ArrayElementSyntax, ArraySyntax};
pub use crate::syntax::expression::binary_operation_syntax::BinaryOperationSyntax;
pub use crate::syntax::expression::call_syntax::{
    CallArg, CallArgElementSyntax, CallArgListSyntax, CallExprSyntax, LambdaSyntax,
};
pub use crate::syntax::expression::member_syntax::MemberSyntax;
pub use crate::syntax::expression::name_syntax::NameExprSyntax;
pub use crate::syntax::expression::subscript_syntax::{SubscriptSyntax, SubscriptIndexListSyntax, SubscriptIndexElementSyntax};
pub use crate::syntax::expression::type_cast_syntax::TypeCastSyntax;
use crate::syntax::literal::LiteralSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeArgumentListSyntax;
use crate::syntax::Syntax;

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

impl Syntax for Expr {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            Expr::Name(n) => Expr::Name(n.with_leading_trivia(trivia)),
            Expr::Literal(l) => Expr::Literal(l.with_leading_trivia(trivia)),
            Expr::BinOp(b) => Expr::BinOp(b.with_leading_trivia(trivia)),
            Expr::UnaryOp(_) => {
                todo!()
            }
            Expr::Subscript(s) => {
                Expr::Subscript(s.with_leading_trivia(trivia))
            }
            Expr::Member(m) => Expr::Member(m.with_leading_trivia(trivia)),
            Expr::Array(a) => Expr::Array(a.with_leading_trivia(trivia)),
            Expr::Tuple { .. } => {
                todo!()
            }
            Expr::Dict { .. } => {
                todo!()
            }
            Expr::StringBuilder { .. } => {
                todo!()
            }
            Expr::Call(c) => Expr::Call(c.with_leading_trivia(trivia)),
            Expr::If(_) => {
                todo!()
            }
            Expr::When { .. } => {
                todo!()
            }
            Expr::Lambda(_) => {
                todo!()
            }
            Expr::Return(_) => {
                todo!()
            }
            Expr::TypeCast(t) => Expr::TypeCast(t.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Expr::Name(n) => Expr::Name(n.with_trailing_trivia(trivia)),
            Expr::Literal(l) => Expr::Literal(l.with_trailing_trivia(trivia)),
            Expr::BinOp(b) => Expr::BinOp(b.with_trailing_trivia(trivia)),
            Expr::UnaryOp(_) => {
                todo!()
            }
            Expr::Subscript(s) => {
                Expr::Subscript(s.with_trailing_trivia(trivia))
            }
            Expr::Member(m) => Expr::Member(m.with_trailing_trivia(trivia)),
            Expr::Array(a) => Expr::Array(a.with_trailing_trivia(trivia)),
            Expr::Tuple { .. } => {
                todo!()
            }
            Expr::Dict { .. } => {
                todo!()
            }
            Expr::StringBuilder { .. } => {
                todo!()
            }
            Expr::Call(c) => Expr::Call(c.with_trailing_trivia(trivia)),
            Expr::If(_) => {
                todo!()
            }
            Expr::When { .. } => {
                todo!()
            }
            Expr::Lambda(_) => {
                todo!()
            }
            Expr::Return(_) => {
                todo!()
            }
            Expr::TypeCast(t) => Expr::TypeCast(t.with_trailing_trivia(trivia)),
        }
    }
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
pub enum PostfixSuffix {
    Operator(String),
    TypeArgumentSuffix(TypeArgumentListSyntax),
    CallSuffix {
        args: Option<CallArgListSyntax>,
        tailing_lambda: Option<LambdaSyntax>,
    },
    IndexingSuffix(SubscriptIndexListSyntax),
    NavigationSuffix {
        navigation: TokenSyntax,
        name: TokenSyntax,
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
