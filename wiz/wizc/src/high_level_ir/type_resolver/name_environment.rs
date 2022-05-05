use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::type_resolver::name_space::NameSpace;
use crate::high_level_ir::typed_type::TypedType;
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::HashMap;
use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::declaration::Declaration;

#[derive(Debug, Clone)]
pub struct NameEnvironment<'a> {
    names: HashMap<String, (Vec<String>, EnvValue)>,
    used_namespaces: Vec<Vec<String>>,
    values: HashMap<String, Vec<DeclarationId>>,
    arena: &'a ResolverArena,
}

impl<'a> NameEnvironment<'a> {
    pub fn new(arena: &'a ResolverArena) -> Self {
        Self {
            names: Default::default(),
            used_namespaces: Default::default(),
            values: Default::default(),
            arena,
        }
    }

    pub(crate) fn use_values_from(&mut self, name_space: &NameSpace) {
        self.names.extend(
            name_space
                .values
                .iter()
                .map(|(k, v)| (k.clone(), (name_space.name_space.clone(), v.clone()))),
        );
    }

    /// use [namespace]::*;
    pub(crate) fn use_asterisk<T: ToString>(&mut self, namespace: &[T]) {
        let ns_id = self.arena.resolve_namespace_from_root(namespace).unwrap();
        let ns = self.arena.get_by_id(&ns_id).unwrap();
        let ns = if let Declaration::Namespace(ns) = ns { ns } else { panic!("{:?}", ns) };
        self.values.extend(ns.children().clone());
    }

    /// use [namespace]::[name];
    pub(crate) fn use_<T: ToString>(&mut self, namespace:&[T], name: &str) {
        if name == "*" {
            self.use_asterisk(namespace);
        } else {
            let ns_id = self.arena.resolve_namespace_from_root(namespace).unwrap();
            let entry = self.values.entry(name.to_string()).or_default();
            entry.push(ns_id)
        }
    }

    pub(crate) fn use_values_from_local(&mut self, local_stack: &StackedHashMap<String, EnvValue>) {
        self.names.extend(
            local_stack
                .clone()
                .into_map()
                .into_iter()
                .map(|(k, v)| (k, (vec![], v))),
        )
    }

    pub(crate) fn get_type(
        &self,
        mut name_space: Vec<String>,
        type_name: &str,
    ) -> Option<&ResolverStruct> {
        self.arena.get_type(&name_space, type_name)
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ResolverStruct> {
        self.get_type(typ.package().into_resolved().names, &typ.name())
    }

    pub(crate) fn get_env_value(&self, name: &str) -> Option<&(Vec<String>, EnvValue)> {
        self.names.get(name)
    }

    pub(crate) fn add_env_value(&mut self, name: &str, v: (Vec<String>, EnvValue)) {
        self.names.insert(name.to_string(), v);
    }
}
