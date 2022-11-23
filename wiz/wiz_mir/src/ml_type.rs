use crate::format::Formatter;
use crate::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;
use wiz_constants as constants;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

impl MLType {
    fn name(&self) -> String {
        match self {
            Self::Value(v) => v.name(),
            Self::Function(f) => f.name(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLValueType {
    Primitive(MLPrimitiveType),
    Struct(String),
    Pointer(Box<MLType>),
    Reference(Box<MLType>),
    Array(Box<MLValueType>, usize),
}

impl MLValueType {
    pub fn name(&self) -> String {
        match self {
            MLValueType::Primitive(primitive) => primitive.to_string(),
            MLValueType::Struct(name) => name.clone(),
            MLValueType::Pointer(p) => format!("*{}", p.name()),
            MLValueType::Reference(r) => format!("&{}", r.name()),
            MLValueType::Array(a, size) => format!("[{};{}]", a.name(), size),
        }
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_signed_integer(&self) -> bool {
        match self {
            Self::Primitive(p) => p.is_signed_integer(),
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLFunctionType {
    pub arguments: Vec<MLValueType>,
    pub return_type: MLValueType,
}

impl MLFunctionType {
    fn name(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum MLPrimitiveType {
    Noting,
    Unit,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Size,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    USize,
    Float,
    Double,
    Bool,
    String,
}

impl MLPrimitiveType {
    pub fn is_signed_integer(&self) -> bool {
        matches!(
            self,
            Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64 | Self::Int128 | Self::Size
        )
    }
}

impl ToString for MLPrimitiveType {
    fn to_string(&self) -> String {
        String::from(match self {
            MLPrimitiveType::Int8 => "i8",
            MLPrimitiveType::Int16 => "i16",
            MLPrimitiveType::Int32 => "i32",
            MLPrimitiveType::Int64 => "i64",
            MLPrimitiveType::Int128 => "i128",
            MLPrimitiveType::Size => "size",
            MLPrimitiveType::UInt8 => "u8",
            MLPrimitiveType::UInt16 => "u16",
            MLPrimitiveType::UInt32 => "u32",
            MLPrimitiveType::UInt64 => "u64",
            MLPrimitiveType::UInt128 => "u128",
            MLPrimitiveType::USize => "usize",
            MLPrimitiveType::Float => "f32",
            MLPrimitiveType::Double => "f64",
            MLPrimitiveType::Bool => "bool",
            MLPrimitiveType::String => "str",
            MLPrimitiveType::Unit => "unit",
            MLPrimitiveType::Noting => "noting",
        })
    }
}

impl TryFrom<&str> for MLPrimitiveType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            constants::NOTING => MLPrimitiveType::Noting,
            constants::UNIT => MLPrimitiveType::Unit,
            constants::INT8 => MLPrimitiveType::Int8,
            constants::UINT8 => MLPrimitiveType::UInt8,
            constants::INT16 => MLPrimitiveType::Int16,
            constants::UINT16 => MLPrimitiveType::UInt16,
            constants::INT32 => MLPrimitiveType::Int32,
            constants::UINT32 => MLPrimitiveType::UInt32,
            constants::INT64 => MLPrimitiveType::Int64,
            constants::UINT64 => MLPrimitiveType::UInt64,
            constants::INT128 => MLPrimitiveType::Int128,
            constants::UINT128 => MLPrimitiveType::UInt128,
            constants::SIZE => MLPrimitiveType::Size,
            constants::USIZE => MLPrimitiveType::USize,
            constants::BOOL => MLPrimitiveType::Bool,
            constants::F32 => MLPrimitiveType::Float,
            constants::F64 => MLPrimitiveType::Double,
            constants::STRING => MLPrimitiveType::String,
            _ => return Err(()),
        })
    }
}

impl MLType {
    pub fn into_value_type(self) -> MLValueType {
        match self {
            MLType::Value(v) => v,
            MLType::Function(f) => {
                panic!("can not cast to MLValueType => {:?}", f)
            }
        }
    }
}

impl MLNode for MLType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLType::Value(v) => v.fmt(f),
            MLType::Function(fun) => fun.fmt(f),
        }
    }
}

impl MLNode for MLValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name())
    }
}

impl MLNode for MLFunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('(')?;
        for argument in self.arguments.iter() {
            argument.fmt(f)?;
            f.write_str(",")?;
        }
        f.write_char(')')?;
        f.write_str(" -> ")?;
        self.return_type.fmt(f)
    }
}
