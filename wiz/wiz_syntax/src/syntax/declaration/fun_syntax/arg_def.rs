use crate::syntax::list::{ElementSyntax, ListSyntax};
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;
use crate::syntax::Syntax;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgDef {
    Value(ValueArgDef),
    Self_(SelfArgDefSyntax),
}

impl Syntax for ArgDef {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self {
            ArgDef::Value(v) => ArgDef::Value(v.with_leading_trivia(trivia)),
            ArgDef::Self_(s) => ArgDef::Self_(s.with_leading_trivia(trivia)),
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self {
            ArgDef::Value(v) => ArgDef::Value(v.with_trailing_trivia(trivia)),
            ArgDef::Self_(s) => ArgDef::Self_(s.with_trailing_trivia(trivia)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValueArgDef {
    pub label: Option<TokenSyntax>,
    pub name: TokenSyntax,
    pub colon: TokenSyntax,
    pub type_name: TypeName,
}

impl Syntax for ValueArgDef {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.label {
            Some(label) => Self {
                label: Some(label.with_leading_trivia(trivia)),
                name: self.name,
                colon: self.colon,
                type_name: self.type_name,
            },
            None => Self {
                label: None,
                name: self.name.with_leading_trivia(trivia),
                colon: self.colon,
                type_name: self.type_name,
            },
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            label: self.label,
            name: self.name,
            colon: self.colon,
            type_name: self.type_name.with_trailing_trivia(trivia),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelfArgDefSyntax {
    pub reference: Option<TokenSyntax>,
    pub self_: TokenSyntax,
}

impl Syntax for SelfArgDefSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.reference {
            None => Self {
                reference: None,
                self_: self.self_.with_leading_trivia(trivia),
            },
            Some(reference) => Self {
                reference: Some(reference.with_leading_trivia(trivia)),
                self_: self.self_,
            },
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            reference: self.reference,
            self_: self.self_.with_trailing_trivia(trivia),
        }
    }
}

pub type ArgDefListSyntax = ListSyntax<ArgDef>;

impl ArgDefListSyntax {
    fn new() -> Self {
        Self {
            open: TokenSyntax::from("("),
            elements: vec![],
            close: TokenSyntax::from(")"),
        }
    }
}

impl Default for ArgDefListSyntax {
    fn default() -> Self {
        Self::new()
    }
}

pub type ArgDefElementSyntax = ElementSyntax<ArgDef>;
