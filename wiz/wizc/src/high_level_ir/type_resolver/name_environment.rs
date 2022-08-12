use crate::high_level_ir::type_resolver::context::EnvValue;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use std::collections::{HashMap, HashSet};
use wiz_arena::arena::{Arena, ArenaStruct};
use wiz_arena::declaration::DeclarationItemKind;
use wiz_arena::declaration_id::DeclarationId;
use wiz_hir::typed_type::{Package, TypedPackage, TypedType, TypedValueType};
use wiz_utils::StackedHashMap;

#[derive(Debug, Clone)]
pub(crate) struct NameEnvironment<'a> {
    local_stack: StackedHashMap<String, EnvValue>,
    values: HashMap<String, HashSet<DeclarationId>>,
    arena: &'a Arena,
}

impl<'a> NameEnvironment<'a> {
    pub fn new(
        arena: &'a Arena,
        local_stack: StackedHashMap<String, EnvValue>,
        self_id: Option<DeclarationId>,
    ) -> Self {
        fn init_local_stack(
            arena: &Arena,
            self_id: Option<DeclarationId>,
            mut map: StackedHashMap<String, EnvValue>,
        ) -> StackedHashMap<String, EnvValue> {
            let mut m = HashMap::new();
            if let Some(self_id) = self_id {
                m.insert("Self".to_string(), EnvValue::Type(self_id));
            }
            let root = arena.get_by_id(&DeclarationId::ROOT).unwrap();
            let children = root.children().clone();

            for (child_name, ids) in children {
                let ids = ids.iter().collect::<Vec<_>>();
                let items = arena.get_by_ids(&ids).unwrap();
                let env_item = if let Some(i) = items.first() {
                    if let DeclarationItemKind::Type(_) = i.kind {
                        EnvValue::from(**ids.first().unwrap())
                    } else if let DeclarationItemKind::Namespace = i.kind {
                        EnvValue::Namespace(**ids.first().unwrap())
                    } else {
                        let mut values = HashSet::new();
                        for (item, id) in items.iter().zip(ids) {
                            if let DeclarationItemKind::Function(rf) = &item.kind {
                                values.insert((*id, rf.ty.clone()));
                            } else if let DeclarationItemKind::Variable(v) = &item.kind {
                                values.insert((*id, v.clone()));
                            } else {
                                unreachable!()
                            }
                        }
                        EnvValue::from(values)
                    }
                } else {
                    unreachable!("{}", child_name)
                };

                m.insert(child_name, env_item);
            }
            map.push(m);
            map
        }
        Self {
            local_stack: init_local_stack(arena, self_id, local_stack),
            values: arena.get_root().children().clone(),
            arena,
        }
    }
}

impl<'a> NameEnvironment<'a> {
    pub fn push(&mut self) {
        self.local_stack.push(Default::default());
    }

    pub fn pop(&mut self) {
        self.local_stack.pop();
    }

    pub fn extend(&mut self, name: String, item: EnvValue) {
        self.local_stack.insert(name, item);
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

    pub(crate) fn get_type(&self, name_space: &[String], type_name: &str) -> Option<&ArenaStruct> {
        self.arena
            .get_type_by_id(&self.get_type_id(name_space, type_name)?)
    }

    pub(crate) fn get_type_id(
        &self,
        name_space: &[String],
        type_name: &str,
    ) -> Option<DeclarationId> {
        match self.get_env_item(name_space, type_name) {
            Some(EnvValue::Type(id)) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn get_type_by_typed_type(&self, typ: TypedType) -> Option<&ArenaStruct> {
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

impl<'a> NameEnvironment<'a> {
    pub fn resolve_member_type(&self, t: TypedType, name: &str) -> Result<TypedType> {
        match t {
            TypedType::Value(v) => match v {
                TypedValueType::Value(v) => {
                    let rs = self
                        .get_type(&v.package.clone().into_resolved().names, &v.name)
                        .ok_or_else(|| {
                            ResolverError::from(format!("Can not resolve type {:?}", v))
                        })?;
                    rs.get_instance_member_type(name).cloned().ok_or_else(|| {
                        ResolverError::from(format!("{:?} not has member named `{}`", v, name))
                    })
                }
                TypedValueType::Array(_, _) => todo!(),
                TypedValueType::Tuple(_) => todo!(),
                TypedValueType::Pointer(_) => todo!(),
                TypedValueType::Reference(rt) => self.resolve_member_type(*rt, name),
            },
            TypedType::Type(v) => Err(ResolverError::from(format!(
                "{:?} has no member {}",
                v, name
            ))),
            n => todo!("{} :=> {:?}", name, n),
        }
    }

    pub fn resolve_current_type(&self) -> Result<TypedType> {
        self.get_type(&[], "Self")
            .map(|i| i.self_type())
            .ok_or_else(|| ResolverError::from("can not resolve Self"))
    }

    pub fn infer_name_type(
        &self,
        name_space: Vec<String>,
        name: &str,
        type_annotation: Option<TypedType>,
    ) -> Result<(TypedType, TypedPackage)> {
        let env_value = self.get_env_item(&name_space, name).ok_or_else(|| {
            ResolverError::from(format!("Cannot resolve name =>{:?} {:?}", name_space, name))
        })?;
        match env_value {
            EnvValue::Value(t_set) => self
                .resolve_overload(name, t_set, type_annotation)
                .map(|(id, t)| {
                    (
                        t,
                        TypedPackage::Resolved({
                            if id != DeclarationId::DUMMY {
                                let mut fqn = self.arena.resolve_fully_qualified_name(&id);
                                // item fqn to parent fqn
                                fqn.pop();
                                Package::from(&fqn)
                            } else {
                                Package::new()
                            }
                        }),
                    )
                })
                .ok_or_else(|| {
                    ResolverError::from(format!(
                        "Dose not match any overloaded function `{}`",
                        name
                    ))
                }),
            EnvValue::Type(id) => {
                let rs = self.arena.get_type_by_id(&id).unwrap();
                let self_type = rs.self_type();
                let package = self_type.package();
                Ok((TypedType::Type(Box::new(self_type)), package))
            }
            EnvValue::Namespace(id) => todo!(),
        }
    }

    fn resolve_overload(
        &self,
        name: &str,
        type_set: HashSet<(DeclarationId, TypedType)>,
        type_annotation: Option<TypedType>,
    ) -> Option<(DeclarationId, TypedType)> {
        let len = type_set.len();
        for (id, ty) in type_set {
            if len == 1 {
                return Some((id, ty));
            } else if let Some(TypedType::Function(annotation)) = &type_annotation {
                if let TypedType::Function(typ) = &ty {
                    if annotation.arguments == typ.arguments {
                        return Some((id, ty));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::NameEnvironment;
    use crate::high_level_ir::type_resolver::context::EnvValue;
    use crate::Arena;
    use std::collections::HashMap;
    use wiz_constants::INT32;
    use wiz_utils::StackedHashMap;

    #[test]
    fn get_type() {
        let mut arena = Arena::default();
        let env = NameEnvironment::new(&mut arena, StackedHashMap::from(HashMap::new()), None);
        let int32 = env.get_type(&[], INT32);

        assert!(matches!(int32, Some(_)))
    }

    #[test]
    fn get_env_item() {
        let mut arena = Arena::default();
        let env = NameEnvironment::new(&mut arena, StackedHashMap::from(HashMap::new()), None);
        let int32 = env.get_env_item(&[], "Int32");

        assert!(matches!(int32, Some(EnvValue::Type(_))))
    }
}
