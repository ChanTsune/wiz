use std::fmt;
use crate::ast::node::Node;
use crate::ast::decl::Decl;

#[derive(fmt::Debug)]
struct Struct {

}

impl Node for Struct {}
impl Decl for Struct {}
