use crate::syntax::node::SyntaxNode;
use crate::syntax::type_name::TypeName;
use std::fmt;
use crate::syntax::token::TokenSyntax;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum ArgDef {
    Value(ValueArgDef),
    Self_(SelfArgDefSyntax),
}

impl SyntaxNode for ArgDef {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ValueArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_name: TypeName,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct SelfArgDefSyntax {
    pub(crate) reference: Option<TokenSyntax>,
    pub(crate) self_: TokenSyntax
}