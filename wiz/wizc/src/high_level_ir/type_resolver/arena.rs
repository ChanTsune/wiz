use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::namespace::NameSpace;
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{TypedType, TypedValueType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResolverArena {
    name_space: NameSpace,
    binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
}

impl Default for ResolverArena {
    fn default() -> Self {
        let mut ns = NameSpace::empty();

        for t in TypedType::builtin_types() {
            match &t {
                TypedType::Value(v) => match v {
                    TypedValueType::Value(v) => {
                        ns.register_type(
                            v.name.clone(),
                            ResolverStruct::new(t.clone(), StructKind::Struct),
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
        Self {
            name_space: ns,
            binary_operators: Default::default(),
        }
    }
}

impl ResolverArena {
    pub(crate) fn register_namespace<T:ToString>(&mut self, namespace: &[T]) {
        self.name_space.set_child(namespace.iter().map(T::to_string).collect())
    }

    pub(crate) fn get_namespace<T: ToString>(&self, namespace: &[T]) -> Option<&NameSpace> {
        self.name_space.get_child(namespace.iter().map(T::to_string).collect())
    }

    pub(crate) fn get_namespace_mut<T: ToString>(&mut self, namespace: &[T]) -> Option<&mut NameSpace> {
        self.name_space.get_child_mut(namespace.iter().map(T::to_string).collect())
    }
}

impl ResolverArena {
    pub(crate) fn get_struct_by<T: ToString>(
        &self,
        name_space: Vec<T>,
        name: &str,
    ) -> Option<&ResolverStruct> {
        let n = self.name_space.get_child(name_space)?;
        n.get_type(name)
    }

    pub(crate) fn resolve_binary_operator(
        &self,
        key: &(TypedBinaryOperator, TypedType, TypedType),
    ) -> Option<&TypedType> {
        self.binary_operators.get(key)
    }
}
