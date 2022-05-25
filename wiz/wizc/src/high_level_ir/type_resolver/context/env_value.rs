use crate::high_level_ir::declaration_id::DeclarationId;
use wiz_hir::typed_type::TypedType;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnvValue {
    Value(HashSet<(DeclarationId, TypedType)>),
    Type(DeclarationId),
}

impl From<(DeclarationId, TypedType)> for EnvValue {
    fn from(typed_type: (DeclarationId, TypedType)) -> Self {
        Self::Value(HashSet::from([typed_type]))
    }
}

impl From<HashSet<(DeclarationId, TypedType)>> for EnvValue {
    fn from(typed_type: HashSet<(DeclarationId, TypedType)>) -> Self {
        Self::Value(typed_type)
    }
}

impl From<DeclarationId> for EnvValue {
    fn from(s: DeclarationId) -> Self {
        Self::Type(s)
    }
}
