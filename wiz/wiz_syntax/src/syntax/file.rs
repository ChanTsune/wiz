use super::declaration::Decl;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SourceSet {
    File(WizFile),
    Dir { name: String, items: Vec<SourceSet> },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WizFile {
    pub name: String,
    pub syntax: FileSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileSyntax {
    pub body: Vec<Decl>,
}

impl Syntax for FileSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}
