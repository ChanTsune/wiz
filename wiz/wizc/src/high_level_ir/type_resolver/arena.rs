use crate::high_level_ir::type_resolver::context::{ResolverStruct, StructKind};
use crate::high_level_ir::type_resolver::namespace::NameSpace;
use crate::high_level_ir::typed_expr::TypedBinaryOperator;
use crate::high_level_ir::typed_type::{TypedType, TypedValueType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResolverArena {
    pub name_space: NameSpace,
    pub binary_operators: HashMap<(TypedBinaryOperator, TypedType, TypedType), TypedType>,
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
                            ResolverStruct::new(t.clone(), StructKind::Struct)
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
