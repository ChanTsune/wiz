use crate::syntax::annotation::{Annotatable, AnnotationsSyntax};
pub use crate::syntax::declaration::fun_syntax::arg_def::{
    ArgDef, ArgDefElementSyntax, ArgDefListSyntax, SelfArgDefSyntax, ValueArgDef,
};
pub use crate::syntax::declaration::fun_syntax::body_def::FunBody;
use crate::syntax::modifier::ModifiersSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeConstraintsSyntax, TypeName, TypeParameterListSyntax};
use crate::syntax::Syntax;

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

impl Syntax for FunSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        match self.annotations {
            None => Self {
                annotations: None,
                modifiers: self.modifiers, // TODO
                fun_keyword: self.fun_keyword,
                name: self.name,
                type_params: self.type_params,
                arg_defs: self.arg_defs,
                return_type: self.return_type,
                type_constraints: self.type_constraints,
                body: self.body,
            },
            Some(annotations) => Self {
                annotations: Some(annotations.with_leading_trivia(trivia)),
                modifiers: self.modifiers,
                fun_keyword: self.fun_keyword,
                name: self.name,
                type_params: self.type_params,
                arg_defs: self.arg_defs,
                return_type: self.return_type,
                type_constraints: self.type_constraints,
                body: self.body,
            },
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.body {
            None => match self.type_constraints {
                None => match self.return_type {
                    None => Self {
                        annotations: self.annotations,
                        modifiers: self.modifiers,
                        fun_keyword: self.fun_keyword,
                        name: self.name,
                        type_params: self.type_params,
                        arg_defs: self.arg_defs.with_trailing_trivia(trivia),
                        return_type: self.return_type,
                        type_constraints: self.type_constraints,
                        body: self.body,
                    },
                    Some(return_type) => Self {
                        annotations: self.annotations,
                        modifiers: self.modifiers,
                        fun_keyword: self.fun_keyword,
                        name: self.name,
                        type_params: self.type_params,
                        arg_defs: self.arg_defs,
                        return_type: Some(return_type.with_trailing_trivia(trivia)),
                        type_constraints: self.type_constraints,
                        body: self.body,
                    },
                },
                Some(type_constraints) => Self {
                    annotations: self.annotations,
                    modifiers: self.modifiers,
                    fun_keyword: self.fun_keyword,
                    name: self.name,
                    type_params: self.type_params,
                    arg_defs: self.arg_defs,
                    return_type: self.return_type,
                    type_constraints: Some(type_constraints.with_trailing_trivia(trivia)),
                    body: self.body,
                },
            },
            Some(body) => Self {
                annotations: self.annotations,
                modifiers: self.modifiers,
                fun_keyword: self.fun_keyword,
                name: self.name,
                type_params: self.type_params,
                arg_defs: self.arg_defs,
                return_type: self.return_type,
                type_constraints: self.type_constraints,
                body: Some(body.with_trailing_trivia(trivia)),
            },
        }
    }
}
