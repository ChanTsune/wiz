use crate::syntax::expression::Expr;
use crate::syntax::Syntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeCastSyntax {
    pub target: Box<Expr>,
    pub operator: TokenSyntax,
    pub type_: TypeName,
}

impl Syntax for TypeCastSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: Box::new(self.target.with_leading_trivia(trivia)),
            operator: self.operator,
            type_: self.type_
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        Self {
            target: self.target,
            operator: self.operator,
            type_: self.type_.with_trailing_trivia(trivia)
        }
    }
}
