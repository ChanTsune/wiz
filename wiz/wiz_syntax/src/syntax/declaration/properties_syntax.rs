use crate::syntax::declaration::fun_syntax::{FunBody, FunSyntax};
use crate::syntax::declaration::TypeAnnotationSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct StructBodySyntax {
    pub open: TokenSyntax,
    pub properties: Vec<StructPropertySyntax>,
    pub close: TokenSyntax,
}

impl Syntax for StructBodySyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open.with_leading_trivia(trivia),
            properties: self.properties,
            close: self.close,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            open: self.open,
            properties: self.properties,
            close: self.close.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StructPropertySyntax {
    StoredProperty(StoredPropertySyntax),
    ComputedProperty,
    Deinit(DeinitializerSyntax),
    Method(FunSyntax),
}

impl Syntax for StructPropertySyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            StructPropertySyntax::StoredProperty(s) => {
                StructPropertySyntax::StoredProperty(s.with_leading_trivia(trivia))
            }
            StructPropertySyntax::ComputedProperty => StructPropertySyntax::ComputedProperty,
            StructPropertySyntax::Deinit(d) => {
                StructPropertySyntax::Deinit(d.with_leading_trivia(trivia))
            }
            StructPropertySyntax::Method(m) => {
                StructPropertySyntax::Method(m.with_leading_trivia(trivia))
            }
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            StructPropertySyntax::StoredProperty(s) => {
                StructPropertySyntax::StoredProperty(s.with_trailing_trivia(trivia))
            }
            StructPropertySyntax::ComputedProperty => StructPropertySyntax::ComputedProperty,
            StructPropertySyntax::Deinit(d) => {
                StructPropertySyntax::Deinit(d.with_trailing_trivia(trivia))
            }
            StructPropertySyntax::Method(m) => {
                StructPropertySyntax::Method(m.with_trailing_trivia(trivia))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StoredPropertySyntax {
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_: TypeAnnotationSyntax,
}

impl Syntax for StoredPropertySyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_: self.type_,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            mutability_keyword: self.mutability_keyword,
            name: self.name,
            type_: self.type_.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DeinitializerSyntax {
    pub deinit_keyword: TokenSyntax,
    pub body: FunBody,
}

impl Syntax for DeinitializerSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            deinit_keyword: self.deinit_keyword.with_leading_trivia(trivia),
            body: self.body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            deinit_keyword: self.deinit_keyword,
            body: self.body.with_trailing_trivia(trivia),
        }
    }
}
