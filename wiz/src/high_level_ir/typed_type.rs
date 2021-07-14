use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct Package {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedType {
    pub(crate) package: Package,
    pub(crate) name: String,
}

impl TypedType {
    fn builtin(name: &str) -> TypedType {
        TypedType {
            package: Package { names: vec![] },
            name: String::from(name),
        }
    }

    pub fn noting() -> TypedType {
        Self::builtin("Noting")
    }

    pub fn unit() -> TypedType {Self::builtin("Unit")}

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
}
