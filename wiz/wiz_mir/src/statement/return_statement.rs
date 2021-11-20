use crate::expr::MLExpr;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::{MLPrimitiveType, MLValueType};
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLReturn {
    pub value: Option<Box<MLExpr>>,
}

impl MLReturn {
    pub fn new(value: Option<MLExpr>) -> Self {
        MLReturn {
            value: value.map(Box::new),
        }
    }
    pub fn type_(&self) -> MLValueType {
        MLValueType::Primitive(MLPrimitiveType::Noting)
    }
}

impl MLNode for MLReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("return ")?;
        match &self.value {
            Some(v) => v.fmt(f),
            None => fmt::Result::Ok(()),
        }
    }
}
