use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLValueType {
    Name(String),
    Pointer(Box<MLValueType>),
}

impl MLValueType {
    pub(crate) fn name(&self) -> String {
        match self {
            MLValueType::Name(name) => name.clone(),
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
