use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedTypeConstraint {
    pub type_: TypedType,
    pub constraints: Vec<TypedType>,
}
