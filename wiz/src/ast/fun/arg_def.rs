use crate::ast::type_name::TypeName;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug)]
pub struct ArgDef {
    label: String,
    name: String,
    type_name: TypeName
}

impl Node for ArgDef {

}