use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::{MLPrimitiveType, MLType, MLValueType};
use crate::statement::MLStmt;
use std::fmt::{Result, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLBlock {
    pub body: Vec<MLStmt>,
}

impl MLBlock {
    pub fn r#type(&self) -> MLType {
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
