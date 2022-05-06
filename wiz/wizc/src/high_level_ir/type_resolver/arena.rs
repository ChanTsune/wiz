use crate::high_level_ir::declaration_id::{DeclarationId, DeclarationIdGenerator};
use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::declaration::DeclarationItem;
use crate::high_level_ir::type_resolver::name_space::NameSpace;
use crate::high_level_ir::type_resolver::namespace::Namespace;
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{
    Package, TypedNamedValueType, TypedPackage, TypedType, TypedValueType,
};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct ArenaError(String);

impl Display for ArenaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for ArenaError {}

#[derive(Debug, Clone)]
pub struct ResolverArena {
    declaration_id_generator: DeclarationIdGenerator,
    declarations: HashMap<DeclarationId, DeclarationItem>,
    name_space: NameSpace,
    binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
}

impl Default for ResolverArena {
    fn default() -> Self {
        let mut declarations = HashMap::new();
        declarations.insert(
            DeclarationId::ROOT,
            DeclarationItem::Namespace(Namespace::root()),
        );

        let mut arena = Self {
            declaration_id_generator: DeclarationIdGenerator::new(0),
            declarations,
            name_space: NameSpace::empty(),
            binary_operators: Default::default(),
        };

        for t in TypedType::builtin_types() {
            match &t {
                TypedType::Value(v) => match v {
                    TypedValueType::Value(v) => {
                        arena.register_struct(&t.package().into_resolved().names, &v.name);
                    }
                    TypedValueType::Array(_, _) => {}
                    TypedValueType::Tuple(_) => {}
                    TypedValueType::Pointer(_) => {}
                    TypedValueType::Reference(_) => {}
                },
                _ => {}
            };
        }
        arena
    }
}

impl ResolverArena {
    pub(crate) fn resolve_namespace<T: ToString>(
        &self,
        parent: DeclarationId,
        namespace: &[T],
    ) -> Option<DeclarationId> {
        if namespace.is_empty() {
            Some(parent)
        } else {
            let name = namespace.get(0).unwrap();
            let parent = self.declarations.get(&parent)?;
            if let DeclarationItem::Namespace(parent) = parent {
                self.resolve_namespace(
                    *parent.get_child(&name.to_string())?.into_iter().collect::<Vec<_>>().first().unwrap(),
                    &namespace[1..],
                )
            } else {
                None
            }
        }
    }

    pub(crate) fn resolve_namespace_from_root<T: ToString>(
        &self,
        namespace: &[T],
    ) -> Option<DeclarationId> {
        self.resolve_namespace(DeclarationId::ROOT, namespace)
    }

    /// register the [declaration] in the given [namespace] as the [name].
    fn register<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        declaration: DeclarationItem,
    ) -> Option<()> {
        let target_namespace_id = self.resolve_namespace_from_root(namespace)?;
        let d = self.declarations.get_mut(&target_namespace_id)?;
        let id = match d {
            DeclarationItem::Namespace(n) => {
                let is_namespace = matches!(declaration, DeclarationItem::Namespace(_));
                if is_namespace && n.get_child(name).is_some() {
                    return None;
                }
                let id = self.declaration_id_generator.next();
                n.add_child(name, id);
                id
            }
            DeclarationItem::Type(_) => panic!("this is type"),
            DeclarationItem::Value(_) => panic!("this is value"),
        };
        self.declarations.insert(id, declaration);
        Some(())
    }

    pub(crate) fn register_namespace<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
    ) -> Option<()> {
        let parent_id = self.resolve_namespace_from_root(namespace)?;
        self.register(
            namespace,
            name,
            DeclarationItem::Namespace(Namespace::new(name, parent_id)),
        )
    }

    pub(crate) fn resolve_fully_qualified_name(&self, id: &DeclarationId) -> Vec<String> {
        let decl = self.declarations.get(id).unwrap();
        match decl {
            DeclarationItem::Namespace(n) => {
                if let Some(parent_id) = n.parent() {
                    let mut parents_name = self.resolve_fully_qualified_name(&parent_id);
                    parents_name.push(n.name());
                    parents_name
                } else {
                    // NOTE: This will root namespace
                    vec![]
                }
            }
            DeclarationItem::Type(t) => {
                vec![t.name.clone()]
            }
            DeclarationItem::Value(t) => todo!(),
        }
    }

    pub(crate) fn create_namespace_all<T: ToString>(&mut self, namespace: &[T]) {
        self.name_space
            .set_child(namespace.iter().map(T::to_string).collect())
    }

    pub(crate) fn get_name_space<T: ToString>(&self, namespace: &[T]) -> Option<&NameSpace> {
        self.name_space
            .get_child(namespace.iter().map(T::to_string).collect())
    }
}

impl ResolverArena {
    fn resolve_declaration_id<T: ToString>(
        &self,
        parent_id: DeclarationId,
        item_name: &[T],
    ) -> Option<DeclarationId> {
        if item_name.is_empty() {
            Some(parent_id)
        } else {
            let name = item_name.get(0).unwrap();
            let parent = self.declarations.get(&parent_id)?;
            match parent {
                DeclarationItem::Namespace(parent) => self.resolve_declaration_id(
                    *parent.get_child(&name.to_string())?.into_iter().collect::<Vec<_>>().first().unwrap(),
                    &item_name[1..],
                ),
                DeclarationItem::Type(_) | DeclarationItem::Value(_) => {
                    if item_name.len() == 1 {
                        Some(parent_id)
                    } else {
                        None
                    }
                }
            }
        }
    }

    pub(crate) fn resolve_declaration_id_from_root<T: ToString>(
        &self,
        fqn: &[T],
    ) -> Option<DeclarationId> {
        self.resolve_declaration_id(DeclarationId::ROOT, fqn)
    }

    pub(crate) fn get<T: ToString>(&self, namespace: &[T], name: &str) -> Option<&DeclarationItem> {
        let id = self.resolve_declaration_id_from_root(
            &namespace
                .iter()
                .map(T::to_string)
                .chain([name.to_string()])
                .collect::<Vec<_>>(),
        )?;
        self.get_by_id(&id)
    }

    pub(crate) fn get_by_id(&self, id: &DeclarationId) -> Option<&DeclarationItem> {
        self.declarations.get(id)
    }

    pub(crate) fn get_by_ids(&self, ids: &[&DeclarationId]) -> Option<Vec<&DeclarationItem>> {
        let mut items = vec![];
        for id in ids {
            items.push(
                self.declarations.get(id)?
            );
        }
        Some(items)
    }

    pub(crate) fn get_mut<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
    ) -> Option<&mut DeclarationItem> {
        let id = self.resolve_declaration_id_from_root(
            &namespace
                .iter()
                .map(T::to_string)
                .chain([name.to_string()])
                .collect::<Vec<_>>(),
        )?;
        self.declarations.get_mut(&id)
    }

    pub(crate) fn register_struct<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str, /* type_parameters */
    ) {
        self.register_type(namespace, name, StructKind::Struct)
    }

    pub(crate) fn register_protocol<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str, /* type_parameters */
    ) {
        self.register_type(namespace, name, StructKind::Protocol)
    }

    fn register_type<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        kind: StructKind, /* type_parameters */
    ) {
        let s = ResolverStruct::new(
            TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                package: TypedPackage::Resolved(Package::from(
                    namespace.iter().map(T::to_string).collect::<Vec<_>>(),
                )),
                name: name.to_string(),
                type_args: None,
            })),
            kind,
        );
        self.register(namespace, name, DeclarationItem::Type(s.clone()));
        let child_ns = self
            .name_space
            .get_child_mut(namespace.iter().map(T::to_string).collect())
            .unwrap();
        child_ns.register_type(name.to_string(), s);
    }

    pub(crate) fn get_type<T: ToString>(
        &self,
        name_space: &[T],
        name: &str,
    ) -> Option<&ResolverStruct> {
        let n = self
            .name_space
            .get_child(name_space.iter().map(T::to_string).collect())?;
        n.get_type(name)
    }

    pub(crate) fn get_type_mut<T: ToString>(
        &mut self,
        name_space: &[T],
        name: &str,
    ) -> Option<&mut ResolverStruct> {
        let n = self
            .name_space
            .get_child_mut(name_space.iter().map(T::to_string).collect())?;
        n.get_type_mut(name)
    }

    pub(crate) fn register_value<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        ty: TypedType,
    ) {
        let vec_namespace = namespace.iter().map(T::to_string).collect::<Vec<_>>();
        let child_ns = self
            .name_space
            .get_child_mut(vec_namespace.clone())
            .unwrap();
        child_ns.register_value(vec_namespace.clone(), name.to_string(), ty.clone());
        self.register(namespace, name, DeclarationItem::Value((vec_namespace, ty)));
    }

    pub(crate) fn resolve_binary_operator(
        &self,
        key: &(TypedBinaryOperator, TypedType, TypedType),
    ) -> Option<&TypedType> {
        self.binary_operators.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::declaration_id::DeclarationId;
    use super::super::super::type_resolver::namespace::Namespace;
    use super::super::declaration::DeclarationItem;
    use super::ResolverArena;
    use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
    use crate::high_level_ir::typed_type::{
        Package, TypedNamedValueType, TypedPackage, TypedType, TypedValueType,
    };

    #[test]
    fn resolve_root_namespace() {
        let arena = ResolverArena::default();

        let namespace_name: [&str; 0] = [];

        let ns_id = arena.resolve_namespace_from_root(&namespace_name);
        assert_eq!(DeclarationId::ROOT, ns_id.unwrap())
    }

    #[test]
    fn resolve_child_namespace() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";

        arena.register_namespace(&root_namespace, child_namespace_name);

        let ns_id = arena.resolve_namespace_from_root(&[child_namespace_name]);
        assert_eq!(
            DeclarationItem::Namespace(Namespace::new(child_namespace_name, DeclarationId::ROOT)),
            *arena.declarations.get(&ns_id.unwrap()).unwrap()
        )
    }

    #[test]
    fn resolve_grandchildren_namespace() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        arena.register_namespace(&root_namespace, child_namespace_name);
        arena.register_namespace(&[child_namespace_name], grandchildren_namespace_name);

        let ns_id = arena
            .resolve_namespace_from_root(&[child_namespace_name, grandchildren_namespace_name]);

        let parent_id = arena.resolve_namespace_from_root(&[child_namespace_name]);
        assert_eq!(
            DeclarationItem::Namespace(Namespace::new(
                grandchildren_namespace_name,
                parent_id.unwrap()
            )),
            *arena.declarations.get(&ns_id.unwrap()).unwrap()
        )
    }

    #[test]
    fn register() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";
        let type_name = "type";

        arena.register_namespace(&root_namespace, child_namespace_name);
        arena.register_namespace(&[child_namespace_name], grandchildren_namespace_name);
        arena.register_struct(
            &[child_namespace_name, grandchildren_namespace_name],
            type_name,
        );
        arena.register_namespace(&root_namespace, child_namespace_name);

        let item = arena.get(
            &[child_namespace_name, grandchildren_namespace_name],
            type_name,
        );
        assert_eq!(
            item,
            Some(&DeclarationItem::Type(ResolverStruct::new(
                TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                    package: TypedPackage::Resolved(Package::from(vec![
                        child_namespace_name,
                        grandchildren_namespace_name
                    ])),
                    name: type_name.to_string(),
                    type_args: None,
                })),
                StructKind::Struct,
            )))
        );
    }

    #[test]
    fn resolve_fully_qualified_name() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        arena.register_namespace(&root_namespace, child_namespace_name);
        arena.register_namespace(&[child_namespace_name], grandchildren_namespace_name);

        let ns_id = arena
            .resolve_namespace_from_root(&[child_namespace_name, grandchildren_namespace_name]);

        assert_eq!(
            arena.resolve_fully_qualified_name(&ns_id.unwrap()),
            ["std", "collections"]
        )
    }
}
