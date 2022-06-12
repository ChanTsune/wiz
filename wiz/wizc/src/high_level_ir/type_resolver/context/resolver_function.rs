use std::collections::HashMap;
use wiz_hir::typed_decl::TypedFunBody;
use wiz_hir::typed_type::{TypedType, TypedTypeParam};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResolverFunction {
    pub(crate) ty: TypedType,
    pub(crate) type_parameters: Option<Vec<TypedTypeParam>>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) used: Vec<HashMap<TypedTypeParam, TypedType>>,
}

impl ResolverFunction {
    pub(crate) fn new(
        ty: TypedType,
        type_parameters: Option<Vec<TypedTypeParam>>,
        body: Option<TypedFunBody>,
    ) -> Self {
        Self {
            ty, type_parameters, body, used: Default::default()
        }
    }

    pub fn is_generic(&self) -> bool {
        self.type_parameters.is_some()
    }
}