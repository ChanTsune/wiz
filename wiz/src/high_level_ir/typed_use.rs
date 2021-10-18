use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_type::Package;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypedUse {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: Package,
    pub(crate) alias: Option<String>,
}
