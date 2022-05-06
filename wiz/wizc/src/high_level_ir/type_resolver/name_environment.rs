use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::type_resolver::declaration::DeclarationItem;
use crate::high_level_ir::type_resolver::name_space::NameSpace;
use crate::high_level_ir::typed_type::TypedType;
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct NameEnvironment<'a> {
    local_names: HashMap<String, EnvValue>,
    values: HashMap<String, HashSet<DeclarationId>>,
    arena: &'a ResolverArena,
}

impl<'a> NameEnvironment<'a> {
    pub fn new(arena: &'a ResolverArena) -> Self {
        Self {
            local_names: Default::default(),
            values: Default::default(),
            arena,
        }
    }

    pub(crate) fn use_values_from(&mut self, name_space: &NameSpace) {
        self.local_names.extend(name_space.values.clone());
    }

    /// use [namespace]::*;
    pub(crate) fn use_asterisk<T: ToString>(&mut self, namespace: &[T]) -> Option<()> {
        let ns_id = self.arena.resolve_namespace_from_root(namespace)?;
        let ns = self.arena.get_by_id(&ns_id).unwrap();
        let ns = if let DeclarationItem::Namespace(ns) = ns {
            ns
        } else {
            panic!("{:?}", ns)
        };
        self.values.extend(ns.children().clone());
        Some(())
    }

    /// use [namespace]::[name];
    pub(crate) fn use_<T: ToString>(&mut self, fqn: &[T]) {
        if fqn.last().map(T::to_string) == Some("*".to_string()) {
            self.use_asterisk(&fqn[..fqn.len() - 1]);
        } else {
            let item = self.arena.resolve_declaration_id_from_root(fqn).unwrap();
            let entry = self
                .values
                .entry(fqn.last().unwrap().to_string())
                .or_default();
            entry.insert(item);
        }
    }

    pub(crate) fn use_values_from_local(&mut self, local_stack: &StackedHashMap<String, EnvValue>) {
        self.local_names.extend(local_stack.clone().into_map())
    }

    pub(crate) fn get_type(
        &self,
        mut name_space: Vec<String>,
        type_name: &str,
    ) -> Option<&ResolverStruct> {
        let maybe_type_parameter = match self.local_names.get(type_name) {
            Some(EnvValue::Type(rs)) => Some(rs),
            _ => None,
        };
        match maybe_type_parameter {
            None => self.arena.get_type(&name_space, type_name),
            Some(tp) => Some(tp),
        }
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ResolverStruct> {
        self.get_type(typ.package().into_resolved().names, &typ.name())
    }

    pub(crate) fn get_env_value(&self, name: &str) -> Option<EnvValue> {
        self.local_names.get(name).cloned()
    }

    pub(crate) fn add_env_value(&mut self, name: &str, v: EnvValue) {
        self.local_names.insert(name.to_string(), v);
    }
}
