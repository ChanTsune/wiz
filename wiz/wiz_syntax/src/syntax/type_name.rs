use crate::syntax::name_space::NameSpaceSyntax;
use crate::syntax::node::SyntaxNode;
use crate::syntax::token::TokenSyntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    NameSpaced(Box<NameSpacedTypeName>),
    Simple(SimpleTypeName),
    Decorated(Box<DecoratedTypeName>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameSpacedTypeName {
    pub name_space: NameSpaceSyntax,
    pub type_name: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SimpleTypeName {
    pub name: String,
    pub type_args: Option<Vec<TypeName>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecoratedTypeName {
    pub decoration: TokenSyntax,
    pub type_: TypeName,
}

impl SyntaxNode for TypeName {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeParam {
    pub name: String,
    pub type_constraints: Option<TypeName>,
}

impl SyntaxNode for TypeParam {}
