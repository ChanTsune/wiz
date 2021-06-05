use crate::ast::type_name::TypeName;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq)]
pub struct ArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_name: TypeName
}

impl Node for ArgDef {

}