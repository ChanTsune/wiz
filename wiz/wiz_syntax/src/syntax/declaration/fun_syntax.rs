use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
pub use crate::syntax::declaration::fun_syntax::arg_def::{
    ArgDef, ArgDefElementSyntax, ArgDefListSyntax, SelfArgDefSyntax, ValueArgDef,
};
pub use crate::syntax::declaration::fun_syntax::body_def::FunBody;
use crate::syntax::modifier::ModifiersSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::type_name::{TypeConstraintsSyntax, TypeName, TypeParameterListSyntax};

mod arg_def;
mod body_def;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub modifiers: ModifiersSyntax,
    pub fun_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub arg_defs: ArgDefListSyntax,
    pub return_type: Option<TypeName>,
    pub type_constraints: Option<TypeConstraintsSyntax>,
    pub body: Option<FunBody>,
}

impl Annotatable for FunSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}
