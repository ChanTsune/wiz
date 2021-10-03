use crate::syntax::node::SyntaxNode;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    Simple(SimpleTypeName),
    Decorated(Box<DecoratedTypeName>),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct SimpleTypeName {
    pub(crate) name: String,
    pub(crate) type_args: Option<Vec<TypeName>>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct DecoratedTypeName {
    pub(crate) decoration: String,
    pub(crate) type_: TypeName
}

impl SyntaxNode for TypeName {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) type_constraints: Option<TypeName>,
}

impl SyntaxNode for TypeParam {}
