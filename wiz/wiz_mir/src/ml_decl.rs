use crate::expr::MLExpr;
use crate::format::Formatter;
use crate::ml_node::MLNode;
use crate::ml_type::{MLType, MLValueType};
use crate::statement::MLStmt;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MLDecl {
    Var(MLVar),
    Fun(MLFun),
    Struct(MLStruct),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLVar {
    pub is_mute: bool,
    pub name: String,
    pub type_: MLType,
    pub value: MLExpr,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLFun {
    pub modifiers: Vec<String>,
    pub name: String,
    pub arg_defs: Vec<MLArgDef>,
    pub return_type: MLValueType,
    pub body: Option<MLFunBody>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLArgDef {
    pub name: String,
    pub type_: MLValueType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLFunBody {
    pub body: Vec<MLStmt>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLStruct {
    pub name: String,
    pub fields: Vec<MLField>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MLField {
    pub name: String,
    pub type_: MLValueType,
}

impl MLNode for MLDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLDecl::Var(v) => v.fmt(f),
            MLDecl::Fun(fun) => fun.fmt(f),
            MLDecl::Struct(s) => s.fmt(f),
        }
    }
}

impl MLNode for MLVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(if self.is_mute { "var" } else { "val" })?;
        f.write_char(' ')?;
        f.write_str(&*self.name)?;
        f.write_char(':')?;
        self.type_.fmt(f)?;
        f.write_str(" = ")?;
        self.value.fmt(f)
    }
}

impl MLNode for MLFun {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for modifier in self.modifiers.iter() {
            f.write_str(modifier)?;
            f.write_char(' ')?;
        }
        f.write_str("fun ")?;
        f.write_str(&*self.name)?;
        f.write_char('(')?;
        for (c, arg_def) in self.arg_defs.iter().enumerate() {
            arg_def.fmt(f)?;
            let s = self.arg_defs.len() - 1;
            if c != s {
                f.write_str(", ")?;
            }
        }
        f.write_str("):")?;
        self.return_type.fmt(f)?;
        match &self.body {
            Some(b) => {
                b.fmt(f)?;
            }
            None => {}
        };
        f.write_char(';')
    }
}

impl MLNode for MLArgDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)?;
        f.write_char(':')?;
        self.type_.fmt(f)
    }
}

impl MLNode for MLFunBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(" {\n")?;
        f.indent_level_up();
        for stmt in self.body.iter() {
            stmt.fmt(f)?;
            f.write_str(";\n")?;
        }
        f.indent_level_down();
        f.write_char('}')
    }
}

impl MLNode for MLStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("struct ")?;
        f.write_str(&*self.name)?;
        f.write_str(" {\n")?;
        f.indent_level_up();
        for field in self.fields.iter() {
            field.fmt(f)?;
            f.write_str(",\n")?;
        }
        f.indent_level_down();
        f.write_str("};")
    }
}

impl MLNode for MLField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)?;
        f.write_str(":")?;
        self.type_.fmt(f)
    }
}
