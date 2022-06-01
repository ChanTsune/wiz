use serde::{Deserialize, Serialize};
use wiz_constants as constants;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum TypedPackage {
    Raw(Package),
    Resolved(Package),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct Package {
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum TypedType {
    Self_,
    Value(TypedValueType),
    Function(Box<TypedFunctionType>),
    Type(Box<TypedType>),
}

impl TypedType {
    pub fn is_self(&self) -> bool {
        matches!(self, Self::Self_)
    }

    pub fn is_generic(&self) -> bool {
        match self {
            TypedType::Self_ => false,
            TypedType::Value(v) => v.is_generic(),
            TypedType::Function(f) => f.is_generic(),
            TypedType::Type(t) => t.is_generic(),
        }
    }

    pub fn package(&self) -> TypedPackage {
        match self {
            TypedType::Self_ => panic!(),
            TypedType::Value(v) => v.package(),
            TypedType::Function(_) => todo!(),
            TypedType::Type(v) => v.package(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            TypedType::Self_ => panic!(),
            TypedType::Value(v) => v.name(),
            TypedType::Function(_) => todo!(),
            TypedType::Type(v) => v.name(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum TypedValueType {
    Value(TypedNamedValueType), // Primitive | Struct | Union | Enum
    Array(Box<TypedType>, usize),
    Tuple(Vec<TypedType>),
    Pointer(Box<TypedType>),
    Reference(Box<TypedType>),
}

impl TypedValueType {
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

    pub fn size() -> Self {
        Self::Value(TypedNamedValueType::size())
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

    pub fn usize() -> Self {
        Self::Value(TypedNamedValueType::usize())
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

    pub fn is_unsafe_pointer(&self) -> bool {
        matches!(self, Self::Pointer(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn is_generic(&self) -> bool {
        match self {
            TypedValueType::Value(v) => v.is_generic(),
            TypedValueType::Array(v, _) => v.is_generic(),
            TypedValueType::Tuple(t) => todo!(),
            TypedValueType::Pointer(v) => v.is_generic(),
            TypedValueType::Reference(v) => v.is_generic(),
        }
    }

    pub fn package(&self) -> TypedPackage {
        match self {
            TypedValueType::Value(v) => v.package.clone(),
            TypedValueType::Array(_, _) | TypedValueType::Tuple(_) => {
                TypedPackage::Resolved(Package::global())
            }
            TypedValueType::Pointer(v) | TypedValueType::Reference(v) => v.package(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            TypedValueType::Value(v) => v.name.clone(),
            TypedValueType::Array(_, _) => todo!(),
            TypedValueType::Tuple(_) => todo!(),
            TypedValueType::Pointer(v) | TypedValueType::Reference(v) => v.name(),
        }
    }
}

impl ToString for TypedValueType {
    fn to_string(&self) -> String {
        match self {
            TypedValueType::Value(v) => v.to_string(),
            TypedValueType::Array(t, len) => {
                format!("[{};{}]", t.to_string(), len)
            }
            TypedValueType::Tuple(_) => {
                todo!()
            }
            TypedValueType::Pointer(v) => {
                format!("*{}", v.to_string())
            }
            TypedValueType::Reference(v) => {
                format!("&{}", v.to_string())
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedFunctionType {
    pub arguments: Vec<TypedArgType>,
    pub return_type: TypedType,
}

impl TypedFunctionType {
    pub fn is_generic(&self) -> bool {
        self.arguments.iter().any(|a| a.is_generic()) || self.return_type.is_generic()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedArgType {
    pub label: String,
    pub typ: TypedType,
}

impl TypedArgType {
    pub fn is_self(&self) -> bool {
        self.typ.is_self()
    }

    pub fn is_generic(&self) -> bool {
        self.typ.is_generic()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedNamedValueType {
    pub package: TypedPackage,
    pub name: String,
    pub type_args: Option<Vec<TypedType>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedTypeParam {
    pub name: String,
}

impl TypedPackage {
    pub fn is_raw(&self) -> bool {
        matches!(self, Self::Raw(_))
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }

    pub fn into_raw(self) -> Package {
        match self {
            TypedPackage::Raw(p) => p,
            TypedPackage::Resolved(_) => {
                panic!("cannot convert resolved package to raw ({:?})", self)
            }
        }
    }

    pub fn into_resolved(self) -> Package {
        match self {
            TypedPackage::Raw(_) => panic!("cannot convert raw package to resolved ({:?})", self),
            TypedPackage::Resolved(p) => p,
        }
    }
}

impl Package {
    pub fn new() -> Self {
        Self { names: vec![] }
    }

    pub fn global() -> Self {
        Self { names: vec![] }
    }

    pub fn is_global(&self) -> bool {
        self.names.is_empty()
    }
}

impl<T: ToString> From<&[T]> for Package {
    fn from(names: &[T]) -> Self {
        Self {
            names: names.iter().map(T::to_string).collect(),
        }
    }
}

impl<T: ToString> From<&Vec<T>> for Package {
    fn from(names: &Vec<T>) -> Self {
        Self {
            names: names.iter().map(T::to_string).collect(),
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

    pub fn noting() -> Self {
        Self::builtin(constants::NOTING)
    }

    pub fn unit() -> Self {
        Self::builtin(constants::UNIT)
    }

    pub fn int8() -> Self {
        Self::builtin(constants::INT8)
    }

    pub fn int16() -> Self {
        Self::builtin(constants::INT16)
    }

    pub fn int32() -> Self {
        Self::builtin(constants::INT32)
    }

    pub fn int64() -> Self {
        Self::builtin(constants::INT64)
    }

    pub fn size() -> Self {
        Self::builtin(constants::SIZE)
    }

    pub fn uint8() -> Self {
        Self::builtin(constants::UINT8)
    }

    pub fn uint16() -> Self {
        Self::builtin(constants::UINT16)
    }

    pub fn uint32() -> Self {
        Self::builtin(constants::UINT32)
    }

    pub fn uint64() -> Self {
        Self::builtin(constants::UINT64)
    }

    pub fn usize() -> Self {
        Self::builtin(constants::USIZE)
    }

    pub fn float() -> Self {
        Self::builtin(constants::F32)
    }

    pub fn double() -> Self {
        Self::builtin(constants::F64)
    }

    pub fn bool() -> Self {
        Self::builtin(constants::BOOL)
    }

    pub fn string() -> Self {
        Self::builtin(constants::STRING)
    }

    pub fn is_string(&self) -> bool {
        Self::string().eq(self)
    }

    pub fn is_generic(&self) -> bool {
        self.type_args.is_some()
    }
}

impl ToString for TypedNamedValueType {
    fn to_string(&self) -> String {
        let fqn = match &self.package {
            TypedPackage::Raw(pkg) | TypedPackage::Resolved(pkg) => {
                if pkg.is_global() {
                    self.name.clone()
                } else {
                    format!("{}::{}", pkg.to_string(), self.name)
                }
            }
        };
        fqn + &match &self.type_args {
            None => String::new(),
            Some(a) => format!(
                "<{}>",
                a.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl TypedType {
    pub fn noting() -> Self {
        Self::Value(TypedValueType::noting())
    }

    pub fn unit() -> Self {
        Self::Value(TypedValueType::unit())
    }

    pub fn int8() -> Self {
        Self::Value(TypedValueType::int8())
    }

    pub fn int16() -> Self {
        Self::Value(TypedValueType::int16())
    }

    pub fn int32() -> Self {
        Self::Value(TypedValueType::int32())
    }

    pub fn int64() -> Self {
        Self::Value(TypedValueType::int64())
    }

    pub fn size() -> Self {
        Self::Value(TypedValueType::size())
    }

    pub fn uint8() -> Self {
        Self::Value(TypedValueType::uint8())
    }

    pub fn uint16() -> Self {
        Self::Value(TypedValueType::uint16())
    }

    pub fn uint32() -> Self {
        Self::Value(TypedValueType::uint32())
    }

    pub fn uint64() -> Self {
        Self::Value(TypedValueType::uint64())
    }

    pub fn usize() -> Self {
        Self::Value(TypedValueType::usize())
    }

    pub fn float() -> Self {
        Self::Value(TypedValueType::float())
    }

    pub fn double() -> Self {
        Self::Value(TypedValueType::double())
    }

    pub fn bool() -> Self {
        Self::Value(TypedValueType::bool())
    }

    pub fn string() -> Self {
        Self::Value(TypedValueType::string())
    }

    pub fn string_ref() -> Self {
        Self::Value(TypedValueType::Reference(Box::new(Self::string())))
    }

    pub fn unsafe_pointer(typ: TypedType) -> Self {
        Self::Value(TypedValueType::Pointer(Box::new(typ)))
    }

    pub fn reference(typ: TypedType) -> Self {
        Self::Value(TypedValueType::Reference(Box::new(typ)))
    }

    pub fn signed_integer_types() -> Vec<TypedType> {
        vec![
            Self::int8(),
            Self::int16(),
            Self::int32(),
            Self::int64(),
            Self::size(),
        ]
    }

    pub fn unsigned_integer_types() -> Vec<TypedType> {
        vec![
            Self::uint8(),
            Self::uint16(),
            Self::uint32(),
            Self::uint64(),
            Self::usize(),
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
        self.is_signed_integer() || self.is_unsigned_integer()
    }

    pub fn is_pointer_type(&self) -> bool {
        match self {
            TypedType::Value(v) => v.is_unsafe_pointer(),
            _ => false,
        }
    }

    pub fn is_array_type(&self) -> bool {
        match self {
            TypedType::Value(v) => v.is_array(),
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        Self::bool().eq(self)
    }

    pub fn is_string(&self) -> bool {
        Self::string().eq(self)
    }

    pub fn is_string_ref(&self) -> bool {
        Self::string_ref().eq(self)
    }
}

impl ToString for TypedType {
    fn to_string(&self) -> String {
        match self {
            TypedType::Value(t) => t.to_string(),
            TypedType::Function(_) => todo!(),
            TypedType::Self_ => todo!(),
            TypedType::Type(t) => {
                format!("Type<{}>", t.to_string())
            }
        }
    }
}
