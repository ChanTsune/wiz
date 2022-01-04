use crate::high_level_ir::type_resolver::context::NameSpace;
use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum EnvValue {
    NameSpace(NameSpace),
    Value(TypedType),
}

impl From<TypedType> for EnvValue {
    fn from(typed_type: TypedType) -> Self {
        Self::Value(typed_type)
    }
}

impl From<NameSpace> for EnvValue {
    fn from(ns: NameSpace) -> Self {
        Self::NameSpace(ns)
    }
}
