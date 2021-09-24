use crate::ast::node::Node;
use crate::ast::type_name::TypeName;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum ArgDef {
    Value(ValueArgDef),
    Self_,
}

impl Node for ArgDef {}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct ValueArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_name: TypeName,
}
