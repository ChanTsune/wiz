use super::decl::Decl;
use super::node::Node;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct File {
    pub(crate) body: Vec<Decl>
}

impl Node for File {
}
