use crate::syntax::node::SyntaxNode;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypeName {
    pub(crate) name: String,
    pub(crate) type_args: Option<Vec<TypeName>>,
}

impl SyntaxNode for TypeName {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) type_constraints: Option<TypeName>,
}

impl SyntaxNode for TypeParam {}
