use crate::high_level_ir::type_resolver::context::ResolverStruct;
use crate::high_level_ir::type_resolver::namespace::Namespace;
use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeclarationItem {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) name: String,
    pub(crate) kind: DeclarationItemKind,
}

impl DeclarationItem {
    pub(crate) fn new(
        annotations: TypedAnnotations,
        name: &str,
        kind: DeclarationItemKind,
    ) -> Self {
        Self {
            annotations,
            name: name.to_string(),
            kind,
        }
    }

    pub(crate) fn has_annotation(&self, annotation: &str) -> bool {
        self.annotations.has_annotate(annotation)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclarationItemKind {
    Namespace(Namespace),
    Type(ResolverStruct),
    Value((Vec<String> /* namespace */, TypedType)),
}
