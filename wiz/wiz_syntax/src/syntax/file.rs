use crate::syntax::declaration::DeclarationSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SourceSet {
    File(WizFile),
    Dir { name: String, items: Vec<SourceSet> },
}

impl SourceSet {
    pub fn name(&self) -> String {
        match self {
            SourceSet::File(f) => &f.name,
            SourceSet::Dir { name, .. } => name,
        }
        .clone()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WizFile {
    pub name: String,
    pub syntax: FileSyntax,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileSyntax {
    pub leading_trivia: Trivia,
    pub body: Vec<DeclarationSyntax>,
    pub trailing_trivia: Trivia,
}

impl Syntax for FileSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: trivia,
            body: self.body,
            trailing_trivia: self.trailing_trivia,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            leading_trivia: self.leading_trivia,
            body: self.body,
            trailing_trivia: trivia,
        }
    }
}
