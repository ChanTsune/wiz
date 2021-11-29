use crate::format::Formatter;
use crate::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLValueType {
    Primitive(MLPrimitiveType),
    Struct(String),
    Pointer(Box<MLValueType>),
    Reference(Box<MLValueType>),
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
        matches!(self, MLValueType::Struct(_))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLFunctionType {
    pub arguments: Vec<MLValueType>,
    pub return_type: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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
