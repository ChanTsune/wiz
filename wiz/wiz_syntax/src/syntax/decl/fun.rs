use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
use crate::syntax::decl::fun::arg_def::ArgDef;
use crate::syntax::decl::fun::body_def::FunBody;
use crate::syntax::modifier::ModifiersSyntax;
use crate::syntax::type_name::{TypeName, TypeParam};

pub mod arg_def;
pub mod body_def;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunSyntax {
    pub annotations: Option<AnnotationsSyntax>,
    pub modifiers: ModifiersSyntax,
    pub name: String,
    pub type_params: Option<Vec<TypeParam>>,
    pub arg_defs: Vec<ArgDef>,
    pub return_type: Option<TypeName>,
    pub body: Option<FunBody>,
}

impl Annotatable for FunSyntax {
    fn with_annotation(mut self, a: AnnotationsSyntax) -> Self {
        self.annotations = Some(a);
        self
    }
}
