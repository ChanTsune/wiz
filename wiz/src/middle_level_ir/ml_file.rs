use crate::middle_level_ir::ml_decl::MLDecl;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFile {
    pub(crate) body: Vec<MLDecl>,
}
