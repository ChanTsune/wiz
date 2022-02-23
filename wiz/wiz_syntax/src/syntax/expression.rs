mod array_syntax;
mod binary_operation_syntax;
mod call_syntax;
mod if_syntax;
mod member_syntax;
mod name_syntax;
mod return_syntax;
mod subscript_syntax;
mod type_cast_syntax;
mod unary_operation_syntax;

pub use crate::syntax::expression::array_syntax::{ArrayElementSyntax, ArraySyntax};
pub use crate::syntax::expression::binary_operation_syntax::BinaryOperationSyntax;
pub use crate::syntax::expression::call_syntax::{
    CallArg, CallArgElementSyntax, CallArgListSyntax, CallExprSyntax, LambdaSyntax, ArgLabelSyntax
};
pub use crate::syntax::expression::if_syntax::{ElseSyntax, IfExprSyntax};
pub use crate::syntax::expression::member_syntax::MemberSyntax;
pub use crate::syntax::expression::name_syntax::NameExprSyntax;
pub use crate::syntax::expression::return_syntax::ReturnSyntax;
pub use crate::syntax::expression::subscript_syntax::{
    SubscriptIndexElementSyntax, SubscriptIndexListSyntax, SubscriptSyntax,
};
pub use crate::syntax::expression::type_cast_syntax::TypeCastSyntax;
pub use crate::syntax::expression::unary_operation_syntax::{
    PostfixUnaryOperationSyntax, PrefixUnaryOperationSyntax, UnaryOperationSyntax,
};
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
            Expr::UnaryOp(u) => Expr::UnaryOp(u.with_leading_trivia(trivia)),
            Expr::Subscript(s) => Expr::Subscript(s.with_leading_trivia(trivia)),
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
            Expr::If(i) => Expr::If(i.with_leading_trivia(trivia)),
            Expr::When { .. } => {
                todo!()
            }
            Expr::Lambda(_) => {
                todo!()
            }
            Expr::Return(r) => Expr::Return(r.with_leading_trivia(trivia)),
            Expr::TypeCast(t) => Expr::TypeCast(t.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            Expr::Name(n) => Expr::Name(n.with_trailing_trivia(trivia)),
            Expr::Literal(l) => Expr::Literal(l.with_trailing_trivia(trivia)),
            Expr::BinOp(b) => Expr::BinOp(b.with_trailing_trivia(trivia)),
            Expr::UnaryOp(u) => Expr::UnaryOp(u.with_trailing_trivia(trivia)),
            Expr::Subscript(s) => Expr::Subscript(s.with_trailing_trivia(trivia)),
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
            Expr::If(i) => Expr::If(i.with_trailing_trivia(trivia)),
            Expr::When { .. } => {
                todo!()
            }
            Expr::Lambda(_) => {
                todo!()
            }
            Expr::Return(r) => Expr::Return(r.with_trailing_trivia(trivia)),
            Expr::TypeCast(t) => Expr::TypeCast(t.with_trailing_trivia(trivia)),
        }
    }
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
