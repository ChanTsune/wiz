use crate::typed_annotation::TypedAnnotations;
use crate::typed_type::Package;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedUse {
    pub annotations: TypedAnnotations,
    pub package: Package,
    pub alias: Option<String>,
}
