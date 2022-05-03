use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::name_space::NameSpace;
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{
    Package, TypedNamedValueType, TypedPackage, TypedType, TypedValueType,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResolverArena {
    name_space: NameSpace,
    binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
}

impl Default for ResolverArena {
    fn default() -> Self {
        let mut arena = Self {
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
    pub(crate) fn register_namespace<T: ToString>(&mut self, namespace: &[T]) {
        self.name_space
            .set_child(namespace.iter().map(T::to_string).collect())
    }

    pub(crate) fn get_namespace<T: ToString>(&self, namespace: &[T]) -> Option<&NameSpace> {
        self.name_space
            .get_child(namespace.iter().map(T::to_string).collect())
    }

    pub(crate) fn get_namespace_mut<T: ToString>(
        &mut self,
        namespace: &[T],
    ) -> Option<&mut NameSpace> {
        self.name_space
            .get_child_mut(namespace.iter().map(T::to_string).collect())
    }
}

impl ResolverArena {
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
        let child_ns = self
            .name_space
            .get_child_mut(namespace.iter().map(T::to_string).collect())
            .unwrap();
        child_ns.register_value(name.to_string(), ty)
    }

    pub(crate) fn resolve_binary_operator(
        &self,
        key: &(TypedBinaryOperator, TypedType, TypedType),
    ) -> Option<&TypedType> {
        self.binary_operators.get(key)
    }
}
