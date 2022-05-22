use crate::high_level_ir::typed_type::TypedType;
use std::collections::HashSet;
use crate::high_level_ir::declaration_id::DeclarationId;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnvValue {
    Value(HashSet<(Vec<String>, TypedType)>),
    Type(DeclarationId),
}

impl From<(Vec<String>, TypedType)> for EnvValue {
    fn from(typed_type: (Vec<String>, TypedType)) -> Self {
        Self::Value(HashSet::from([typed_type]))
    }
}

impl From<HashSet<(Vec<String>, TypedType)>> for EnvValue {
    fn from(typed_type: HashSet<(Vec<String>, TypedType)>) -> Self {
        Self::Value(typed_type)
    }
}

impl From<DeclarationId> for EnvValue {
    fn from(s: DeclarationId) -> Self {
        Self::Type(s)
    }
}
