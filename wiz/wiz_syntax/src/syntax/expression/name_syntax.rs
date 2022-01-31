use crate::syntax::name_space::NameSpaceSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeArgumentListSyntax;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameExprSyntax {
    pub name_space: NameSpaceSyntax,
    pub name: TokenSyntax,
    pub type_arguments: Option<TypeArgumentListSyntax>,
}

impl NameExprSyntax {
    pub fn simple(name: TokenSyntax) -> Self {
        NameExprSyntax {
            name_space: NameSpaceSyntax::default(),
            name,
            type_arguments: None,
        }
    }
}

impl Syntax for NameExprSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            name_space: self.name_space.with_leading_trivia(trivia),
            name: self.name,
            type_arguments: self.type_arguments,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.type_arguments {
            None =>         Self {
                name_space: self.name_space,
                name: self.name.with_trailing_trivia(trivia),
                type_arguments: None,
            },
                Some(type_arguments) =>         Self {
                name_space: self.name_space,
                name: self.name,
                type_arguments: Some(type_arguments.with_trailing_trivia(trivia)),
            }
        }
    }
}
