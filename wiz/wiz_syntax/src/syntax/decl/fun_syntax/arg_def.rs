use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgDef {
    Value(ValueArgDef),
    Self_(SelfArgDefSyntax),
}

impl Syntax for ArgDef {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValueArgDef {
    pub label: Option<TokenSyntax>,
    pub name: TokenSyntax,
    pub type_name: TypeName,
}

impl Syntax for ValueArgDef {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelfArgDefSyntax {
    pub reference: Option<TokenSyntax>,
    pub self_: TokenSyntax,
}

impl Syntax for SelfArgDefSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        todo!()
    }
}
