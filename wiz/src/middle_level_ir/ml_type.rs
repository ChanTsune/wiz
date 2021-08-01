use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum MLType {
    Value(MLValueType),
    Function(MLFunctionType),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct MLValueType {
    pub(crate) name: String,
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
