use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::statement::MLStmt;
use crate::middle_level_ir::ml_type::{MLPrimitiveType, MLType, MLValueType};
use std::fmt::{Result, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLBlock {
    pub(crate) body: Vec<MLStmt>,
}

impl MLBlock {
    pub(crate) fn r#type(&self) -> MLType {
        if let Some(stmt) = self.body.last() {
            if let MLStmt::Expr(expr) = stmt {
                expr.type_()
            } else {
                MLType::Value(MLValueType::Primitive(MLPrimitiveType::Unit))
            }
        } else {
            MLType::Value(MLValueType::Primitive(MLPrimitiveType::Unit))
        }
    }
}

impl MLNode for MLBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("{\n")?;
        f.indent_level_up();
        for stmt in self.body.iter() {
            stmt.fmt(f)?;
            f.write_str(";\n")?;
        }
        f.indent_level_down();
        f.write_char('}')
    }
}
