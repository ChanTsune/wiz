pub use crate::syntax::declaration::fun_syntax::arg_def::{
    ArgDef, ArgDefElementSyntax, ArgDefListSyntax, SelfArgDefSyntax, ValueArgDef,
};
pub use crate::syntax::declaration::fun_syntax::body_def::{ExprFunBodySyntax, FunBody};
use crate::syntax::declaration::TypeAnnotationSyntax;
use crate::syntax::token::TokenSyntax;
use crate::syntax::trivia::Trivia;
use crate::syntax::type_name::{TypeConstraintsSyntax, TypeParameterListSyntax};
use crate::syntax::Syntax;

mod arg_def;
mod body_def;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunSyntax {
    pub fun_keyword: TokenSyntax,
    pub name: TokenSyntax,
    pub type_params: Option<TypeParameterListSyntax>,
    pub arg_defs: ArgDefListSyntax,
    pub return_type: Option<TypeAnnotationSyntax>,
    pub type_constraints: Option<TypeConstraintsSyntax>,
    pub body: Option<FunBody>,
}

impl Syntax for FunSyntax {
    fn with_leading_trivia(self, trivia: Trivia) -> Self {
        Self {
            fun_keyword: self.fun_keyword.with_leading_trivia(trivia),
            name: self.name,
            type_params: self.type_params,
            arg_defs: self.arg_defs,
            return_type: self.return_type,
            type_constraints: self.type_constraints,
            body: self.body,
        }
    }

    fn with_trailing_trivia(self, trivia: Trivia) -> Self {
        match self.body {
            None => match self.type_constraints {
                None => match self.return_type {
                    None => Self {
                        fun_keyword: self.fun_keyword,
                        name: self.name,
                        type_params: self.type_params,
                        arg_defs: self.arg_defs.with_trailing_trivia(trivia),
                        return_type: self.return_type,
                        type_constraints: self.type_constraints,
                        body: self.body,
                    },
                    Some(return_type) => Self {
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
