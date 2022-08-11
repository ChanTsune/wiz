mod env_value;
mod resolver_function;
mod resolver_struct;

use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
pub(crate) use crate::high_level_ir::type_resolver::context::env_value::EnvValue;
pub(crate) use crate::high_level_ir::type_resolver::context::resolver_function::ResolverFunction;
pub(crate) use crate::high_level_ir::type_resolver::context::resolver_struct::{
    ResolverStruct, StructKind,
};
use crate::high_level_ir::type_resolver::declaration::DeclarationItemKind;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::name_environment::NameEnvironment;
use crate::high_level_ir::type_resolver::result::Result;
use std::collections::{HashMap, HashSet};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_decl::TypedFunBody;
use wiz_hir::typed_expr::TypedBinaryOperator;
use wiz_hir::typed_type::{
    Package, TypedArgType, TypedFunctionType, TypedNamedValueType, TypedPackage, TypedType,
    TypedTypeParam, TypedValueType,
};
use wiz_utils::StackedHashMap;

#[derive(Debug)]
pub struct ResolverContext<'a> {
    global_used_name_space: Vec<Vec<String>>,
    used_name_space: Vec<Vec<String>>,
    pub(crate) arena: &'a mut ResolverArena,
    current_type: Option<TypedType>,
    current_namespace_id: DeclarationId,
    local_stack: StackedHashMap<String, EnvValue>,
}

impl<'a> ResolverContext<'a> {
    pub(crate) fn new(arena: &'a mut ResolverArena) -> Self {
        Self {
            global_used_name_space: Default::default(),
            used_name_space: Default::default(),
            arena,
            current_type: None,
            current_namespace_id: DeclarationId::ROOT,
            local_stack: StackedHashMap::new(),
        }
    }

    pub(crate) fn arena(&self) -> &ResolverArena {
        self.arena
    }

    pub(crate) fn current_namespace(&self) -> Vec<String> {
        self.arena
            .resolve_fully_qualified_name(&self.current_namespace_id)
    }

    pub fn push_name_space(&mut self, name: &str) {
        let c = self.arena.get_by_id(&self.current_namespace_id).unwrap();
        let ids = c.get_child(name).unwrap();
        let ids = ids.iter().copied().collect::<Vec<_>>();
        let id = ids.first().unwrap();
        self.current_namespace_id = *id;
    }

    pub fn pop_name_space(&mut self) {
        let ns = self.arena.get_by_id(&self.current_namespace_id).unwrap();
        self.current_namespace_id = ns.parent().unwrap_or(DeclarationId::ROOT);
    }

    pub(crate) fn current_type(&self) -> Option<&ResolverStruct> {
        match &self.arena.get_by_id(&self.current_namespace_id)?.kind {
            DeclarationItemKind::Type(rs) => Some(rs),
            DeclarationItemKind::Namespace
            | DeclarationItemKind::Variable(_)
            | DeclarationItemKind::Function(..) => None,
        }
    }

    pub(crate) fn current_type_mut(&mut self) -> Option<&mut ResolverStruct> {
        match &mut self.arena.get_mut_by_id(&self.current_namespace_id)?.kind {
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
        let item = self.arena.get_by_id(&id)?;
        match &item.kind {
            DeclarationItemKind::Namespace => Some(id),
            DeclarationItemKind::Type(_) | DeclarationItemKind::Function(..) => {
                self._current_module_id(item.parent().unwrap())
            }
            DeclarationItemKind::Variable(_) => None,
        }
    }

    pub fn resolve_current_type(&self) -> Result<TypedType> {
        self.current_type
            .clone()
            .ok_or_else(|| ResolverError::from("can not resolve Self"))
    }

    pub fn set_current_type(&mut self, t: TypedType) {
        self.current_type = Some(t)
    }

    pub fn clear_current_type(&mut self) {
        self.current_type = None
    }

    pub fn push_local_stack(&mut self) {
        self.local_stack.push(HashMap::new());
    }

    pub fn pop_local_stack(&mut self) {
        self.local_stack.pop();
    }

    pub fn clear_local_stack(&mut self) {
        self.local_stack = StackedHashMap::new()
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
        let mut env = NameEnvironment::new(self.arena, self.local_stack.clone());
        env.use_asterisk(&[]);

        let module_id = self.current_module_id().unwrap();

        if self.current_namespace_id != module_id {
            let module_name = self.arena.resolve_fully_qualified_name(&module_id);
            env.use_asterisk(&module_name);
        }

        let namespace_name = self
            .arena
            .resolve_fully_qualified_name(&self.current_namespace_id);
        env.use_asterisk(&namespace_name);

        let used_ns = self
            .global_used_name_space
            .iter()
            .map(|ns| (true, ns))
            .chain(self.used_name_space.iter().map(|ns| (false, ns)));
        for (is_global, u) in used_ns {
            env.use_(u);
        }
        env
    }

    pub(crate) fn global_use_name_space(&mut self, ns: Vec<String>) {
        self.global_used_name_space.push(ns);
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

    pub fn resolve_member_type(&mut self, t: TypedType, name: &str) -> Result<TypedType> {
        match t {
            TypedType::Value(v) => match v {
                TypedValueType::Value(v) => {
                    let ne = self.get_current_name_environment();
                    let rs = ne
                        .get_type(v.package.clone().into_resolved().names, &v.name)
                        .ok_or_else(|| {
                            ResolverError::from(format!("Can not resolve type {:?}", v))
                        })?;
                    rs.get_instance_member_type(name).cloned().ok_or_else(|| {
                        ResolverError::from(format!("{:?} not has member named `{}`", v, name))
                    })
                }
                TypedValueType::Array(_, _) => {
                    todo!()
                }
                TypedValueType::Tuple(_) => {
                    todo!()
                }
                TypedValueType::Pointer(_) => {
                    todo!()
                }
                TypedValueType::Reference(rt) => self.resolve_member_type(*rt, name),
            },
            TypedType::Type(v) => Err(ResolverError::from(format!(
                "{:?} has no member {}",
                v, name
            ))),
            _ => todo!("dose not impl"),
        }
    }

    pub fn infer_name_type(
        &mut self,
        name_space: Vec<String>,
        name: &str,
        type_annotation: Option<TypedType>,
    ) -> Result<(TypedType, TypedPackage)> {
        if name_space.is_empty() && name == "Self" {
            let self_type = self.resolve_current_type()?;
            let package = self_type.package();
            return Ok((TypedType::Type(Box::new(self_type)), package));
        }
        let env = self.get_current_name_environment();
        let env_value = env.get_env_item(&name_space, name).ok_or_else(|| {
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
        }
    }

    fn resolve_overload(
        &mut self,
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
                    self.arena
                        .resolve_binary_operator(&key)
                        .cloned()
                        .ok_or_else(|| ResolverError::from(format!("{:?} is not defined.", key)))
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
                    ResolverError::from(format!(
                        "Cannot resolve name => {:?}{}",
                        &p.names, &type_.name
                    ))
                })?;
                match env_value {
                    EnvValue::Type(id) => {
                        let rs = self.arena.get_type_by_id(&id).unwrap();
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
        if typ.is_self() {
            self.resolve_current_type()
        } else {
            Ok(match typ {
                TypedType::Value(v) => TypedType::Value(self.full_value_type_name(v)?),
                TypedType::Type(v) => TypedType::Type(Box::new(self.full_type_name(v)?)),
                TypedType::Self_ => self.resolve_current_type()?,
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
    }

    pub(crate) fn register_struct(
        &mut self,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena
            .register_struct(&self.current_namespace_id, name, annotation)
    }

    pub(crate) fn register_protocol(
        &mut self,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena
            .register_protocol(&self.current_namespace_id, name, annotation)
    }

    pub(crate) fn register_type_parameter(
        &mut self,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena
            .register_type_parameter(&self.current_namespace_id, name, annotation)
    }

    pub(crate) fn register_function(
        &mut self,
        name: &str,
        ty: TypedType,
        type_parameters: Option<Vec<TypedTypeParam>>,
        body: Option<TypedFunBody>,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena.register_function(
            &self.current_namespace_id,
            name,
            ty,
            type_parameters,
            body,
            annotation,
        )
    }
    pub(crate) fn register_value(
        &mut self,
        name: &str,
        ty: TypedType,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena
            .register_value(&self.current_namespace_id, name, ty, annotation)
    }

    pub(crate) fn update_function(&mut self, id: &DeclarationId, ty: TypedType) -> Option<()> {
        let item = self.arena.get_mut_by_id(id)?;
        if let DeclarationItemKind::Function(rf) = &item.kind {
            let mut rf = rf.clone();
            rf.ty = ty;
            item.kind = DeclarationItemKind::Function(rf);
            Some(())
        } else {
            None
        }
    }

    pub(crate) fn register_namespace(
        &mut self,
        name: &str,
        annotation: TypedAnnotations,
    ) -> Option<DeclarationId> {
        self.arena
            .register_namespace(&self.current_namespace_id, name, annotation)
    }
}

#[cfg(test)]
mod tests {
    use super::{ResolverContext, ResolverStruct, StructKind};
    use crate::ResolverArena;
    use wiz_constants::INT32;
    use wiz_hir::typed_type::TypedType;

    #[test]
    fn test_context_name_environment() {
        let mut arena = ResolverArena::default();
        let mut context = ResolverContext::new(&mut arena);

        let env = context.get_current_name_environment();

        assert_eq!(
            env.get_type(vec![], INT32),
            Some(&ResolverStruct::new(TypedType::int32(), StructKind::Struct)),
        );
    }
}
