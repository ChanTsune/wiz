pub mod arena;
pub mod context;
pub mod declaration;
pub mod error;
mod name_environment;
pub mod result;
#[cfg(test)]
mod tests;

use crate::high_level_ir::declaration_id::DeclarationId;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::context::ResolverContext;
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use wiz_hir::typed_decl::{
    TypedArgDef, TypedDecl, TypedDeclKind, TypedExtension, TypedFun, TypedFunBody,
    TypedMemberFunction, TypedProtocol, TypedStoredProperty, TypedStruct, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedArray, TypedBinOp, TypedCall, TypedCallArg, TypedExprKind, TypedIf, TypedInstanceMember,
    TypedLiteralKind, TypedName, TypedPostfixUnaryOp, TypedPrefixUnaryOp, TypedPrefixUnaryOperator,
    TypedReturn, TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use wiz_hir::typed_file::{TypedFile, TypedSourceSet};
use wiz_hir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use wiz_hir::typed_type::{Package, TypedArgType, TypedFunctionType, TypedType, TypedValueType};
use wiz_hir::typed_type_constraint::TypedTypeConstraint;
use wiz_session::Session;

#[derive(Debug)]
pub(crate) struct TypeResolver<'s> {
    session: &'s mut Session,
    context: ResolverContext<'s>,
}

impl<'s> TypeResolver<'s> {
    pub fn new(session: &'s mut Session, arena: &'s mut ResolverArena) -> Self {
        Self {
            session,
            context: ResolverContext::new(arena),
        }
    }

    pub(crate) fn global_use<T: ToString>(&mut self, name_space: &[T]) {
        self.context
            .global_use_name_space(name_space.iter().map(T::to_string).collect())
    }

    pub fn preload_source_set(&mut self, s: &TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.preload_file(f),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name);
                items.iter().try_for_each(|i| self.preload_source_set(i))?;
                self.context.pop_name_space();
                Ok(())
            }
        }
    }

    pub fn preload_file(&mut self, f: &TypedFile) -> Result<()> {
        self.context.push_name_space(&f.name);
        for u in f.uses.iter() {
            self.context.use_name_space(u.package.names.clone());
        }
        for d in f.body.iter() {
            self.preload_decl(d)?;
        }
        for u in f.uses.iter() {
            self.context.unuse_name_space(&u.package.names);
        }
        self.context.pop_name_space();
        Ok(())
    }

    fn preload_decl(&mut self, d: &TypedDecl) -> Result<()> {
        match &d.kind {
            TypedDeclKind::Var(v) => {
                let v = self.typed_var(v.clone())?;
                self.context.register_value(
                    &v.name,
                    v.type_
                        .ok_or_else(|| ResolverError::from("Cannot resolve variable type"))?,
                    d.annotations.clone(),
                );
            }
            TypedDeclKind::Fun(f) => {
                let id = self
                    .context
                    .register_function(&f.name, TypedType::noting(), f.body.clone(),d.annotations.clone())
                    .unwrap();
                let fun = self.preload_fun(f)?;
                self.context
                    .update_value(&id, fun.type_().unwrap())
                    .unwrap();
            }
            TypedDeclKind::Struct(s) => {
                let _ = self.preload_struct(s)?;
            }
            TypedDeclKind::Class => todo!(),
            TypedDeclKind::Enum => todo!(),
            TypedDeclKind::Protocol(p) => {
                let _ = self.preload_protocol(p)?;
            }
            TypedDeclKind::Extension(e) => {
                let _ = self.preload_extension(e)?;
            }
        }
        Ok(())
    }

    fn preload_fun(&mut self, f: &TypedFun) -> Result<TypedFun> {
        self.context.push_name_space(&f.name);
        self.context.push_local_stack();
        if let Some(type_params) = &f.type_params {
            for type_param in type_params {
                self.context
                    .register_type_parameter(&type_param.name, Default::default());
            }
        }
        let arg_defs = f
            .arg_defs
            .iter()
            .map(|a| {
                let a = self.typed_arg_def(a.clone())?;
                self.context
                    .register_to_env(a.name.clone(), (DeclarationId::DUMMY, a.type_.clone()));
                Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&f.name, &f.return_type, &f.body)?;
        let fun = TypedFun {
            name: f.name.clone(),
            type_params: f.type_params.clone(),
            type_constraints: f.type_constraints.clone(),
            arg_defs,
            body: None,
            return_type: Some(return_type),
        };
        self.context.pop_local_stack();
        self.context.pop_name_space();
        Ok(fun)
    }

    fn preload_struct(&mut self, s: &TypedStruct) -> Result<()> {
        let TypedStruct {
            name,
            type_params: _,
            stored_properties,
            computed_properties,
            member_functions,
        } = s;
        self.context.push_name_space(name);
        let rs = self.context.current_type().ok_or_else(|| {
            ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
        })?;
        let this_type = rs.self_type();
        self.context.set_current_type(this_type);
        for stored_property in stored_properties.iter() {
            let type_ = self.context.full_type_name(&stored_property.type_)?;
            let rs = self.context.current_type_mut().ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.stored_properties
                .insert(stored_property.name.clone(), type_);
        }
        for computed_property in computed_properties.iter() {
            let type_ = self.context.full_type_name(&computed_property.type_)?;
            let rs = self.context.current_type_mut().ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.computed_properties
                .insert(computed_property.name.clone(), type_);
        }

        for member_function in member_functions.iter() {
            let type_ = self
                .context
                .full_type_name(&member_function.type_().unwrap())?;
            self.context.register_function(
                &member_function.name,
                type_.clone(),
                member_function.body.clone(),
                Default::default(),
            );
            let rs = self.context.current_type_mut().ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.member_functions
                .insert(member_function.name.clone(), type_);
        }
        self.context.clear_current_type();
        self.context.pop_name_space();
        Ok(())
    }

    fn preload_extension(&mut self, e: &TypedExtension) -> Result<()> {
        let TypedExtension {
            name,
            protocol,
            computed_properties,
            member_functions,
        } = e;
        let this_type = self.context.full_type_name(name)?;
        self.context.set_current_type(this_type.clone());
        for computed_property in computed_properties {
            let type_ = self.context.full_type_name(&computed_property.type_)?;
            let rs = self
                .context
                .arena
                .get_type_mut(
                    &this_type.package().into_resolved().names,
                    &this_type.name(),
                )
                .ok_or_else(|| {
                    ResolverError::from(format!(
                        "Struct {:?} not exist. Maybe before preload",
                        this_type
                    ))
                })?;
            rs.computed_properties
                .insert(computed_property.name.clone(), type_);
        }
        for member_function in member_functions {
            let type_ = self
                .context
                .full_type_name(&member_function.type_().unwrap())?;
            let rs = self
                .context
                .arena
                .get_type_mut(
                    &this_type.package().into_resolved().names,
                    &this_type.name(),
                )
                .ok_or_else(|| {
                    ResolverError::from(format!(
                        "Struct {:?} not exist. Maybe before preload",
                        this_type
                    ))
                })?;
            rs.member_functions
                .insert(member_function.name.clone(), type_);
        }
        self.context.clear_current_type();
        Ok(())
    }

    fn preload_protocol(&mut self, p: &TypedProtocol) -> Result<()> {
        let TypedProtocol {
            name,
            type_params: _,
            computed_properties,
            member_functions,
        } = p;
        self.context.push_name_space(name);
        let rs = self.context.current_type().ok_or_else(|| {
            ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
        })?;
        let this_type = rs.self_type();
        self.context.set_current_type(this_type);
        for computed_property in computed_properties.iter() {
            let type_ = self.context.full_type_name(&computed_property.type_)?;
            let rs = self.context.current_type_mut().ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.computed_properties
                .insert(computed_property.name.clone(), type_);
        }
        for member_function in member_functions.iter() {
            let type_ = self
                .context
                .full_type_name(&member_function.type_().unwrap())?;
            let rs = self.context.current_type_mut().ok_or_else(|| {
                ResolverError::from(format!("Struct {:?} not exist. Maybe before preload", name))
            })?;
            rs.member_functions
                .insert(member_function.name.clone(), type_);
        }
        self.context.clear_current_type();
        self.context.pop_name_space();
        Ok(())
    }

    pub fn source_set(&mut self, s: TypedSourceSet) -> Result<TypedSourceSet> {
        Ok(match s {
            TypedSourceSet::File(f) => TypedSourceSet::File(self.file(f)?),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(&name);
                let items = items
                    .into_iter()
                    .map(|i| self.source_set(i))
                    .collect::<Result<Vec<TypedSourceSet>>>()?;
                self.context.pop_name_space();
                TypedSourceSet::Dir { name, items }
            }
        })
    }

    pub fn file(&mut self, f: TypedFile) -> Result<TypedFile> {
        self.context.push_name_space(&f.name);
        for u in f.uses.iter() {
            self.context.use_name_space(u.package.names.clone());
        }
        let result = Ok(TypedFile {
            name: f.name,
            uses: vec![],
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<Result<Vec<_>>>()?,
        });
        for u in f.uses.iter() {
            self.context.unuse_name_space(&u.package.names);
        }
        self.context.pop_name_space();
        result
    }

    pub fn decl(&mut self, d: TypedDecl) -> Result<TypedDecl> {
        Ok(TypedDecl {
            annotations: d.annotations,
            package: d.package,
            modifiers: d.modifiers,
            kind: match d.kind {
                TypedDeclKind::Var(v) => TypedDeclKind::Var(self.typed_var(v)?),
                TypedDeclKind::Fun(f) => TypedDeclKind::Fun(self.typed_fun(f)?),
                TypedDeclKind::Struct(s) => TypedDeclKind::Struct(self.typed_struct(s)?),
                TypedDeclKind::Class => TypedDeclKind::Class,
                TypedDeclKind::Enum => TypedDeclKind::Enum,
                TypedDeclKind::Protocol(p) => TypedDeclKind::Protocol(self.typed_protocol(p)?),
                TypedDeclKind::Extension(e) => TypedDeclKind::Extension(self.typed_extension(e)?),
            },
        })
    }

    pub fn typed_var(&mut self, t: TypedVar) -> Result<TypedVar> {
        let TypedVar {
            is_mut,
            name,
            type_,
            value,
        } = t;
        let value = self.expr(
            value,
            match type_ {
                Some(type_) => Some(self.context.full_type_name(&type_)?),
                None => None,
            },
        )?;
        let v = TypedVar {
            is_mut,
            name,
            type_: value.type_(),
            value,
        };
        Ok(v)
    }

    fn typed_function_return_type(
        &mut self,
        name: &str,
        return_type: &Option<TypedType>,
        body: &Option<TypedFunBody>,
    ) -> Result<TypedType> {
        match return_type {
            None => match body {
                None => Err(ResolverError::from(format!(
                    "abstract function {:?} must be define type",
                    name
                ))),
                Some(TypedFunBody::Block(_)) => Ok(TypedType::unit()),
                Some(TypedFunBody::Expr(e)) => {
                    self.expr(e.clone(), None)?.type_().ok_or_else(|| {
                        ResolverError::from(format!(
                            "Can not resolve expr type at function {:?}",
                            name
                        ))
                    })
                }
            },
            Some(b) => self.context.full_type_name(b),
        }
    }

    fn typed_arg_def(&mut self, a: TypedArgDef) -> Result<TypedArgDef> {
        Ok(TypedArgDef {
            label: a.label,
            name: a.name,
            type_: self.context.full_type_name(&a.type_)?,
        })
    }

    pub fn typed_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        self.context.push_name_space(&f.name);
        self.context.push_local_stack();
        if let Some(type_params) = &f.type_params {
            for type_param in type_params {
                let vec_current_namespace = self.context.current_namespace();
                if let Some(tc) = &f.type_constraints {
                    let con = tc.iter().find(|t| t.type_.name() == type_param.name);
                    if let Some(con) = con {
                        for c in con.constraints.iter() {
                            let c = self.context.full_type_name(c)?;
                            let ne = self.context.get_current_name_environment();
                            let crs = ne.get_type_by_typed_type(c).unwrap();
                            let members = crs.member_functions.clone();
                            let rs = self
                                .context
                                .arena
                                .get_type_mut(&vec_current_namespace, &type_param.name)
                                .unwrap();
                            rs.member_functions.extend(members);
                        }
                    }
                };
            }
        }
        let arg_defs = f
            .arg_defs
            .into_iter()
            .map(|a| {
                let a = self.typed_arg_def(a)?;
                self.context
                    .register_to_env(a.name.clone(), (DeclarationId::DUMMY, a.type_.clone()));
                Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&f.name, &f.return_type, &f.body)?;
        let fun = TypedFun {
            name: f.name,
            type_params: f.type_params,
            type_constraints: match f.type_constraints {
                None => None,
                Some(tc) => Some(self.typed_type_constraints(tc)?),
            },
            arg_defs,
            body: match f.body {
                Some(b) => Some(self.typed_fun_body(b)?),
                None => None,
            },
            return_type: Some(return_type),
        };
        self.context.pop_local_stack();
        self.context.pop_name_space();
        Ok(fun)
    }

    pub fn typed_struct(&mut self, s: TypedStruct) -> Result<TypedStruct> {
        let TypedStruct {
            name,
            type_params,
            stored_properties,
            computed_properties, // TODO
            member_functions,
        } = s;
        self.context.push_name_space(&name);
        let rs = self.context.current_type().unwrap();
        let this_type = rs.self_type();
        self.context.set_current_type(this_type);
        let stored_properties = stored_properties
            .into_iter()
            .map(|s| self.typed_stored_property(s))
            .collect::<Result<Vec<_>>>()?;
        let computed_properties = computed_properties.into_iter().collect();
        let member_functions = member_functions
            .into_iter()
            .map(|m| self.typed_member_function(m))
            .collect::<Result<Vec<_>>>()?;
        self.context.clear_current_type();
        self.context.pop_name_space();
        Ok(TypedStruct {
            name,
            type_params,
            stored_properties,
            computed_properties,
            member_functions,
        })
    }

    fn typed_stored_property(&mut self, s: TypedStoredProperty) -> Result<TypedStoredProperty> {
        let TypedStoredProperty { name, type_ } = s;
        Ok(TypedStoredProperty {
            name,
            type_: self.context.full_type_name(&type_)?,
        })
    }

    fn typed_member_function(&mut self, mf: TypedMemberFunction) -> Result<TypedMemberFunction> {
        self.context.push_local_stack();
        let arg_defs = mf
            .arg_defs
            .into_iter()
            .map(|a| {
                let a = self.typed_arg_def(a)?;
                self.context
                    .register_to_env(a.name.clone(), (DeclarationId::DUMMY, a.type_.clone()));
                Ok(a)
            })
            .collect::<Result<Vec<_>>>()?;
        let return_type = self.typed_function_return_type(&mf.name, &mf.return_type, &mf.body)?;
        let result = Ok(TypedMemberFunction {
            name: mf.name,
            arg_defs,
            type_params: mf.type_params,
            body: match mf.body {
                None => None,
                Some(body) => Some(self.typed_fun_body(body)?),
            },
            return_type: Some(return_type),
        });
        self.context.pop_local_stack();
        result
    }

    fn typed_fun_body(&mut self, b: TypedFunBody) -> Result<TypedFunBody> {
        Ok(match b {
            TypedFunBody::Expr(e) => TypedFunBody::Expr(self.expr(e, None)?),
            TypedFunBody::Block(b) => TypedFunBody::Block(self.typed_block(b)?),
        })
    }

    fn typed_extension(&mut self, e: TypedExtension) -> Result<TypedExtension> {
        let this_type = self.context.full_type_name(&e.name)?;
        self.context.set_current_type(this_type.clone());
        let result = Ok(TypedExtension {
            name: this_type,
            protocol: match &e.protocol {
                Some(p) => Some(self.context.full_type_name(p)?),
                None => None,
            },
            computed_properties: e.computed_properties.into_iter().map(|i| i).collect(),
            member_functions: e
                .member_functions
                .into_iter()
                .map(|m| self.typed_member_function(m))
                .collect::<Result<_>>()?,
        });
        self.context.clear_current_type();
        result
    }

    fn typed_protocol(&mut self, p: TypedProtocol) -> Result<TypedProtocol> {
        self.context.push_name_space(&p.name);
        let rs = self.context.current_type().unwrap();
        self.context.set_current_type(rs.self_type());
        let result = TypedProtocol {
            name: p.name,
            type_params: p.type_params, // TODO type params
            member_functions: p
                .member_functions
                .into_iter()
                .map(|m| self.typed_member_function(m))
                .collect::<Result<_>>()?,
            computed_properties: p.computed_properties,
        };
        self.context.clear_current_type();
        self.context.pop_name_space();
        Ok(result)
    }

    fn typed_block(&mut self, b: TypedBlock) -> Result<TypedBlock> {
        Ok(TypedBlock {
            body: b
                .body
                .into_iter()
                .map(|s| self.stmt(s))
                .collect::<Result<_>>()?,
        })
    }

    fn typed_type_constraints(
        &mut self,
        type_constraints: Vec<TypedTypeConstraint>,
    ) -> Result<Vec<TypedTypeConstraint>> {
        type_constraints
            .iter()
            .map(|t| {
                Ok(TypedTypeConstraint {
                    type_: self.context.full_type_name(&t.type_)?,
                    constraints: t
                        .constraints
                        .iter()
                        .map(|c| self.context.full_type_name(c))
                        .collect::<Result<_>>()?,
                })
            })
            .collect::<Result<_>>()
    }

    pub fn expr(
        &mut self,
        e: TypedExprKind,
        type_annotation: Option<TypedType>,
    ) -> Result<TypedExprKind> {
        Ok(match e {
            TypedExprKind::Name(n) => TypedExprKind::Name(self.typed_name(n, type_annotation)?),
            TypedExprKind::Literal(l, t) => {
                let (kind, ty) = self.typed_literal(l, t, type_annotation)?;
                TypedExprKind::Literal(kind, ty)
            }
            TypedExprKind::BinOp(b) => TypedExprKind::BinOp(self.typed_binop(b)?),
            TypedExprKind::UnaryOp(u) => TypedExprKind::UnaryOp(self.typed_unary_op(u)?),
            TypedExprKind::Subscript(s) => TypedExprKind::Subscript(self.typed_subscript(s)?),
            TypedExprKind::Member(m) => TypedExprKind::Member(self.typed_instance_member(m)?),
            TypedExprKind::Array(a) => TypedExprKind::Array(self.typed_array(a)?),
            TypedExprKind::Tuple => TypedExprKind::Tuple,
            TypedExprKind::Dict => TypedExprKind::Dict,
            TypedExprKind::StringBuilder => TypedExprKind::StringBuilder,
            TypedExprKind::Call(c) => TypedExprKind::Call(self.typed_call(c)?),
            TypedExprKind::If(i) => TypedExprKind::If(self.typed_if(i)?),
            TypedExprKind::When => TypedExprKind::When,
            TypedExprKind::Lambda(l) => TypedExprKind::Lambda(l),
            TypedExprKind::Return(r) => TypedExprKind::Return(self.typed_return(r)?),
            TypedExprKind::TypeCast(t) => TypedExprKind::TypeCast(self.typed_type_cast(t)?),
        })
    }

    pub fn typed_name(
        &mut self,
        n: TypedName,
        type_annotation: Option<TypedType>,
    ) -> Result<TypedName> {
        let (type_, package) = {
            if n.package.is_resolved() {
                (n.type_.unwrap(), n.package)
            } else {
                self.context.resolve_name_type(
                    n.package.into_raw().names,
                    &n.name,
                    type_annotation,
                )?
            }
        };
        Ok(TypedName {
            package,
            type_: Some(type_),
            name: n.name,
            type_arguments: match n.type_arguments {
                None => None,
                Some(t) => Some(
                    t.iter()
                        .map(|ta| self.context.full_type_name(ta))
                        .collect::<Result<_>>()?,
                ),
            },
        })
    }

    fn typed_literal(
        &mut self,
        l: TypedLiteralKind,
        type_: Option<TypedType>,
        type_annotation: Option<TypedType>,
    ) -> Result<(TypedLiteralKind, Option<TypedType>)> {
        let ty = match &l {
            TypedLiteralKind::Integer { .. } => {
                if type_.is_some() {
                    type_
                } else if type_annotation.is_some() {
                    type_annotation
                } else {
                    Some(TypedType::int64())
                }
            }
            TypedLiteralKind::FloatingPoint { .. } => {
                if type_.is_some() {
                    type_
                } else if type_annotation.is_some() {
                    type_annotation
                } else {
                    Some(TypedType::double())
                }
            }
            TypedLiteralKind::String { .. } => Some(TypedType::string_ref()),
            TypedLiteralKind::Boolean { .. } => Some(TypedType::bool()),
            TypedLiteralKind::NullLiteral => type_annotation,
        };
        Ok((l, ty))
    }

    pub fn typed_unary_op(&mut self, u: TypedUnaryOp) -> Result<TypedUnaryOp> {
        Ok(match u {
            TypedUnaryOp::Prefix(p) => TypedUnaryOp::Prefix(self.typed_prefix_unary_op(p)?),
            TypedUnaryOp::Postfix(p) => TypedUnaryOp::Postfix(self.typed_postfix_unary_op(p)?),
        })
    }

    pub fn typed_prefix_unary_op(&mut self, u: TypedPrefixUnaryOp) -> Result<TypedPrefixUnaryOp> {
        let target = Box::new(self.expr(*u.target, None)?);
        Ok(match &u.operator {
            TypedPrefixUnaryOperator::Negative
            | TypedPrefixUnaryOperator::Positive
            | TypedPrefixUnaryOperator::Not => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: target.type_(),
                target,
            },
            TypedPrefixUnaryOperator::Reference => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: target
                    .type_()
                    .map(|t| TypedType::Value(TypedValueType::Reference(Box::new(t)))),
                target,
            },
            TypedPrefixUnaryOperator::Dereference => TypedPrefixUnaryOp {
                operator: u.operator,
                type_: match target.type_() {
                    None => None,
                    Some(TypedType::Value(TypedValueType::Reference(t))) => Some(*t),
                    Some(TypedType::Value(TypedValueType::Pointer(t))) => Some(*t),
                    Some(_) => None,
                },
                target,
            },
        })
    }

    pub fn typed_postfix_unary_op(
        &mut self,
        u: TypedPostfixUnaryOp,
    ) -> Result<TypedPostfixUnaryOp> {
        let target = Box::new(self.expr(*u.target, None)?);
        Ok(TypedPostfixUnaryOp {
            operator: u.operator,
            type_: target.type_(),
            target,
        })
    }

    pub fn typed_binop(&mut self, b: TypedBinOp) -> Result<TypedBinOp> {
        let left = self.expr(*b.left, None)?;
        let right = self.expr(*b.right, None)?;
        let (left, right) = match (left, right) {
            (
                TypedExprKind::Literal(TypedLiteralKind::Integer(left_value), left_type),
                TypedExprKind::Literal(TypedLiteralKind::Integer(right_value), right_type),
            ) => (
                TypedExprKind::Literal(TypedLiteralKind::Integer(left_value), left_type),
                TypedExprKind::Literal(TypedLiteralKind::Integer(right_value), right_type),
            ),
            (left, TypedExprKind::Literal(TypedLiteralKind::Integer(value), type_)) => {
                let left_type = left.type_();
                let is_integer = match &left_type {
                    None => false,
                    Some(t) => t.is_integer(),
                };
                if is_integer {
                    (
                        left,
                        TypedExprKind::Literal(TypedLiteralKind::Integer(value), left_type),
                    )
                } else {
                    (
                        left,
                        TypedExprKind::Literal(TypedLiteralKind::Integer(value), type_),
                    )
                }
            }
            (left, right) => (left, right),
        };
        let type_ = self.context.resolve_binop_type(
            left.type_().unwrap(),
            b.operator.clone(),
            right.type_().unwrap(),
        )?;
        Ok(TypedBinOp {
            left: Box::new(left),
            operator: b.operator,
            right: Box::new(right),
            type_: Some(type_),
        })
    }

    pub fn typed_instance_member(&mut self, m: TypedInstanceMember) -> Result<TypedInstanceMember> {
        let target = self.expr(*m.target, None)?;
        let type_ = self
            .context
            .resolve_member_type(target.type_().unwrap(), &m.name)?;
        Ok(TypedInstanceMember {
            target: Box::new(target),
            name: m.name,
            is_safe: m.is_safe,
            type_: Some(type_),
        })
    }

    pub fn typed_subscript(&mut self, s: TypedSubscript) -> Result<TypedSubscript> {
        let target = self.expr(*s.target, None)?;
        let target_type = target.type_().unwrap();
        if let TypedType::Value(v) = target_type {
            match v {
                TypedValueType::Value(v) => {
                    if v.is_string() {
                        return Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i, None))
                                .collect::<Result<_>>()?,
                            type_: Some(TypedType::uint8()),
                        });
                    }
                }
                TypedValueType::Array(et, _) => {
                    return Ok(TypedSubscript {
                        target: Box::new(target),
                        indexes: s
                            .indexes
                            .into_iter()
                            .map(|i| self.expr(i, None))
                            .collect::<Result<_>>()?,
                        type_: Some(*et),
                    })
                }
                TypedValueType::Tuple(_) => {
                    todo!()
                }
                TypedValueType::Pointer(p) => {
                    return Ok(TypedSubscript {
                        target: Box::new(target),
                        indexes: s
                            .indexes
                            .into_iter()
                            .map(|i| self.expr(i, None))
                            .collect::<Result<_>>()?,
                        type_: Some(*p),
                    })
                }
                TypedValueType::Reference(r) => {
                    if r.is_string() {
                        return Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i, None))
                                .collect::<Result<_>>()?,
                            type_: Some(TypedType::uint8()),
                        });
                    }
                }
            }
        }
        Ok(TypedSubscript {
            target: Box::new(target),
            indexes: s
                .indexes
                .into_iter()
                .map(|i| self.expr(i, None))
                .collect::<Result<_>>()?,
            type_: s.type_,
        })
    }

    pub fn typed_array(&mut self, a: TypedArray) -> Result<TypedArray> {
        let elements = a
            .elements
            .into_iter()
            .map(|e| self.expr(e, None))
            .collect::<Result<Vec<_>>>()?;
        let len = elements.len();
        Ok(if let Some(e) = elements.get(0) {
            let e_type = e.type_();
            if elements.iter().all(|e| e.type_() == e_type) {
                TypedArray {
                    elements,
                    type_: e_type
                        .map(|e| TypedType::Value(TypedValueType::Array(Box::new(e), len))),
                }
            } else {
                return Err(ResolverError::from("Array elements must be same type."));
            }
        } else {
            // empty case
            TypedArray {
                elements,
                type_: None,
            }
        })
    }

    pub fn typed_call(&mut self, c: TypedCall) -> Result<TypedCall> {
        let (target, args) = match self.expr((*c.target).clone(), None) {
            Ok(TypedExprKind::Name(n)) => {
                let target = TypedExprKind::Name(n);
                if let TypedType::Function(f) = target.type_().unwrap() {
                    if c.args.len() != f.arguments.len() {
                        Err(ResolverError::from(format!(
                            "{:?} required {} arguments, but {} were given.",
                            target,
                            f.arguments.len(),
                            c.args.len()
                        )))
                    } else {
                        Ok((
                            target,
                            c.args
                                .into_iter()
                                .zip(f.arguments)
                                .map(|(c, annotation)| self.typed_call_arg(c, Some(annotation.typ)))
                                .collect::<Result<Vec<_>>>()?,
                        ))
                    }
                } else if let TypedType::Type(t) = target.type_().unwrap() {
                    let rs = self
                        .context
                        .arena
                        .get_type(&t.package().into_resolved().names, &t.name())
                        .unwrap();
                    if rs.stored_properties.len() != c.args.len() {
                        Err(ResolverError::from(format!(
                            "`{}` required {} arguments, but {} were given.",
                            t.name(),
                            rs.stored_properties.len(),
                            c.args.len()
                        )))
                    } else {
                        Ok((
                            target,
                            c.args
                                .into_iter()
                                .zip(rs.stored_properties.clone().into_iter().map(|(_, t)| t))
                                .map(|(c, annotation)| self.typed_call_arg(c, Some(annotation)))
                                .collect::<Result<Vec<_>>>()?,
                        ))
                    }
                } else {
                    Err(ResolverError::from(format!(
                        "{:?} is not callable.",
                        target
                    )))
                }
            }
            Ok(_) | Err(_) => {
                let args = c
                    .args
                    .into_iter()
                    .map(|c| self.typed_call_arg(c, None))
                    .collect::<Result<Vec<_>>>()?;
                let arg_annotation = TypedType::Function(Box::new(TypedFunctionType {
                    arguments: args
                        .iter()
                        .map(|a| TypedArgType {
                            label: a.label.clone().unwrap_or_else(|| "_".to_string()),
                            typ: a.arg.type_().unwrap(),
                        })
                        .collect(),
                    return_type: TypedType::noting(),
                }));
                let target = self.expr(*c.target, Some(arg_annotation))?;
                Ok((target, args))
            }
        }?;
        let c_type = match target.type_().unwrap() {
            TypedType::Value(v) => Err(ResolverError::from(format!("{:?} is not callable.", v))),
            TypedType::Type(t) => Ok(*t),
            TypedType::Self_ => Err(ResolverError::from("Self is not callable.")),
            TypedType::Function(f) => Ok(f.return_type),
        }?;
        Ok(TypedCall {
            target: Box::new(target),
            args,
            type_: Some(c_type),
        })
    }

    pub fn typed_call_arg(
        &mut self,
        a: TypedCallArg,
        type_annotation: Option<TypedType>,
    ) -> Result<TypedCallArg> {
        Ok(TypedCallArg {
            label: a.label,
            arg: Box::new(self.expr(*a.arg, type_annotation)?),
            is_vararg: a.is_vararg,
        })
    }

    pub fn typed_if(&mut self, i: TypedIf) -> Result<TypedIf> {
        let condition = Box::new(self.expr(*i.condition, None)?);
        let body = self.typed_block(i.body)?;
        let else_body = match i.else_body {
            Some(b) => Some(self.typed_block(b)?),
            None => None,
        };
        let type_ = if let Some(else_body) = &else_body {
            else_body.type_().unwrap_or_else(TypedType::noting)
        } else {
            TypedType::noting()
        };
        Ok(TypedIf {
            condition,
            body,
            else_body,
            type_: Some(type_),
        })
    }

    pub fn typed_return(&mut self, r: TypedReturn) -> Result<TypedReturn> {
        let value = match r.value {
            Some(v) => Some(Box::new(self.expr(*v, None)?)),
            None => None,
        };
        Ok(TypedReturn { value })
    }

    pub fn typed_type_cast(&mut self, t: TypedTypeCast) -> Result<TypedTypeCast> {
        Ok(TypedTypeCast {
            target: Box::new(self.expr(*t.target, None)?),
            is_safe: t.is_safe,
            type_: Some(self.context.full_type_name(&t.type_.unwrap())?),
        })
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Result<TypedStmt> {
        Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(self.expr(e, None)?),
            TypedStmt::Decl(d) => TypedStmt::Decl({
                let mut d = self.decl(d)?;
                if let TypedDeclKind::Var(v) = &d.kind {
                    d.package = Package::new();
                    self.context.register_to_env(
                        v.name.clone(),
                        (
                            DeclarationId::DUMMY,
                            v.type_.clone().ok_or_else(|| {
                                ResolverError::from("Cannot resolve variable type")
                            })?,
                        ),
                    )
                };
                d
            }),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(self.assignment_stmt(a)?),
            TypedStmt::Loop(l) => TypedStmt::Loop(self.typed_loop_stmt(l)?),
        })
    }

    pub fn assignment_stmt(&mut self, a: TypedAssignmentStmt) -> Result<TypedAssignmentStmt> {
        Ok(match a {
            TypedAssignmentStmt::Assignment(a) => {
                TypedAssignmentStmt::Assignment(self.typed_assignment(a)?)
            }
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(self.typed_assignment_and_operation(a)?)
            }
        })
    }

    pub fn typed_assignment(&mut self, a: TypedAssignment) -> Result<TypedAssignment> {
        Ok(TypedAssignment {
            target: self.expr(a.target, None)?,
            value: self.expr(a.value, None)?,
        })
    }

    pub fn typed_assignment_and_operation(
        &mut self,
        a: TypedAssignmentAndOperation,
    ) -> Result<TypedAssignmentAndOperation> {
        Ok(TypedAssignmentAndOperation {
            target: self.expr(a.target, None)?,
            operator: a.operator, // TODO
            value: self.expr(a.value, None)?,
        })
    }

    pub fn typed_loop_stmt(&mut self, l: TypedLoopStmt) -> Result<TypedLoopStmt> {
        Ok(match l {
            TypedLoopStmt::While(w) => TypedLoopStmt::While(self.typed_while_loop_stmt(w)?),
            TypedLoopStmt::For(f) => TypedLoopStmt::For(self.typed_for_loop_stmt(f)?),
        })
    }

    pub fn typed_while_loop_stmt(&mut self, w: TypedWhileLoopStmt) -> Result<TypedWhileLoopStmt> {
        let TypedWhileLoopStmt { condition, block } = w;
        let condition = self.expr(condition, None)?;
        if !condition.type_().unwrap().is_boolean() {
            return Err(ResolverError::from("while loop condition must be boolean"));
        };
        Ok(TypedWhileLoopStmt {
            condition,
            block: self.typed_block(block)?,
        })
    }

    pub fn typed_for_loop_stmt(&mut self, f: TypedForStmt) -> Result<TypedForStmt> {
        let TypedForStmt {
            values,
            iterator,
            block,
        } = f;
        Ok(TypedForStmt {
            values,
            iterator: self.expr(iterator, None)?,
            block: self.typed_block(block)?,
        })
    }
}
