use std::fmt::{Write, Result};
use crate::middle_level_ir::expr::{MLBlock, MLExpr};
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::ml_type::MLValueType;
use crate::middle_level_ir::format::Formatter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLIf {
    pub(crate) condition: Box<MLExpr>,
    pub(crate) body: MLBlock,
    pub(crate) else_body: Option<MLBlock>,
    pub(crate) type_: MLValueType,
}

impl MLNode for MLIf {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("if (")?;
        self.condition.fmt(f)?;
        f.write_str(") ")?;
        self.body.fmt(f)?;
        match &self.else_body {
            Some(b) => {
                f.write_str(" else ")?;
                b.fmt(f)?;
            }
            None => {}
        };
        Ok(())
    }
}

