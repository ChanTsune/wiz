use crate::syntax::declaration::properties_syntax::{StructBodySyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeParameterListSyntax;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructSyntax {
    pub struct_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub body: StructBodySyntax,
}

impl Syntax for StructSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            struct_keyword: self.struct_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_params: self.type_params,
            body: self.body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            struct_keyword: self.struct_keyword,
            name: self.name,
            type_params: self.type_params,
            body: self.body.with_trailing_trivia(trivia),
        }
    }
}
