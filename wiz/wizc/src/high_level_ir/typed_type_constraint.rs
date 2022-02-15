use crate::high_level_ir::typed_type::TypedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedTypeConstraint {
    pub type_: TypedType,
    pub constraints: Vec<TypedType>,
}
