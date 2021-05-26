use super::decl::Decl;
use super::node::Node;
use std::fmt;

#[derive(fmt::Debug)]
pub struct File {
    pub(crate) body: Vec<Box<dyn Decl>>
}

impl Node for File {
}
