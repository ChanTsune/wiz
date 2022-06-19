use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::context::{ResolverFunction, ResolverStruct};
use std::collections::{HashMap, HashSet};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_type::TypedType;

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

    pub fn parent(&self) -> Option<DeclarationId> {
        self.parent
    }

    pub fn is_namespace(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Namespace)
    }

    pub fn is_type(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Type(_))
    }

    pub fn is_value(&self) -> bool {
        self.is_variable() || self.is_function()
    }

    pub fn is_variable(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Variable(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self.kind, DeclarationItemKind::Function(..))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclarationItemKind {
    Namespace,
    Type(ResolverStruct),
    Variable(TypedType),
    Function(ResolverFunction),
}
