use std::fmt;
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::format::Formatter;
use std::fmt::Write;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLValueType {
    Primitive(String),
    Struct(String),
    Pointer(Box<MLValueType>),
}

impl MLValueType {
    pub(crate) fn name(&self) -> String {
        match self {
            MLValueType::Primitive(name) => name.clone(),
            MLValueType::Struct(name) => name.clone(),
            MLValueType::Pointer(p) => String::from("*") + &*p.name(),
        }
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLFunctionType {
    pub(crate) arguments: Vec<MLValueType>,
    pub(crate) return_type: MLValueType,
}

impl MLType {
    pub fn into_value_type(self) -> MLValueType {
        match self {
            MLType::Value(v) => v,
            MLType::Function(f) => {
                panic!("can not cast to MLValueType")
            }
        }
    }
}

impl MLNode for MLType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLType::Value(v) => {v.fmt(f)}
            MLType::Function(fun) => {fun.fmt(f)}
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
        };
        f.write_char(')')?;
        f.write_str(" -> ")?;
        self.return_type.fmt(f)
    }
}
