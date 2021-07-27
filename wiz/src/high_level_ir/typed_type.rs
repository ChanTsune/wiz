use std::fmt;
use crate::high_level_ir::typed_decl::TypedArgDef;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct Package {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedType {
    Value(TypedValueType),
    Function(Box<TypedFunctionType>),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedValueType {
    pub(crate) package: Package,
    pub(crate) name: String,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedFunctionType {
    pub(crate) arguments: Vec<TypedArgDef>,
    pub(crate) return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedTypeParam {
    pub(crate) name: String,
    pub(crate) type_constraint: Vec<TypedType>,
}

impl TypedType {
    fn builtin(name: &str) -> TypedType {
        TypedType::Value(TypedValueType {
            package: Package { names: vec![] },
            name: String::from(name),
        })
    }

    pub fn noting() -> TypedType {
        Self::builtin("Noting")
    }

    pub fn unit() -> TypedType {
        Self::builtin("Unit")
    }

    pub fn int8() -> TypedType {
        Self::builtin("Int8")
    }

    pub fn int16() -> TypedType {
        Self::builtin("Int16")
    }

    pub fn int32() -> TypedType {
        Self::builtin("Int32")
    }

    pub fn int64() -> TypedType {
        Self::builtin("Int64")
    }

    pub fn uint8() -> TypedType {
        Self::builtin("UInt8")
    }

    pub fn uint16() -> TypedType {
        Self::builtin("UInt16")
    }

    pub fn uint32() -> TypedType {
        Self::builtin("UInt32")
    }

    pub fn uint64() -> TypedType {
        Self::builtin("UInt64")
    }

    pub fn float() -> TypedType {
        Self::builtin("Float")
    }

    pub fn double() -> TypedType {
        Self::builtin("Double")
    }

    pub fn bool() -> TypedType {
        Self::builtin("Bool")
    }

    pub fn string() -> TypedType {
        Self::builtin("String")
    }
}
