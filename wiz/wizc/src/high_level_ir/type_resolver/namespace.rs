use crate::high_level_ir::declaration_id::DeclarationId;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Namespace {
    parent: Option<DeclarationId>,
    children: HashMap<String, HashSet<DeclarationId>>,
}

impl Namespace {
    pub fn root() -> Self {
        Self::default()
    }

    pub fn new(parent: DeclarationId) -> Self {
        Self {
            parent: Some(parent),
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, name: &str, id: DeclarationId) {
        let entry = self.children.entry(name.to_string()).or_default();
        entry.insert(id);
    }

    pub fn get_child(&self, name: &str) -> Option<HashSet<DeclarationId>> {
        self.children.get(name).cloned()
    }

    pub fn parent(&self) -> Option<DeclarationId> {
        self.parent
    }

    pub fn children(&self) -> &HashMap<String, HashSet<DeclarationId>> {
        &self.children
    }
}
