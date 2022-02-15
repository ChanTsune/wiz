use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_type::Package;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedUse {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: Package,
    pub(crate) alias: Option<String>,
}
