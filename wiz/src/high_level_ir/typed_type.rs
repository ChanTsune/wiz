use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::typed_decl::TypedArgDef;
use std::fmt;
use std::option::Option::Some;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct Package {
    pub(crate) names: Vec<String>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedType {
    Value(TypedValueType),
    Function(Box<TypedFunctionType>),
    Type(TypedValueType),
    Reference(TypedValueType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedValueType {
    pub(crate) package: Option<Package>,
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
    pub(crate) fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    pub(crate) fn global() -> Self {
        Self { names: vec![] }
    }

    pub(crate) fn is_global(&self) -> bool {
        self.names.is_empty()
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
        self.names.join("::")
    }
}

impl TypedValueType {
    pub(crate) fn is_unsafe_pointer(&self) -> bool {
        self.name == UNSAFE_POINTER
            && if let Some(pkg) = &self.package {
                pkg.is_global()
            } else {
                false
            }
    }
}

impl ToString for TypedValueType {
    fn to_string(&self) -> String {
        match &self.package {
            None => { self.name.clone() }
            Some(pkg) => {pkg.to_string()}
        }
    }
}

impl TypedType {
    fn builtin(name: &str) -> TypedType {
        TypedType::Value(TypedValueType {
            package: Some(Package::global()),
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

    pub fn unsafe_pointer(typ: TypedType) -> TypedType {
        TypedType::Value(TypedValueType {
            package: Some(Package::global()),
            name: UNSAFE_POINTER.to_string(),
            type_args: Some(vec![typ]),
        })
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

    pub fn is_function_type(&self) -> bool {
        matches!(self, Self::Function(_))
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
            TypedType::Value(v) => v.is_unsafe_pointer(),
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        Self::bool().eq(self)
    }
}

impl ToString for TypedType {
    fn to_string(&self) -> String {
        match self {
            TypedType::Value(t) => {t.to_string()}
            TypedType::Function(t) => { todo!()}
            TypedType::Type(t) => {format!("Type<{}>", t.to_string())}
            TypedType::Reference(t) => { String::from("&") + &*t.to_string() }
        }
    }
}
