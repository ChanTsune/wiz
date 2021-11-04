use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UseSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub use_keyword: TokenSyntax,
    pub package_name: PackageName,
    pub alias: Option<AliasSyntax>,
}

impl Annotatable for UseSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

impl Syntax for UseSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.annotations {
            None => {
                Self {
                    annotations: None,
                    use_keyword: self.use_keyword.with_leading_trivia(trivia),
                    package_name: self.package_name,
                    alias: self.alias,
                }
            }
            Some(annotations) => {
                Self {
                    annotations: Some(annotations.with_leading_trivia(trivia)),
                    use_keyword: self.use_keyword,
                    package_name: self.package_name,
                    alias: self.alias,
                }
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.alias {
            None => {
                todo!();
                Self {
                    annotations: self.annotations,
                    use_keyword: self.use_keyword,
                    package_name: self.package_name,
                    alias: None,
                }
            }
            Some(alias) => {
                Self {
                    annotations: self.annotations,
                    use_keyword: self.use_keyword,
                    package_name: self.package_name,
                    alias: Some(alias.with_trailing_trivia(trivia)),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PackageName {
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AliasSyntax {
    pub as_keyword: TokenSyntax,
    pub name: TokenSyntax,
}

impl Syntax for AliasSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            as_keyword: self.as_keyword.with_leading_trivia(trivia),
            name: self.name
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            as_keyword: self.as_keyword,
            name: self.name.with_trailing_trivia(trivia)
        }
    }
}
