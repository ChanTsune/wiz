use std::collections::HashMap;
use wiz_hir::typed_decl::TypedFunBody;
use wiz_hir::typed_type::{TypedType, TypedTypeParam};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArenaFunction {
    pub ty: TypedType,
    pub type_parameters: Option<Vec<TypedTypeParam>>,
    pub body: Option<TypedFunBody>,
    pub used: Vec<HashMap<TypedTypeParam, TypedType>>,
}

impl ArenaFunction {
    pub fn new(
        ty: TypedType,
        type_parameters: Option<Vec<TypedTypeParam>>,
        body: Option<TypedFunBody>,
    ) -> Self {
        Self {
            ty,
            type_parameters,
            body,
            used: Default::default(),
        }
    }

    pub fn is_generic(&self) -> bool {
        self.type_parameters.is_some()
    }
}
