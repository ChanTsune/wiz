use crate::syntax::declaration::fun_syntax::{ArgDefListSyntax, FunBody, FunSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeName, TypeParameterListSyntax};
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StructSyntax {
    pub struct_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub open: TokenSyntax,
    pub properties: Vec<StructPropertySyntax>,
    pub close: TokenSyntax,
}

impl Syntax for StructSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            struct_keyword: self.struct_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_params: self.type_params,
            open: self.open,
            properties: self.properties,
            close: self.close,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            struct_keyword: self.struct_keyword,
            name: self.name,
            type_params: self.type_params,
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
    Init(InitializerSyntax),
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
            StructPropertySyntax::Init(i) => {
                StructPropertySyntax::Init(i.with_leading_trivia(trivia))
            }
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
            StructPropertySyntax::Init(i) => {
                StructPropertySyntax::Init(i.with_trailing_trivia(trivia))
            }
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
    pub type_: TypeName,
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
pub struct InitializerSyntax {
    pub init_keyword: TokenSyntax,
    pub args: ArgDefListSyntax,
    pub body: FunBody,
}

impl Syntax for InitializerSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            init_keyword: self.init_keyword.with_leading_trivia(trivia),
            args: self.args,
            body: self.body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            init_keyword: self.init_keyword,
            args: self.args,
            body: self.body.with_trailing_trivia(trivia),
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
