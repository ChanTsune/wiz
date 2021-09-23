use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_decl::MLVar;
use crate::middle_level_ir::ml_expr::MLExpr;
use crate::middle_level_ir::ml_node::MLNode;
use std::fmt;
use std::fmt::Write;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLStmt {
    Expr(MLExpr),
    Var(MLVar),
    Assignment(MLAssignmentStmt),
    Loop(MLLoopStmt),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLAssignmentStmt {
    pub(crate) target: MLExpr,
    pub(crate) value: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLLoopStmt {
    pub(crate) condition: MLExpr,
    pub(crate) block: MLBlock,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLBlock {
    pub(crate) body: Vec<MLStmt>,
}

impl MLNode for MLStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLStmt::Expr(e) => e.fmt(f),
            MLStmt::Var(d) => d.fmt(f),
            MLStmt::Assignment(a) => a.fmt(f),
            MLStmt::Loop(l) => l.fmt(f),
        }
    }
}

impl MLNode for MLAssignmentStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_str(" = ")?;
        self.value.fmt(f)
    }
}

impl MLNode for MLLoopStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("while (")?;
        self.condition.fmt(f)?;
        f.write_str(") ")?;
        self.block.fmt(f)
    }
}

impl MLNode for MLBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
