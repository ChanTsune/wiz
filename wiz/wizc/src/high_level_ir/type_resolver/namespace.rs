use crate::high_level_ir::declaration_id::DeclarationId;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Namespace {
    name: String,
    parent: Option<DeclarationId>,
    children: HashMap<String, Vec<DeclarationId>>,
}

impl Namespace {
    pub fn root() -> Self {
        Self::default()
    }

    pub fn new(name: &str, parent: DeclarationId) -> Self {
        Self {
            name: name.to_string(),
            parent: Some(parent),
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, name: &str, id: DeclarationId) {
        let entry = self.children.entry(name.to_string()).or_default();
        entry.push(id);
    }

    pub fn get_child(&self, name: &str) -> Option<Vec<DeclarationId>> {
        self.children.get(name).cloned()
    }

    pub fn parent(&self) -> Option<DeclarationId> {
        self.parent
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
