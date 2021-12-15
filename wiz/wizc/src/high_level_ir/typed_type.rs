use crate::constants::{self, UNSAFE_POINTER};
use crate::high_level_ir::typed_decl::TypedArgDef;
use std::option::Option::Some;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedPackage {
    Raw(Package),
    Resolved(Package),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Package {
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedType {
    Value(TypedNamedValueType),
    Function(Box<TypedFunctionType>),
    Type(TypedNamedValueType),
    Reference(TypedNamedValueType),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum _TypedType {
    Self_,
    Value(TypedValueType),
    Function(Box<_TypedFunctionType>),
    Type(Box<_TypedType>),
}

impl _TypedType {
    pub(crate) fn is_self(&self) -> bool {
        matches!(self, Self::Self_)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedValueType {
    Value(TypedNamedValueType), // Primitive | Struct | Union | Enum
    Array(Box<_TypedType>),
    Tuple(Vec<_TypedType>),
    Pointer(Box<_TypedType>),
    Reference(Box<_TypedType>),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct _TypedFunctionType {
    pub args_type: Vec<TypedArgType>,
    pub return_type: _TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedArgType {
    pub label: String,
    pub typ: _TypedType,
}

impl TypedArgType {
    pub(crate) fn is_self(&self) -> bool {
        self.typ.is_self()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedNamedValueType {
    pub(crate) package: TypedPackage,
    pub(crate) name: String,
    pub(crate) type_args: Option<Vec<TypedType>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedFunctionType {
    pub(crate) arguments: Vec<TypedArgDef>,
    pub(crate) return_type: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedTypeParam {
    pub(crate) name: String,
    pub(crate) type_constraint: Vec<TypedType>,
}

impl TypedPackage {
    pub(crate) fn is_raw(&self) -> bool {
        matches!(self, Self::Raw(_))
    }

    pub(crate) fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }

    pub(crate) fn into_raw(self) -> Package {
        match self {
            TypedPackage::Raw(p) => p,
            TypedPackage::Resolved(_) => {
                panic!()
            }
        }
    }

    pub(crate) fn into_resolved(self) -> Package {
        match self {
            TypedPackage::Raw(_) => {
                panic!()
            }
            TypedPackage::Resolved(p) => p,
        }
    }
}

impl Package {
    pub(crate) fn new() -> Self {
        Self { names: vec![] }
    }

    pub(crate) fn global() -> Self {
        Self { names: vec![] }
    }

    pub(crate) fn is_global(&self) -> bool {
        self.names.is_empty()
    }
}

impl<T> From<Vec<T>> for Package
where
    T: ToString,
{
    fn from(names: Vec<T>) -> Self {
        Self {
            names: names.into_iter().map(|name| name.to_string()).collect(),
        }
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
        self.names.join("::")
    }
}

impl TypedNamedValueType {
    fn builtin(name: &str) -> Self {
        Self {
            package: TypedPackage::Resolved(Package::global()),
            name: String::from(name),
            type_args: None,
        }
    }

    pub(crate) fn noting() -> Self {
        Self::builtin(constants::NOTING)
    }

    pub(crate) fn unit() -> Self {
        Self::builtin(constants::UNIT)
    }

    pub(crate) fn int8() -> Self {
        Self::builtin(constants::INT8)
    }

    pub(crate) fn int16() -> Self {
        Self::builtin(constants::INT16)
    }

    pub(crate) fn int32() -> Self {
        Self::builtin(constants::INT32)
    }

    pub(crate) fn int64() -> Self {
        Self::builtin(constants::INT64)
    }

    pub(crate) fn uint8() -> Self {
        Self::builtin(constants::UINT8)
    }

    pub(crate) fn uint16() -> Self {
        Self::builtin(constants::UINT16)
    }

    pub(crate) fn uint32() -> Self {
        Self::builtin(constants::UINT32)
    }

    pub(crate) fn uint64() -> Self {
        Self::builtin(constants::UINT64)
    }

    pub(crate) fn float() -> Self {
        Self::builtin(constants::F32)
    }

    pub(crate) fn double() -> Self {
        Self::builtin(constants::F64)
    }

    pub(crate) fn bool() -> Self {
        Self::builtin(constants::BOOL)
    }

    pub(crate) fn string() -> Self {
        Self::builtin(constants::STRING)
    }

    pub(crate) fn is_unsafe_pointer(&self) -> bool {
        self.name == UNSAFE_POINTER
            && if let TypedPackage::Resolved(pkg) = &self.package {
                pkg.is_global()
            } else {
                false
            }
    }

    pub(crate) fn is_string(&self) -> bool {
        Self::string().eq(self)
    }
}

impl ToString for TypedNamedValueType {
    fn to_string(&self) -> String {
        let fqn = match &self.package {
            TypedPackage::Raw(pkg) | TypedPackage::Resolved(pkg) => {
                if pkg.is_global() {
                    self.name.clone()
                } else {
                    pkg.to_string() + "::" + &*self.name
                }
            }
        };
        fqn + &match &self.type_args {
            None => String::new(),
            Some(a) => {
                String::from("<")
                    + &a.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                    + ">"
            }
        }
    }
}

impl TypedType {
    pub fn noting() -> Self {
        Self::Value(TypedNamedValueType::noting())
    }

    pub fn unit() -> Self {
        Self::Value(TypedNamedValueType::unit())
    }

    pub fn int8() -> Self {
        Self::Value(TypedNamedValueType::int8())
    }

    pub fn int16() -> Self {
        Self::Value(TypedNamedValueType::int16())
    }

    pub fn int32() -> Self {
        Self::Value(TypedNamedValueType::int32())
    }

    pub fn int64() -> Self {
        Self::Value(TypedNamedValueType::int64())
    }

    pub fn uint8() -> Self {
        Self::Value(TypedNamedValueType::uint8())
    }

    pub fn uint16() -> Self {
        Self::Value(TypedNamedValueType::uint16())
    }

    pub fn uint32() -> Self {
        Self::Value(TypedNamedValueType::uint32())
    }

    pub fn uint64() -> Self {
        Self::Value(TypedNamedValueType::uint64())
    }

    pub fn float() -> Self {
        Self::Value(TypedNamedValueType::float())
    }

    pub fn double() -> Self {
        Self::Value(TypedNamedValueType::double())
    }

    pub fn bool() -> Self {
        Self::Value(TypedNamedValueType::bool())
    }

    pub fn string() -> Self {
        Self::Value(TypedNamedValueType::string())
    }

    pub fn unsafe_pointer(typ: TypedType) -> Self {
        Self::Value(TypedNamedValueType {
            package: TypedPackage::Resolved(Package::global()),
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

    pub fn is_string(&self) -> bool {
        Self::string().eq(self)
    }
}

impl ToString for TypedType {
    fn to_string(&self) -> String {
        match self {
            TypedType::Value(t) => t.to_string(),
            TypedType::Function(t) => {
                todo!()
            }
            TypedType::Type(t) => {
                format!("Type<{}>", t.to_string())
            }
            TypedType::Reference(t) => format!("&{}", t.to_string()),
        }
    }
}
