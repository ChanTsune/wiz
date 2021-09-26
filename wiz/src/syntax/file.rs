use super::decl::Decl;
use super::node::SyntaxNode;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct WizFile {
    pub(crate) name: String,
    pub(crate) syntax: FileSyntax,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct FileSyntax {
    pub(crate) body: Vec<Decl>,
}

impl SyntaxNode for FileSyntax {}
