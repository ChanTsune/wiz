use crate::builder::error::BResult;
use crate::ml_decl::{MLArgDef, MLFun, MLFunBody};
use crate::ml_type::MLValueType;
use crate::statement::MLStmt;

#[derive(Debug, Clone)]
pub struct FunBuilder {
    name: String,
    arg_defs: Vec<MLArgDef>,
    return_type: MLValueType,
    stmts: Vec<MLStmt>,
    declare: bool,
}

impl FunBuilder {
    pub fn new(name: String, arg_defs: Vec<MLArgDef>, return_type: MLValueType) -> Self {
        Self {
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
        self.stmts.push(statement);
        Ok(())
    }

    pub fn build(&self) -> (MLFun, Option<MLFun>) {
        let f = MLFun {
            name: self.name.clone(),
            arg_defs: self.arg_defs.clone(),
            return_type: self.return_type.clone(),
            body: if self.declare {
                None
            } else {
                Some(MLFunBody {
                    body: self.stmts.clone(),
                })
            },
        };
        if self.declare {
            (f, None)
        } else {
            (
                MLFun {
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
            name: f.name,
            arg_defs: f.arg_defs,
            return_type: f.return_type,
            stmts,
            declare,
        }
    }
}
