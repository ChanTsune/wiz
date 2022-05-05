use crate::high_level_ir::type_resolver::context::{NameSpace, ResolverStruct};
use crate::high_level_ir::typed_type::TypedType;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnvValue {
    NameSpace(NameSpace),
    Value(HashSet<(Vec<String>, TypedType)>),
    Type(ResolverStruct),
}

impl From<(Vec<String>, TypedType)> for EnvValue {
    fn from(typed_type: (Vec<String>, TypedType)) -> Self {
        Self::Value(HashSet::from([typed_type]))
    }
}

impl From<NameSpace> for EnvValue {
    fn from(ns: NameSpace) -> Self {
        Self::NameSpace(ns)
    }
}

impl From<HashSet<(Vec<String>, TypedType)>> for EnvValue {
    fn from(typed_type: HashSet<(Vec<String>, TypedType)>) -> Self {
        Self::Value(typed_type)
    }
}

impl From<ResolverStruct> for EnvValue {
    fn from(s: ResolverStruct) -> Self {
        Self::Type(s)
    }
}
