use crate::middle_level_ir::expr::MLExpr;
use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::ml_type::{MLPrimitiveType, MLValueType};
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLReturn {
    pub(crate) value: Option<Box<MLExpr>>,
}

impl MLReturn {
    pub fn new(expr: MLExpr) -> Self {
        MLReturn {
            value: Some(Box::new(expr)),
        }
    }
    pub(crate) fn type_(&self) -> MLValueType {
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
