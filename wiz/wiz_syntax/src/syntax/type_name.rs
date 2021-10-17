use crate::syntax::node::SyntaxNode;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeName {
    Simple(SimpleTypeName),
    Decorated(Box<DecoratedTypeName>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SimpleTypeName {
    pub name: String,
    pub type_args: Option<Vec<TypeName>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecoratedTypeName {
    pub decoration: String,
    pub type_: TypeName,
}

impl SyntaxNode for TypeName {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeParam {
    pub name: String,
    pub type_constraints: Option<TypeName>,
}

impl SyntaxNode for TypeParam {}
