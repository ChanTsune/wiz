use crate::high_level_ir::declaration_id::{DeclarationId, DeclarationIdGenerator};
use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::declaration::{DeclarationItem, DeclarationItemKind};
use crate::high_level_ir::typed_annotation::TypedAnnotations;
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
    binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
}

impl Default for ResolverArena {
    fn default() -> Self {
        let mut declarations = HashMap::new();
        declarations.insert(
            DeclarationId::ROOT,
            DeclarationItem::new(Default::default(), "", DeclarationItemKind::Namespace, None),
        );

        let mut arena = Self {
            declaration_id_generator: DeclarationIdGenerator::new(0),
            declarations,
            binary_operators: Default::default(),
        };

        for t in TypedType::builtin_types() {
            match &t {
                TypedType::Value(v) => match v {
                    TypedValueType::Value(v) => {
                        arena.register_struct(
                            &t.package().into_resolved().names,
                            &v.name,
                            Default::default(),
                        );
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
            if parent.is_namespace() {
                self.resolve_namespace(
                    *parent
                        .get_child(&name.to_string())?
                        .into_iter()
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap(),
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
    ) -> Option<DeclarationId> {
        let target_namespace_id = self.resolve_namespace_from_root(namespace)?;
        let d = self.declarations.get_mut(&target_namespace_id)?;
        if declaration.is_namespace() && d.get_child(name).is_some() {
            return None;
        }
        let id = self.declaration_id_generator.next();
        d.add_child(name, id);
        self.declarations.insert(id, declaration);
        Some(id)
    }

    pub(crate) fn register_namespace<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        let parent_id = self.resolve_namespace_from_root(namespace)?;
        self.register(
            namespace,
            name,
            DeclarationItem::new(
                annotation,
                name,
                DeclarationItemKind::Namespace,
                Some(parent_id),
            ),
        )
    }

    pub(crate) fn resolve_fully_qualified_name(&self, id: &DeclarationId) -> Vec<String> {
        let decl = self.declarations.get(id).unwrap();
        if let Some(parent_id) = decl.parent() {
            let mut parents_name = self.resolve_fully_qualified_name(&parent_id);
            parents_name.push(decl.name.clone());
            parents_name
        } else {
            // NOTE: This will root namespace
            vec![]
        }
    }
}

impl ResolverArena {
    pub(crate) fn resolve_declaration_id<T: ToString>(
        &self,
        parent_id: DeclarationId,
        item_name: &[T],
    ) -> Option<DeclarationId> {
        if item_name.is_empty() {
            Some(parent_id)
        } else {
            let name = item_name.get(0).unwrap();
            let parent = self.declarations.get(&parent_id)?;
            self.resolve_declaration_id(
                *parent
                    .get_child(&name.to_string())?
                    .into_iter()
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap(),
                &item_name[1..],
            )
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
            items.push(self.declarations.get(id)?);
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
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::Struct)
    }

    pub(crate) fn register_type_parameter<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str, /* type_parameters */
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::TypeParameter)
    }

    pub(crate) fn register_protocol<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str, /* type_parameters */
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::Protocol)
    }

    fn register_type<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        annotation: TypedAnnotations,
        kind: StructKind, /* type_parameters */
    ) -> Option<DeclarationId> {
        let parent_id = self.resolve_namespace_from_root(namespace)?;
        let s = ResolverStruct::new(
            TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                package: TypedPackage::Resolved(Package::from(namespace)),
                name: name.to_string(),
                type_args: None,
            })),
            kind,
        );
        self.register(
            namespace,
            name,
            DeclarationItem::new(
                annotation,
                name,
                DeclarationItemKind::Type(s),
                Some(parent_id),
            ),
        )
    }

    pub(crate) fn get_type<T: ToString>(
        &self,
        name_space: &[T],
        name: &str,
    ) -> Option<&ResolverStruct> {
        match &self.get(name_space, name)?.kind {
            DeclarationItemKind::Namespace => panic!("this is namespace"),
            DeclarationItemKind::Type(t) => Some(t),
            DeclarationItemKind::Value(v) => panic!("V:{:?}", v),
        }
    }

    pub(crate) fn get_type_mut<T: ToString>(
        &mut self,
        name_space: &[T],
        name: &str,
    ) -> Option<&mut ResolverStruct> {
        match &mut self.get_mut(name_space, name)?.kind {
            DeclarationItemKind::Namespace => panic!("this is namespace"),
            DeclarationItemKind::Type(t) => Some(t),
            DeclarationItemKind::Value(v) => panic!("V:{:?}", v),
        }
    }

    pub(crate) fn register_value<T: ToString>(
        &mut self,
        namespace: &[T],
        name: &str,
        ty: TypedType,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        let vec_namespace = namespace.iter().map(T::to_string).collect::<Vec<_>>();
        let parent_id = self.resolve_namespace_from_root(namespace)?;
        self.register(
            namespace,
            name,
            DeclarationItem::new(
                annotation,
                name,
                DeclarationItemKind::Value((vec_namespace, ty)),
                Some(parent_id),
            ),
        )
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
    use super::super::declaration::DeclarationItemKind;
    use super::ResolverArena;
    use crate::high_level_ir::type_resolver::declaration::DeclarationItem;
    use crate::high_level_ir::typed_type::{TypedArgType, TypedFunctionType, TypedType};

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
        let std_namespace_name = "std";

        let std_namespace_id =
            arena.register_namespace(&root_namespace, std_namespace_name, Default::default());

        let ns_id = arena.resolve_namespace_from_root(&[std_namespace_name]);

        assert_eq!(std_namespace_id.unwrap(), ns_id.unwrap());
    }

    #[test]
    fn resolve_grandchildren_namespace() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        arena
            .register_namespace(&root_namespace, child_namespace_name, Default::default())
            .unwrap();
        let std_collections_id = arena.register_namespace(
            &[child_namespace_name],
            grandchildren_namespace_name,
            Default::default(),
        );

        let ns_id = arena
            .resolve_namespace_from_root(&[child_namespace_name, grandchildren_namespace_name]);

        assert_eq!(std_collections_id, ns_id);
    }

    #[test]
    fn register() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";
        let type_name = "Type";
        let member_function_name = "member_function";

        arena.register_namespace(&root_namespace, child_namespace_name, Default::default());
        arena.register_namespace(
            &[child_namespace_name],
            grandchildren_namespace_name,
            Default::default(),
        );
        let type_id = arena.register_struct(
            &[child_namespace_name, grandchildren_namespace_name],
            type_name,
            Default::default(),
        );

        let member_function_id = arena.register_value(
            &[
                child_namespace_name,
                grandchildren_namespace_name,
                type_name,
            ],
            member_function_name,
            TypedType::Function(Box::new(TypedFunctionType {
                arguments: vec![TypedArgType {
                    label: "self".to_string(),
                    typ: TypedType::Self_,
                }],
                return_type: TypedType::Self_,
            })),
            Default::default(),
        );

        assert_eq!(
            type_id.unwrap(),
            arena
                .resolve_declaration_id_from_root(&[
                    child_namespace_name,
                    grandchildren_namespace_name,
                    type_name
                ])
                .unwrap()
        );

        assert_eq!(
            member_function_id.unwrap(),
            arena
                .resolve_declaration_id_from_root(&[
                    child_namespace_name,
                    grandchildren_namespace_name,
                    type_name,
                    member_function_name
                ])
                .unwrap()
        );
    }

    #[test]
    fn register_duplicate_namespace() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";
        let type_name = "Type";

        arena.register_namespace(&root_namespace, child_namespace_name, Default::default());
        arena.register_namespace(
            &[child_namespace_name],
            grandchildren_namespace_name,
            Default::default(),
        );
        arena.register_struct(
            &[child_namespace_name, grandchildren_namespace_name],
            type_name,
            Default::default(),
        );
        let none =
            arena.register_namespace(&root_namespace, child_namespace_name, Default::default());

        assert_eq!(none, None);
    }

    #[test]
    fn get() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let std_namespace_name = "std";

        arena
            .register_namespace(&root_namespace, std_namespace_name, Default::default())
            .unwrap();

        let std_namespace = arena.get(&root_namespace, std_namespace_name);

        assert_eq!(
            DeclarationItem::new(
                Default::default(),
                std_namespace_name,
                DeclarationItemKind::Namespace,
                Some(DeclarationId::ROOT),
            ),
            *std_namespace.unwrap()
        )
    }

    #[test]
    fn resolve_fully_qualified_name() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        arena.register_namespace(&root_namespace, child_namespace_name, Default::default());
        arena.register_namespace(
            &[child_namespace_name],
            grandchildren_namespace_name,
            Default::default(),
        );

        let ns_id = arena
            .resolve_namespace_from_root(&[child_namespace_name, grandchildren_namespace_name]);

        assert_eq!(
            arena.resolve_fully_qualified_name(&ns_id.unwrap()),
            ["std", "collections"]
        )
    }
}
