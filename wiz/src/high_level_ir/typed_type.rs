use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::typed_decl::TypedArgDef;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct Package {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedType {
    Value(TypedValueType),
    Function(Box<TypedFunctionType>),
    Type(TypedValueType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedValueType {
    pub(crate) package: Package,
    pub(crate) name: String,
    pub(crate) type_args: Option<Vec<TypedType>>,
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

impl Package {
    pub(crate) fn global() -> Self {
        Self { names: vec![] }
    }
}

impl TypedType {
    fn builtin(name: &str) -> TypedType {
        TypedType::Value(TypedValueType {
            package: Package::global(),
            name: String::from(name),
            type_args: None,
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

    pub fn signed_integer_types() -> Vec<TypedType> {
        vec![Self::int8(), Self::int16(), Self::int32(), Self::int64()]
    }

    pub fn unsigned_integer_types() -> Vec<TypedType> {
        vec![
            Self::uint8(),
            Self::uint16(),
            Self::uint32(),
            Self::uint64(),
        ]
    }

    pub fn integer_types() -> Vec<TypedType> {
        Self::signed_integer_types()
            .into_iter()
            .chain(Self::unsigned_integer_types())
            .collect()
    }

    pub fn floating_point_types() -> Vec<TypedType> {
        vec![Self::float(), Self::double()]
    }

    pub fn builtin_types() -> Vec<TypedType> {
        Self::integer_types()
            .into_iter()
            .chain(Self::floating_point_types())
            .chain(vec![
                Self::noting(),
                Self::unit(),
                Self::bool(),
                Self::string(),
            ])
            .collect()
    }

    pub fn is_primitive(&self) -> bool {
        Self::builtin_types().contains(self)
    }

    pub fn is_floating_point(&self) -> bool {
        Self::floating_point_types().contains(self)
    }

    pub fn is_signed_integer(&self) -> bool {
        Self::signed_integer_types().contains(self)
    }

    pub fn is_unsigned_integer(&self) -> bool {
        Self::unsigned_integer_types().contains(self)
    }

    pub fn is_integer(&self) -> bool {
        Self::integer_types().contains(self)
    }

    pub fn is_pointer_type(&self) -> bool {
        match self {
            TypedType::Value(v) => v.name == UNSAFE_POINTER,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        Self::bool().eq(self)
    }
}
