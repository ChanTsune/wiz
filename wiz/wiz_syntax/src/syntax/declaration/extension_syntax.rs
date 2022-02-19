use crate::syntax::declaration::{StructBodySyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeConstraintsSyntax, TypeName, TypeParameterListSyntax};
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExtensionSyntax {
    pub extension_keyword: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub name: TypeName,
    pub protocol_extension: Option<ProtocolConformSyntax>,
    pub type_constraints: Option<TypeConstraintsSyntax>,
    pub body: StructBodySyntax,
}

impl Syntax for ExtensionSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            extension_keyword: self.extension_keyword.with_leading_trivia(trivia),
            type_params: self.type_params,
            name: self.name,
            protocol_extension: self.protocol_extension,
            type_constraints: self.type_constraints,
            body: self.body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            extension_keyword: self.extension_keyword,
            type_params: self.type_params,
            name: self.name,
            protocol_extension: self.protocol_extension,
            type_constraints: self.type_constraints,
            body: self.body.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProtocolConformSyntax {
    pub colon: TokenSyntax,
    pub protocol: TypeName,
}
