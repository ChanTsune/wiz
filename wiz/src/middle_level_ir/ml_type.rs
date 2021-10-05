use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLValueType {
    Primitive(MLPrimitiveType),
    Struct(String),
    Pointer(Box<MLValueType>),
    Reference(Box<MLValueType>),
}

impl MLValueType {
    pub(crate) fn name(&self) -> String {
        match self {
            MLValueType::Primitive(primitive) => primitive.to_string(),
            MLValueType::Struct(name) => name.clone(),
            MLValueType::Pointer(p) => String::from("*") + &*p.name(),
            MLValueType::Reference(r) => String::from("&") + &*r.name(),
        }
    }

    pub(crate) fn is_struct(&self) -> bool {
        matches!(self, MLValueType::Struct(_))
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLFunctionType {
    pub(crate) arguments: Vec<MLValueType>,
    pub(crate) return_type: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLPrimitiveType {
    Void,
    Unit,
    Int8,
    Int16,
    Int32,
    Int64,
    Size,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    USize,
    Float,
    Double,
    Bool,
    String,
}

impl ToString for MLPrimitiveType {
    fn to_string(&self) -> String {
        String::from(match self {
            MLPrimitiveType::Int8 => "Int8",
            MLPrimitiveType::Int16 => "Int16",
            MLPrimitiveType::Int32 => "Int32",
            MLPrimitiveType::Int64 => "Int64",
            MLPrimitiveType::Size => "Size",
            MLPrimitiveType::UInt8 => "UInt8",
            MLPrimitiveType::UInt16 => "UInt16",
            MLPrimitiveType::UInt32 => "UInt32",
            MLPrimitiveType::UInt64 => "UInt64",
            MLPrimitiveType::USize => "USize",
            MLPrimitiveType::Float => "Float",
            MLPrimitiveType::Double => "Double",
            MLPrimitiveType::Bool => "Bool",
            MLPrimitiveType::String => "String",
            MLPrimitiveType::Unit => "Unit",
            MLPrimitiveType::Void => "Void",
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
