use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq)]
pub struct TypeName {
    pub(crate) name: String,
    pub(crate) type_params: Vec<TypeParam>
}

impl Node for TypeName {

}

#[derive(fmt::Debug, Eq, PartialEq)]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) type_constraints: Vec<TypeName>
}

impl Node for TypeParam {

}
