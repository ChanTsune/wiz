use crate::builder::error::BResult;
use crate::ml_decl::{MLArgDef, MLFun, MLFunBody};
use crate::ml_type::MLValueType;
use crate::statement::MLStmt;

#[derive(Debug, Clone)]
pub struct FunBuilder {
    modifiers: Vec<String>,
    name: String,
    arg_defs: Vec<MLArgDef>,
    return_type: MLValueType,
    stmts: Vec<MLStmt>,
    declare: bool,
}

impl FunBuilder {
    pub fn new(name: String, arg_defs: Vec<MLArgDef>, return_type: MLValueType) -> Self {
        Self {
            modifiers: vec![],
            name,
            arg_defs,
            return_type,
            stmts: vec![],
            declare: true,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn build_stmt(&mut self, statement: MLStmt) -> BResult<()> {
        self.declare = false;
        Ok(self.stmts.push(statement))
    }

    pub fn build(self) -> (MLFun, Option<MLFun>) {
        let f = MLFun {
            modifiers: self.modifiers,
            name: self.name,
            arg_defs: self.arg_defs,
            return_type: self.return_type,
            body: if self.declare {
                None
            } else {
                Some(MLFunBody { body: self.stmts })
            },
        };
        if self.declare {
            (f, None)
        } else {
            (
                MLFun {
                    modifiers: f.modifiers.clone(),
                    name: f.name.clone(),
                    arg_defs: f.arg_defs.clone(),
                    return_type: f.return_type.clone(),
                    body: None,
                },
                Some(f),
            )
        }
    }
}

impl From<MLFun> for FunBuilder {
    fn from(f: MLFun) -> Self {
        let (stmts, declare) = match f.body {
            None => (vec![], true),
            Some(b) => (b.body, false),
        };
        Self {
            modifiers: f.modifiers,
            name: f.name,
            arg_defs: f.arg_defs,
            return_type: f.return_type,
            stmts,
            declare,
        }
    }
}
