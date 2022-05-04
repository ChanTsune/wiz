use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct};
use crate::high_level_ir::type_resolver::name_space::NameSpace;
use crate::high_level_ir::typed_type::TypedType;
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NameEnvironment<'a> {
    pub names: HashMap<String, (Vec<String>, EnvValue)>,
    arena: &'a ResolverArena,
}

impl<'a> NameEnvironment<'a> {
    pub fn new(arena: &'a ResolverArena) -> Self {
        Self {
            names: Default::default(),
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
        if name_space.is_empty() {
            match self.names.get(type_name) {
                Some((_, EnvValue::Type(rs))) => Some(rs),
                _ => None,
            }
        } else {
            let n = name_space.remove(0);
            match (name_space.is_empty(), self.names.get(&n)) {
                (false, Some((_, EnvValue::NameSpace(ns)))) => match ns.get(name_space) {
                    Some(EnvValue::NameSpace(ns)) => ns.get_type(type_name),
                    _ => None,
                },
                (true, Some((_, EnvValue::NameSpace(ns)))) => ns.get_type(type_name),
                (_, _) => None,
            }
        }
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ResolverStruct> {
        self.get_type(typ.package().into_resolved().names, &typ.name())
    }
}
