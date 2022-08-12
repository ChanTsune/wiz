use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::type_resolver::declaration::DeclarationItemKind;
use std::collections::{HashMap, HashSet};
use wiz_hir::typed_type::TypedType;
use wiz_utils::StackedHashMap;

#[derive(Debug, Clone)]
pub(crate) struct NameEnvironment<'a> {
    local_stack: StackedHashMap<String, EnvValue>,
    values: HashMap<String, HashSet<DeclarationId>>,
    arena: &'a ResolverArena,
}

impl<'a> NameEnvironment<'a> {
    pub fn new(arena: &'a ResolverArena, local_stack: StackedHashMap<String, EnvValue>) -> Self {
        Self {
            local_stack,
            values: arena.get_root().children().clone(),
            arena,
        }
    }
}

impl<'a> NameEnvironment<'a> {
    pub fn push(&mut self) -> Self {
        self.local_stack.push(Default::default());
        Self {
            local_stack: self.local_stack.clone(),
            values: self.values.clone(),
            arena: self.arena,
        }
    }

    pub fn pop(&mut self) -> Self {
        self.local_stack.pop();
        Self {
            local_stack: self.local_stack.clone(),
            values: self.values.clone(),
            arena: self.arena,
        }
    }
}

impl<'a> NameEnvironment<'a> {
    /// use [namespace]::*;
    pub(crate) fn use_asterisk(&mut self, namespace: &[String]) -> Option<()> {
        let ns_id = self.arena.resolve_declaration_id_from_root(namespace)?;
        let ns = self.arena.get_by_id(&ns_id).unwrap();
        self.values.extend(ns.children().clone());
        Some(())
    }

    /// use [namespace]::[name];
    pub(crate) fn use_(&mut self, fqn: &[String]) -> Option<()> {
        let last = fqn.last()?;
        if last == "*" {
            self.use_asterisk(&fqn[..fqn.len() - 1])?;
        } else {
            let item = self.arena.resolve_declaration_id_from_root(fqn).unwrap();
            let entry = self.values.entry(last.to_string()).or_default();
            entry.insert(item);
        };
        Some(())
    }

    pub(crate) fn get_type(
        &self,
        name_space: &[String],
        type_name: &str,
    ) -> Option<&ResolverStruct> {
        let maybe_type_parameter = match self.get_env_item(name_space, type_name) {
            Some(EnvValue::Type(id)) => Some(id),
            _ => None,
        };
        let n = match maybe_type_parameter {
            None => self.arena.get_type(&name_space, type_name),
            Some(id) => self.arena.get_type_by_id(&id),
        };
        n
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ResolverStruct> {
        self.get_type(&typ.package().into_resolved().names, &typ.name())
    }

    pub(crate) fn get_env_item(&self, namespace: &[String], name: &str) -> Option<EnvValue> {
        if namespace.is_empty() {
            let maybe_local_value = self.local_stack.get(name).cloned();
            match maybe_local_value {
                None => {
                    let ids = self.values.get(name)?;
                    let ids = ids.iter().collect::<Vec<_>>();
                    let items = self.arena.get_by_ids(&ids)?;
                    if !items.is_empty() {
                        return if let DeclarationItemKind::Type(_) = &items.first().unwrap().kind {
                            Some(EnvValue::from(**ids.first().unwrap()))
                        } else {
                            let mut values = HashSet::new();
                            for (item, id) in items.iter().zip(ids) {
                                if let DeclarationItemKind::Function(rf) = &item.kind {
                                    values.insert((*id, rf.ty.clone()));
                                } else if let DeclarationItemKind::Variable(v) = &item.kind {
                                    values.insert((*id, v.clone()));
                                } else {
                                    None?
                                }
                            }
                            Some(EnvValue::from(values))
                        };
                    };
                    None
                }
                Some(t) => Some(t),
            }
        } else {
            let ids = self.values.get(&namespace[0])?;
            let ids = ids.iter().copied().collect::<Vec<_>>();
            let parent_id = ids.first()?;
            let id = self
                .arena
                .resolve_declaration_id(*parent_id, &namespace[1..])?;
            let item = self.arena.get_by_id(&id)?;
            let children = item.get_child(name)?;
            let children = children.iter().collect::<Vec<_>>();
            let items = self.arena.get_by_ids(&children)?;
            if !items.is_empty() {
                return if let DeclarationItemKind::Type(_) = &items.first().unwrap().kind {
                    Some(EnvValue::from(**children.first().unwrap()))
                } else {
                    let mut values = HashSet::new();
                    for (item, id) in items.iter().zip(children) {
                        if let DeclarationItemKind::Function(rf) = &item.kind {
                            values.insert((*id, rf.ty.clone()));
                        } else if let DeclarationItemKind::Variable(v) = &item.kind {
                            values.insert((*id, v.clone()));
                        } else {
                            None?
                        }
                    }
                    Some(EnvValue::from(values))
                };
            };
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NameEnvironment;
    use crate::high_level_ir::type_resolver::context::EnvValue;
    use crate::ResolverArena;
    use std::collections::HashMap;
    use wiz_utils::StackedHashMap;

    #[test]
    fn get_type() {
        let mut arena = ResolverArena::default();
        let env = NameEnvironment::new(&mut arena, StackedHashMap::from(HashMap::new()));
        let int32 = env.get_type(&[], "Int32");

        assert!(matches!(int32, Some(_)))
    }

    #[test]
    fn get_env_item() {
        let mut arena = ResolverArena::default();
        let env = NameEnvironment::new(&mut arena, StackedHashMap::from(HashMap::new()));
        let int32 = env.get_env_item(&[], "Int32");

        assert!(matches!(int32, Some(EnvValue::Type(_))))
    }
}
