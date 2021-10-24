use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::expression::Expr;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::TypeName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VarSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub mutability_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_: Option<TypeName>,
    pub value: Expr,
}

impl Annotatable for VarSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}
