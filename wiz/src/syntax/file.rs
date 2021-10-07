use super::decl::Decl;
use super::node::SyntaxNode;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SourceSet {
    File(WizFile),
    Dir { name: String, items: Vec<SourceSet> },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WizFile {
    pub(crate) name: String,
    pub(crate) syntax: FileSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileSyntax {
    pub(crate) body: Vec<Decl>,
}

impl SyntaxNode for FileSyntax {}
