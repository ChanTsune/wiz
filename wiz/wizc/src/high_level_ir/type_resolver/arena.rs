use crate::high_level_ir::declaration_id::{DeclarationId, DeclarationIdGenerator};
use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::declaration::{DeclarationItem, DeclarationItemKind};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use wiz_constants::annotation::BUILTIN;
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_expr::TypedBinaryOperator;
use wiz_hir::typed_type::{Package, TypedNamedValueType, TypedPackage, TypedType, TypedValueType};

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
            DeclarationItem::new(
                TypedAnnotations::from(vec![BUILTIN]),
                "",
                DeclarationItemKind::Namespace,
                None,
            ),
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
                            &DeclarationId::ROOT,
                            &v.name,
                            TypedAnnotations::from(vec![BUILTIN]),
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
    fn register(
        &mut self,
        namespace: &DeclarationId,
        name: &str,
        declaration: DeclarationItem,
    ) -> Option<DeclarationId> {
        let d = self.declarations.get_mut(namespace)?;
        if !declaration.is_value() && d.get_child(name).is_some() {
            return None;
        }
        let id = self.declaration_id_generator.next();
        d.add_child(name, id);
        self.declarations.insert(id, declaration);
        Some(id)
    }

    pub(crate) fn register_namespace(
        &mut self,
        namespace: &DeclarationId,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register(
            namespace,
            name,
            DeclarationItem::new(
                annotation,
                name,
                DeclarationItemKind::Namespace,
                Some(*namespace),
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
            let parent = self.declarations.get(&parent_id)?;
            self.resolve_declaration_id(
                *parent
                    .get_child(&item_name[0].to_string())?
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

    pub(crate) fn get_mut_by_id(&mut self, id: &DeclarationId) -> Option<&mut DeclarationItem> {
        self.declarations.get_mut(id)
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
        self.get_mut_by_id(&id)
    }

    pub(crate) fn get_type_by_id(&self, id: &DeclarationId) -> Option<&ResolverStruct> {
        match &self.get_by_id(id)?.kind {
            DeclarationItemKind::Type(rs) => Some(rs),
            DeclarationItemKind::Namespace | DeclarationItemKind::Value(_) => None,
        }
    }

    pub(crate) fn register_struct(
        &mut self,
        namespace: &DeclarationId,
        name: &str, /* type_parameters */
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::Struct)
    }

    pub(crate) fn register_type_parameter(
        &mut self,
        namespace: &DeclarationId,
        name: &str, /* type_parameters */
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::TypeParameter)
    }

    pub(crate) fn register_protocol(
        &mut self,
        namespace: &DeclarationId,
        name: &str, /* type_parameters */
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register_type(namespace, name, annotation, StructKind::Protocol)
    }

    fn register_type(
        &mut self,
        namespace: &DeclarationId,
        name: &str,
        annotation: TypedAnnotations,
        kind: StructKind, /* type_parameters */
    ) -> Option<DeclarationId> {
        let vec_namespace = self.resolve_fully_qualified_name(namespace);
        let s = ResolverStruct::new(
            TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                package: TypedPackage::Resolved(Package::from(&vec_namespace)),
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
                Some(*namespace),
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

    pub(crate) fn register_value(
        &mut self,
        namespace: &DeclarationId,
        name: &str,
        ty: TypedType,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.register(
            namespace,
            name,
            DeclarationItem::new(
                annotation,
                name,
                DeclarationItemKind::Value(ty),
                Some(*namespace),
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

impl Display for ResolverArena {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = self.get_by_id(&DeclarationId::ROOT) {
            let mut h = vec![false];
            self._fmt(f, "root", root, 0, true, &mut h)
        } else {
            f.write_str("root...")
        }
    }
}

impl ResolverArena {
    fn ident(
        f: &mut Formatter<'_>,
        level: usize,
        is_last: bool,
        hierarchy_tree: &[bool],
    ) -> std::fmt::Result {
        let mut i = 0;
        let s = hierarchy_tree.len();
        while i < s - 1 {
            f.write_str(if hierarchy_tree[i] { "│  " } else { "   " })?;
            i += 1;
        }
        if level > 0 {
            f.write_str(if is_last { "└──" } else { "├──" })?;
        }
        Ok(())
    }

    fn _fmt(
        &self,
        f: &mut Formatter<'_>,
        name: &str,
        item: &DeclarationItem,
        level: usize,
        is_last: bool,
        hierarchy_tree: &mut Vec<bool>,
    ) -> std::fmt::Result {
        Self::ident(f, level, is_last, &hierarchy_tree)?;
        f.write_str(name)?;
        if item.is_namespace() {
            f.write_char('/')?;
        } else if item.is_type() {
            f.write_char('*')?;
        }
        f.write_char('\n')?;
        let children_count = item.children().len();
        for (i, child) in item.children().iter().enumerate() {
            let last = (i + 1) == children_count;
            hierarchy_tree.push(i != (children_count - 1));
            let id = child.1.iter().find(|_| true).unwrap();
            let item = self.get_by_id(id).unwrap();
            self._fmt(f, child.0, item, level + 1, last, hierarchy_tree)?;
            hierarchy_tree.pop();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::declaration_id::DeclarationId;
    use super::super::declaration::DeclarationItemKind;
    use super::ResolverArena;
    use crate::high_level_ir::type_resolver::declaration::DeclarationItem;
    use wiz_hir::typed_type::{TypedArgType, TypedFunctionType, TypedType};

    #[test]
    fn resolve_declaration_id_from_root() {
        let arena = ResolverArena::default();

        let namespace_name: [&str; 0] = [];

        let ns_id = arena.resolve_declaration_id_from_root(&namespace_name);
        assert_eq!(DeclarationId::ROOT, ns_id.unwrap())
    }

    #[test]
    fn resolve_child_namespace() {
        let mut arena = ResolverArena::default();
        let std_namespace_name = "std";

        let std_namespace_id =
            arena.register_namespace(&DeclarationId::ROOT, std_namespace_name, Default::default());

        let ns_id = arena.resolve_declaration_id_from_root(&[std_namespace_name]);

        assert_eq!(std_namespace_id.unwrap(), ns_id.unwrap());
    }

    #[test]
    fn resolve_grandchildren_namespace() {
        let mut arena = ResolverArena::default();
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        let child_namespace_id = arena
            .register_namespace(
                &DeclarationId::ROOT,
                child_namespace_name,
                Default::default(),
            )
            .unwrap();
        let std_collections_id = arena.register_namespace(
            &child_namespace_id,
            grandchildren_namespace_name,
            Default::default(),
        );

        let ns_id = arena.resolve_declaration_id_from_root(&[
            child_namespace_name,
            grandchildren_namespace_name,
        ]);

        assert_eq!(std_collections_id, ns_id);
    }

    #[test]
    fn register() {
        let mut arena = ResolverArena::default();
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";
        let type_name = "Type";
        let member_function_name = "member_function";

        let child_namespace_id = arena
            .register_namespace(
                &DeclarationId::ROOT,
                child_namespace_name,
                Default::default(),
            )
            .unwrap();
        let grandchildren_namespace_id = arena
            .register_namespace(
                &child_namespace_id,
                grandchildren_namespace_name,
                Default::default(),
            )
            .unwrap();
        let type_id = arena
            .register_struct(&grandchildren_namespace_id, type_name, Default::default())
            .unwrap();

        let member_function_id = arena.register_value(
            &type_id,
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
            type_id,
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
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";
        let type_name = "Type";

        let child_namespace_id = arena
            .register_namespace(
                &DeclarationId::ROOT,
                child_namespace_name,
                Default::default(),
            )
            .unwrap();
        let grandchildren_namespace_id = arena
            .register_namespace(
                &child_namespace_id,
                grandchildren_namespace_name,
                Default::default(),
            )
            .unwrap();
        arena.register_struct(&grandchildren_namespace_id, type_name, Default::default());
        let none = arena.register_namespace(
            &DeclarationId::ROOT,
            child_namespace_name,
            Default::default(),
        );

        assert_eq!(none, None);
    }

    #[test]
    fn get() {
        let mut arena = ResolverArena::default();
        let root_namespace: [&str; 0] = [];
        let std_namespace_name = "std";

        arena
            .register_namespace(&DeclarationId::ROOT, std_namespace_name, Default::default())
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
        let child_namespace_name = "std";
        let grandchildren_namespace_name = "collections";

        let child_namespace_id = arena
            .register_namespace(
                &DeclarationId::ROOT,
                child_namespace_name,
                Default::default(),
            )
            .unwrap();
        arena.register_namespace(
            &child_namespace_id,
            grandchildren_namespace_name,
            Default::default(),
        );

        let ns_id = arena.resolve_declaration_id_from_root(&[
            child_namespace_name,
            grandchildren_namespace_name,
        ]);

        assert_eq!(
            arena.resolve_fully_qualified_name(&ns_id.unwrap()),
            ["std", "collections"]
        )
    }
}
