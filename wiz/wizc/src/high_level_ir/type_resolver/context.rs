mod env_value;

pub(crate) use crate::high_level_ir::type_resolver::context::env_value::EnvValue;
use crate::high_level_ir::type_resolver::name_environment::NameEnvironment;
use std::collections::HashMap;
use wiz_arena::{Arena, ArenaStruct, DeclarationId, DeclarationItemKind};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_decl::TypedFunBody;
use wiz_hir::typed_expr::TypedBinaryOperator;
use wiz_hir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedTypeParam, TypedValueType,
};
use wiz_infer::error::InferError;
use wiz_infer::result::Result;
use wiz_utils::StackedHashMap;

#[derive(Debug)]
pub struct ResolverContext<'a> {
    used_name_space: Vec<Vec<String>>,
    arena: &'a mut Arena,
    current_namespace_id: DeclarationId,
    local_stack: StackedHashMap<String, EnvValue>,
}

impl<'a> ResolverContext<'a> {
    pub(crate) fn new(arena: &'a mut Arena) -> Self {
        Self {
            used_name_space: Default::default(),
            current_namespace_id: DeclarationId::ROOT,
            local_stack: StackedHashMap::new(),
            arena,
        }
    }

    pub(crate) fn arena_mut(&mut self) -> &mut Arena {
        self.arena
    }

    pub(crate) fn arena(&self) -> &Arena {
        self.arena
    }

    pub(crate) fn current_namespace(&self) -> Vec<String> {
        self.arena()
            .resolve_fully_qualified_name(&self.current_namespace_id)
    }

    pub(crate) fn set_current_namespace_id_force(&mut self, id: DeclarationId) {
        self.current_namespace_id = id;
    }

    pub(crate) fn get_current_namespace_id(&self) -> DeclarationId {
        self.current_namespace_id
    }

    pub fn push_name_space(&mut self, name: &str) {
        let c = self.arena().get_by_id(&self.current_namespace_id).unwrap();
        let ids = c.get_child(name).unwrap();
        let ids = ids.iter().copied().collect::<Vec<_>>();
        let id = ids.first().unwrap();
        self.current_namespace_id = *id;
    }

    pub fn pop_name_space(&mut self) {
        let ns = self.arena().get_by_id(&self.current_namespace_id).unwrap();
        self.current_namespace_id = ns.parent().unwrap_or(DeclarationId::ROOT);
    }

    pub(crate) fn current_type_id(&self) -> Option<DeclarationId> {
        self._current_type_id(self.current_namespace_id)
    }

    fn _current_type_id(&self, id: DeclarationId) -> Option<DeclarationId> {
        let item = self.arena().get_by_id(&id)?;
        match &item.kind {
            DeclarationItemKind::Type(_) => Some(id),
            DeclarationItemKind::Namespace
            | DeclarationItemKind::Variable(_)
            | DeclarationItemKind::Function(..) => self._current_type_id(item.parent()?),
        }
    }

    pub(crate) fn current_type_mut(&mut self) -> Option<&mut ArenaStruct> {
        let id = self.current_namespace_id;
        match &mut self.arena_mut().get_mut_by_id(&id)?.kind {
            DeclarationItemKind::Type(rs) => Some(rs),
            DeclarationItemKind::Namespace
            | DeclarationItemKind::Variable(_)
            | DeclarationItemKind::Function(..) => None,
        }
    }

    pub(crate) fn current_module_id(&self) -> Option<DeclarationId> {
        self._current_module_id(self.current_namespace_id)
    }

    fn _current_module_id(&self, id: DeclarationId) -> Option<DeclarationId> {
        let item = self.arena().get_by_id(&id)?;
        match &item.kind {
            DeclarationItemKind::Namespace => Some(id),
            DeclarationItemKind::Type(_) | DeclarationItemKind::Function(..) => {
                self._current_module_id(item.parent().unwrap())
            }
            DeclarationItemKind::Variable(_) => None,
        }
    }

    pub fn push_local_stack(&mut self) {
        self.local_stack.push(HashMap::new());
    }

    pub fn pop_local_stack(&mut self) {
        self.local_stack.pop();
    }

    pub(crate) fn register_to_env<T>(&mut self, name: String, value: T)
    where
        EnvValue: From<T>,
    {
        let value = EnvValue::from(value);
        if self.local_stack.stack_is_empty() {
            panic!("illegal function call {}:{:?}", name, value);
        } else {
            self.local_stack.insert(name, value);
        }
    }

    pub(crate) fn get_current_name_environment(&self) -> NameEnvironment {
        let mut env = NameEnvironment::new(
            self.arena(),
            self.local_stack.clone(),
            self.current_type_id(),
        );
        env.use_asterisk(&[]);

        let module_id = self.current_module_id().unwrap();

        if self.current_namespace_id != module_id {
            let module_name = self.arena().resolve_fully_qualified_name(&module_id);
            env.use_asterisk(&module_name);
        }

        let namespace_name = self
            .arena()
            .resolve_fully_qualified_name(&self.current_namespace_id);
        env.use_asterisk(&namespace_name);

        for u in self.used_name_space.iter() {
            env.use_(u);
        }
        env
    }

    pub(crate) fn use_name_space(&mut self, n: Vec<String>) {
        self.used_name_space.push(n);
    }

    pub(crate) fn unuse_name_space(&mut self, n: &Vec<String>) {
        let i = self.used_name_space.iter().rposition(|i| i.eq(n));
        if let Some(i) = i {
            self.used_name_space.remove(i);
        };
    }

    pub fn resolve_binop_type(
        &self,
        left: TypedType,
        kind: TypedBinaryOperator,
        right: TypedType,
    ) -> Result<TypedType> {
        match kind {
            TypedBinaryOperator::Equal
            | TypedBinaryOperator::GrateThanEqual
            | TypedBinaryOperator::GrateThan
            | TypedBinaryOperator::LessThanEqual
            | TypedBinaryOperator::LessThan
            | TypedBinaryOperator::NotEqual => Ok(TypedType::bool()),
            TypedBinaryOperator::InfixFunctionCall(op) => {
                todo!("InfixFunctionCall => {}", op)
            }
            kind => {
                let is_both_integer = left.is_integer() && right.is_integer();
                let is_both_float = left.is_floating_point() && right.is_floating_point();
                let is_both_same = left == right;
                let is_pointer_op = left.is_pointer_type() && right.is_integer();
                if (is_both_same && (is_both_integer || is_both_float)) || is_pointer_op {
                    Ok(left)
                } else {
                    let key = (kind, left, right);
                    Err(InferError::from(format!("{:?} is not defined.", key)))
                }
            }
        }
    }

    fn full_value_type_name(&self, type_: &TypedValueType) -> Result<TypedValueType> {
        Ok(match type_ {
            TypedValueType::Value(t) => TypedValueType::Value(self.full_named_value_type_name(t)?),
            TypedValueType::Array(a, n) => {
                TypedValueType::Array(Box::new(self.full_type_name(a)?), *n)
            }
            TypedValueType::Tuple(_) => {
                todo!()
            }
            TypedValueType::Pointer(t) => {
                TypedValueType::Pointer(Box::new(self.full_type_name(t)?))
            }
            TypedValueType::Reference(t) => {
                TypedValueType::Reference(Box::new(self.full_type_name(t)?))
            }
        })
    }

    fn full_named_value_type_name(
        &self,
        type_: &TypedNamedValueType,
    ) -> Result<TypedNamedValueType> {
        let env = self.get_current_name_environment();
        Ok(match type_.package {
            TypedPackage::Raw(ref p) => {
                let env_value = env.get_env_item(&p.names, &type_.name).ok_or_else(|| {
                    InferError::from(format!(
                        "Cannot resolve name => {:?}{}",
                        &p.names, &type_.name
                    ))
                })?;
                match env_value {
                    EnvValue::Type(id) => {
                        let rs = self.arena().get_type_by_id(&id).unwrap();
                        TypedNamedValueType {
                            package: TypedPackage::Resolved(Package::from(&rs.namespace)),
                            name: type_.name.clone(),
                            type_args: match &type_.type_args {
                                None => None,
                                Some(v) => Some(
                                    v.iter()
                                        .map(|i| self.full_type_name(i))
                                        .collect::<Result<Vec<_>>>()?,
                                ),
                            },
                        }
                    }
                    _ => panic!(),
                }
            }
            TypedPackage::Resolved(_) => type_.clone(),
        })
    }

    pub fn full_type_name(&self, typ: &TypedType) -> Result<TypedType> {
        let env = self.get_current_name_environment();
        Ok(match typ {
            TypedType::Value(v) => TypedType::Value(self.full_value_type_name(v)?),
            TypedType::Type(v) => TypedType::Type(Box::new(self.full_type_name(v)?)),
            TypedType::Self_ => env.resolve_current_type()?,
            TypedType::Function(f) => TypedType::Function(Box::new(TypedFunctionType {
                arguments: f
                    .arguments
                    .iter()
                    .map(|a| {
                        Ok(TypedArgType {
                            label: a.label.clone(),
                            typ: self.full_type_name(&a.typ)?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
                return_type: self.full_type_name(&f.return_type)?,
            })),
        })
    }

    pub(crate) fn register_type_parameter(
        &mut self,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        let id = self.current_namespace_id;
        self.arena_mut()
            .register_type_parameter(&id, name, annotation)
    }

    pub(crate) fn register_function(
        &mut self,
        name: &str,
        ty: TypedType,
        type_parameters: Option<Vec<TypedTypeParam>>,
        body: Option<TypedFunBody>,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        let id = self.current_namespace_id;
        self.arena_mut()
            .register_function(&id, name, ty, type_parameters, body, annotation)
    }
    pub(crate) fn register_value(
        &mut self,
        name: &str,
        ty: TypedType,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        let id = self.current_namespace_id;
        self.arena_mut().register_value(&id, name, ty, annotation)
    }

    pub(crate) fn update_function(&mut self, id: &DeclarationId, ty: TypedType) -> Option<()> {
        let item = self.arena_mut().get_mut_by_id(id)?;
        if let DeclarationItemKind::Function(rf) = &item.kind {
            let mut rf = rf.clone();
            rf.ty = ty;
            item.kind = DeclarationItemKind::Function(rf);
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResolverContext;
    use wiz_arena::{Arena, ArenaStruct, StructKind};
    use wiz_constants::INT32;

    #[test]
    fn test_context_name_environment() {
        let mut arena = Arena::default();
        let mut context = ResolverContext::new(&mut arena);

        let env = context.get_current_name_environment();

        assert_eq!(
            env.get_type(&[], INT32),
            Some(&ArenaStruct::new(INT32, &[], StructKind::Struct)),
        );
    }
}
