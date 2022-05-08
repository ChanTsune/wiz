use crate::high_level_ir::type_resolver::context::ResolverStruct;
use crate::high_level_ir::type_resolver::namespace::Namespace;
use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeclarationItem {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) kind: DeclarationItemKind,
}

impl DeclarationItem {
    pub(crate) fn new(kind: DeclarationItemKind) -> Self {
        Self {
            annotations: Default::default(),
            kind
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclarationItemKind {
    Namespace(Namespace),
    Type(ResolverStruct),
    Value((Vec<String> /* namespace */, TypedType)),
}
