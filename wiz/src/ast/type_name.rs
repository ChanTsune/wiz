use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug)]
pub struct TypeName {
    name: String,
    type_params: Vec<TypeParam>
}

impl Node for TypeName {

}

#[derive(fmt::Debug)]
pub struct TypeParam {
    name: String,
    type_constraint: TypeName
}

impl Node for TypeParam {

}