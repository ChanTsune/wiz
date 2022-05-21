use crate::expr::MLExpr;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::MLValueType;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLLiteralKind {
    Integer(String),
    FloatingPoint(String),
    String(String),
    Boolean(String),
    Null,
    Struct(Vec<(String, MLExpr)>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLLiteral {
    pub kind: MLLiteralKind,
    pub type_: MLValueType,
}

impl MLNode for MLLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MLLiteralKind::Integer(value) => f.write_str(value),
            MLLiteralKind::FloatingPoint(value) => f.write_str(value),
            MLLiteralKind::String(value) => {
                f.write_char('"')?;
                f.write_str(value)?;
                f.write_char('"')
            }
            MLLiteralKind::Boolean(value) => f.write_str(value),
            MLLiteralKind::Null => Err(Default::default()),
            MLLiteralKind::Struct(fields) => {
                self.type_.fmt(f)?;
                f.write_char('(')?;
                for field in fields {
                    f.write_str(&field.0)?;
                    f.write_char(':')?;
                    field.1.fmt(f)?;
                }
                f.write_char(')')
            }
        }
    }
}
