use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::context::ResolverStruct;
use crate::high_level_ir::type_resolver::namespace::Namespace;
use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_type::TypedType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeclarationItem {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) name: String,
    pub(crate) kind: DeclarationItemKind,
    parent: Option<DeclarationId>,
    children: HashMap<String, HashSet<DeclarationId>>,
}

impl DeclarationItem {
    pub(crate) fn new(
        annotations: TypedAnnotations,
        name: &str,
        kind: DeclarationItemKind,
        parent: Option<DeclarationId>,
    ) -> Self {
        Self {
            annotations,
            name: name.to_string(),
            kind,
            children: Default::default(),
            parent,
        }
    }

    pub(crate) fn has_annotation(&self, annotation: &str) -> bool {
        self.annotations.has_annotate(annotation)
    }

    pub fn add_child(&mut self, name: &str, id: DeclarationId) {
        let entry = self.children.entry(name.to_string()).or_default();
        entry.insert(id);
    }

    pub fn get_child(&self, name: &str) -> Option<HashSet<DeclarationId>> {
        self.children.get(name).cloned()
    }

    pub fn children(&self) -> &HashMap<String, HashSet<DeclarationId>> {
        &self.children
    }

    pub fn is_namespace(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Namespace(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Type(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Value(_))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclarationItemKind {
    Namespace(Namespace),
    Type(ResolverStruct),
    Value((Vec<String> /* namespace */, TypedType)),
}
