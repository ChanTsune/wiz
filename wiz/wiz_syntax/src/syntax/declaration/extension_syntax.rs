use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::declaration::Decl;
use crate::syntax::modifier::ModifiersSyntax;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeConstraintsSyntax, TypeParameterListSyntax};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExtensionSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub modifiers: ModifiersSyntax,
    pub extension_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub type_constraints: Option<TypeConstraintsSyntax>,
    pub body: Vec<Decl>,
}

impl Annotatable for ExtensionSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}

impl Syntax for ExtensionSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.annotations {
            None => Self {
                annotations: None,
                modifiers: self.modifiers, // TODO
                extension_keyword: self.extension_keyword,
                name: self.name,
                type_params: self.type_params,
                type_constraints: self.type_constraints,
                body: self.body,
            },
            Some(annotations) => Self {
                annotations: Some(annotations.with_leading_trivia(trivia)),
                modifiers: self.modifiers,
                extension_keyword: self.extension_keyword,
                name: self.name,
                type_params: self.type_params,
                type_constraints: self.type_constraints,
                body: self.body,
            },
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            annotations: self.annotations,
            modifiers: self.modifiers,
            extension_keyword: self.extension_keyword,
            name: self.name,
            type_params: self.type_params,
            type_constraints: self.type_constraints,
            body: self.body, // TODO
        }
    }
}
