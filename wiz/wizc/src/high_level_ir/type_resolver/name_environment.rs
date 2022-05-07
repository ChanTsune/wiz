use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::type_resolver::declaration::DeclarationItemKind;
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

    /// use [namespace]::*;
    pub(crate) fn use_asterisk<T: ToString>(&mut self, namespace: &[T]) -> Option<()> {
        let ns_id = self.arena.resolve_namespace_from_root(namespace)?;
        let ns = self.arena.get_by_id(&ns_id).unwrap();
        let ns = if let DeclarationItemKind::Namespace(ns) = ns {
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
        name_space: Vec<String>,
        type_name: &str,
    ) -> Option<&ResolverStruct> {
        let maybe_type_parameter = match self.local_names.get(type_name) {
            Some(EnvValue::Type(rs)) => Some(rs),
            _ => None,
        };
        let n = match maybe_type_parameter {
            None => self.arena.get_type(&name_space, type_name),
            Some(tp) => Some(tp),
        };
        n
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ResolverStruct> {
        self.get_type(typ.package().into_resolved().names, &typ.name())
    }

    pub(crate) fn get_env_item<T: ToString>(
        &self,
        namespace: &[T],
        name: &str,
    ) -> Option<EnvValue> {
        if namespace.is_empty() {
            let maybe_local_value = self.local_names.get(name).cloned();
            match maybe_local_value {
                None => {
                    let ids = self.values.get(name)?;
                    let ids = ids.iter().collect::<Vec<_>>();
                    let items = self.arena.get_by_ids(&ids)?;
                    if !items.is_empty() {
                        if let DeclarationItemKind::Type(t) = items.first().unwrap() {
                            return Some(EnvValue::from(t.clone()));
                        } else {
                            let mut values = HashSet::new();
                            for item in items {
                                if let DeclarationItemKind::Value(v) = item {
                                    values.insert(v.clone());
                                } else {
                                    None?
                                }
                            }
                            return Some(EnvValue::from(values));
                        }
                    };
                    None
                }
                Some(t) => Some(t),
            }
        } else {
            let ids = self.values.get(&namespace[0].to_string())?;
            let ids = ids.iter().copied().collect::<Vec<_>>();
            let parent_id = ids.first()?;
            let id = self.arena.resolve_namespace(*parent_id, &namespace[1..])?;
            let item = self.arena.get_by_id(&id)?;
            let child = match item {
                DeclarationItemKind::Namespace(ns) => ns.get_child(name),
                DeclarationItemKind::Type(_) => panic!(),
                DeclarationItemKind::Value(_) => panic!(),
            }?;
            let child = child.iter().collect::<Vec<_>>();
            let items = self.arena.get_by_ids(&child)?;
            if !items.is_empty() {
                if let DeclarationItemKind::Type(t) = items.first().unwrap() {
                    return Some(EnvValue::from(t.clone()));
                } else {
                    let mut values = HashSet::new();
                    for item in items {
                        if let DeclarationItemKind::Value(v) = item {
                            values.insert(v.clone());
                        } else {
                            None?
                        }
                    }
                    return Some(EnvValue::from(values));
                }
            };
            None
        }
    }
}
