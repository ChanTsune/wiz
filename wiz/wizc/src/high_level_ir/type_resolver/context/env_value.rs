use crate::high_level_ir::type_resolver::context::{NameSpace, ResolverStruct};
use crate::high_level_ir::typed_type::TypedType;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnvValue {
    NameSpace(NameSpace),
    Value(HashSet<TypedType>),
    Type(ResolverStruct),
}

impl EnvValue {
    pub(crate) fn get<T: ToString>(&self, mut ns: Vec<T>) -> Option<&EnvValue> {
        if ns.is_empty() {
            Some(self)
        } else {
            match self {
                EnvValue::NameSpace(n) => n.get(ns),
                EnvValue::Value(_) => None,
                EnvValue::Type(_) => todo!(),
            }
        }
    }
}

impl From<TypedType> for EnvValue {
    fn from(typed_type: TypedType) -> Self {
        Self::Value(HashSet::from([typed_type]))
    }
}

impl From<NameSpace> for EnvValue {
    fn from(ns: NameSpace) -> Self {
        Self::NameSpace(ns)
    }
}

impl From<HashSet<TypedType>> for EnvValue {
    fn from(typed_type: HashSet<TypedType>) -> Self {
        Self::Value(typed_type)
    }
}

impl From<ResolverStruct> for EnvValue {
    fn from(s: ResolverStruct) -> Self {
        Self::Type(s)
    }
}
