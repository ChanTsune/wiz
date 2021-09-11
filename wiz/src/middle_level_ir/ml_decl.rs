use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::ml_stmt::MLStmt;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;
use std::fmt::Write;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLDecl {
    Var(MLVar),
    Fun(MLFun),
    Struct(MLStruct),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLVar {
    pub(crate) is_mute: bool,
    pub(crate) name: String,
    pub(crate) type_: MLType,
    pub(crate) value: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFun {
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) arg_defs: Vec<MLArgDef>,
    pub(crate) return_type: MLType,
    pub(crate) body: Option<MLFunBody>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLArgDef {
    pub(crate) name: String,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFunBody {
    pub(crate) body: Vec<MLStmt>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLStruct {
    pub(crate) name: String,
    pub(crate) fields: Vec<MLField>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLField {
    pub(crate) name: String,
    pub(crate) type_: MLType,
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
        for arg_def in self.arg_defs.iter() {
            arg_def.fmt(f)?;
        }
        f.write_str("):")?;
        self.return_type.fmt(f)?;
        match &self.body {
            Some(b) => b.fmt(f),
            None => fmt::Result::Ok(()),
        }
    }
}

impl MLNode for MLArgDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)?;
        self.type_.fmt(f)
    }
}

impl MLNode for MLFunBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("{\n")?;
        for stmt in self.body.iter() {
            stmt.fmt(f)?;
            f.write_char('\n')?;
        }
        f.write_char('}')
    }
}

impl MLNode for MLStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("struct {\n")?;
        f.write_str(&*self.name)?;
        for field in self.fields.iter() {
            field.fmt(f)?;
            f.write_str(";\n")?;
        }
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
