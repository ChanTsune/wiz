use crate::syntax::node::SyntaxNode;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgDef {
    Value(ValueArgDef),
    Self_(SelfArgDefSyntax),
}

impl SyntaxNode for ArgDef {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValueArgDef {
    pub label: String,
    pub name: String,
    pub type_name: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelfArgDefSyntax {
    pub reference: Option<TokenSyntax>,
    pub self_: TokenSyntax,
}
