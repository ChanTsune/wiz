pub mod context;
pub mod error;
pub mod result;
#[cfg(test)]
mod tests;

use crate::high_level_ir::type_resolver::context::{EnvValue, ResolverContext, ResolverStruct};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedInitializer, TypedMemberFunction,
    TypedStruct, TypedValueArgDef, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedArray, TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember,
    TypedLiteral, TypedName, TypedPostfixUnaryOp, TypedPrefixUnaryOp, TypedReturn, TypedSubscript,
    TypedTypeCast, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedValueType};

#[derive(Debug, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            context: ResolverContext::new(),
        }
    }

    pub fn detect_type_from_source_set(&mut self, s: &TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.detect_type(f),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name.clone());
                items
                    .iter()
                    .map(|i| self.detect_type_from_source_set(i))
                    .collect::<Result<Vec<()>>>()?;
                self.context.pop_name_space();
                Result::Ok(())
            }
        }
    }

    pub fn detect_type(&mut self, f: &TypedFile) -> Result<()> {
        self.context.push_name_space(f.name.clone());
        let current_namespace = self.context.current_namespace.clone();
        let ns = self.context.get_current_namespace_mut()?;
        for d in f.body.iter() {
            match d {
                TypedDecl::Struct(s) => {
                    ns.register_type(s.name.clone(), ResolverStruct::new());
                    ns.register_value(
                        s.name.clone(),
                        TypedType::Type(TypedValueType {
                            package: Some(Package {
                                names: current_namespace.clone(),
                            }),
                            name: s.name.clone(),
                            type_args: None,
                        }),
                    );
                }
                TypedDecl::Class => {}
                TypedDecl::Enum => {}
                TypedDecl::Protocol => {}
                _ => {}
            }
        }
        self.context.pop_name_space();
        Result::Ok(())
    }

    pub fn preload_source_set(&mut self, s: TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.preload_file(f),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name);
                items
                    .into_iter()
                    .map(|i| self.preload_source_set(i))
                    .collect::<Result<Vec<()>>>()?;
                self.context.pop_name_space();
                Result::Ok(())
            }
        }
    }

    pub fn preload_file(&mut self, f: TypedFile) -> Result<()> {
        let name = f.name.clone();
        if name != String::from("builtin.ll") {
            self.context.push_name_space(f.name.clone());
        };
        for d in f.body {
            self.preload_decl(d)?;
        }
        if name != String::from("builtin.ll") {
            self.context.pop_name_space();
        };
        Result::Ok(())
    }

    fn preload_decl(&mut self, d: TypedDecl) -> Result<()> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.typed_var(v)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace.register_value(
                    v.name,
                    v.type_
                        .ok_or(ResolverError::from("Cannot resolve variable type"))?,
                );
            }
            TypedDecl::Fun(f) => {
                let fun = self.preload_fun(f)?;
                let namespace = self.context.get_current_namespace_mut()?;
                namespace.register_value(fun.name.clone(), fun.type_().unwrap());
            }
            TypedDecl::Struct(_) => {}
            TypedDecl::Class => {}
            TypedDecl::Enum => {}
            TypedDecl::Protocol => {}
            TypedDecl::Extension => {}
            TypedDecl::Use(_) => {}
        }
        Result::Ok(())
    }

    pub fn preload_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        let c_name_space = self.context.current_namespace.clone();
        self.context.push_local_stack();
        let arg_defs = f
            .arg_defs
            .iter()
            .map(|a| {
                let a = self.typed_arg_def(a.clone())?;
                self.context.register_to_env(
                    a.name(),
                    EnvValue::from(a.type_()
                        .ok_or(ResolverError::from("Can not resolve 'self type'"))?),
                );
                Result::Ok(a)
            })
            .collect::<Result<Vec<TypedArgDef>>>()?;
        let return_type = self.typed_function_return_type(&f)?;
        let fun = TypedFun {
            annotations: f.annotations,
            package: Some(Package::new(c_name_space)),
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params, // TODO
            arg_defs,
            body: None,
            return_type: Some(return_type),
        };
        self.context.pop_local_stack();
        Result::Ok(fun)
    }

    pub fn source_set(&mut self, s: TypedSourceSet) -> Result<TypedSourceSet> {
        Result::Ok(match s {
            TypedSourceSet::File(f) => TypedSourceSet::File(self.file(f)?),
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name.clone());
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
        let name = f.name.clone();
        if name != String::from("builtin.ll") {
            self.context.push_name_space(f.name.clone());
        };
        let result = Result::Ok(TypedFile {
            name: f.name,
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<Result<Vec<TypedDecl>>>()?,
        });
        if name != String::from("builtin.ll") {
            self.context.pop_name_space();
        };
        result
    }

    pub fn decl(&mut self, d: TypedDecl) -> Result<TypedDecl> {
        Result::Ok(match d {
            TypedDecl::Var(v) => TypedDecl::Var(self.typed_var(v)?),
            TypedDecl::Fun(f) => TypedDecl::Fun(self.typed_fun(f)?),
            TypedDecl::Struct(s) => TypedDecl::Struct(self.typed_struct(s)?),
            TypedDecl::Class => TypedDecl::Class,
            TypedDecl::Enum => TypedDecl::Enum,
            TypedDecl::Protocol => TypedDecl::Protocol,
            TypedDecl::Extension => TypedDecl::Extension,
            TypedDecl::Use(u) => TypedDecl::Use(u),
        })
    }

    pub fn typed_var(&mut self, t: TypedVar) -> Result<TypedVar> {
        let value = self.expr(t.value)?;
        let v_type = match (t.type_, value.type_()) {
            (Some(vt), Some(et)) => {
                if vt != et {
                    Result::Err(ResolverError::from(format!(
                        "Type unmatched {:?} != {:?}",
                        vt, et
                    )))
                } else {
                    Result::Ok(et)
                }
            }
            (Some(vt), None) => {
                eprintln!("maybe invalid type ...");
                Result::Ok(vt)
            }
            (None, Some(et)) => Result::Ok(et),
            (None, None) => Result::Err(ResolverError::from(format!(
                "Can not resolve var type {:?}",
                value
            ))),
        }?;
        let v = TypedVar {
            annotations: t.annotations,
            package: None,
            is_mut: t.is_mut,
            name: t.name,
            type_: Some(v_type),
            value,
        };
        self.context.register_to_env(
            v.name.clone(),
            EnvValue::from(v.type_
                .clone()
                .ok_or(ResolverError::from("Cannot resolve variable type"))?),
        );
        Result::Ok(v)
    }

    fn typed_function_return_type(&mut self, f: &TypedFun) -> Result<TypedType> {
        match &f.return_type {
            None => match &f.body {
                None => Result::Err(ResolverError::from(format!(
                    "abstract function {:?} must be define type",
                    f.name
                ))),
                Some(TypedFunBody::Block(_)) => Result::Ok(TypedType::unit()),
                Some(TypedFunBody::Expr(e)) => {
                    self.expr(e.clone())?
                        .type_()
                        .ok_or(ResolverError::from(format!(
                            "Can not resolve expr type at function {:?}",
                            f.name
                        )))
                }
            },
            Some(b) => self.context.full_type_name(b.clone()),
        }
    }

    fn typed_arg_def(&mut self, a: TypedArgDef) -> Result<TypedArgDef> {
        Result::Ok(match a {
            TypedArgDef::Value(a) => TypedArgDef::Value(TypedValueArgDef {
                label: a.label,
                name: a.name,
                type_: self.context.full_type_name(a.type_)?,
            }),
            TypedArgDef::Self_(_) => {
                let self_type = self.context.get_current_type();
                TypedArgDef::Self_(self_type)
            }
            TypedArgDef::RefSelf(_) => {
                let self_type = self.context.get_current_type();
                TypedArgDef::RefSelf(self_type)
            }
        })
    }

    pub fn typed_fun(&mut self, f: TypedFun) -> Result<TypedFun> {
        let c_name_space = self.context.current_namespace.clone();
        self.context.push_local_stack();
        let arg_defs = f
            .arg_defs
            .iter()
            .map(|a| {
                let a = self.typed_arg_def(a.clone())?;
                self.context.register_to_env(
                    a.name(),
                    EnvValue::from(
                        a.type_()
                            .ok_or(ResolverError::from("Can not resolve 'self type'"))?,
                    )
                );
                Result::Ok(a)
            })
            .collect::<Result<Vec<TypedArgDef>>>()?;
        let return_type = self.typed_function_return_type(&f)?;
        let fun = TypedFun {
            annotations: f.annotations,
            package: Some(Package::new(c_name_space)),
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params, // TODO
            arg_defs,
            body: match f.body {
                Some(b) => Some(self.typed_fun_body(b)?),
                None => None,
            },
            return_type: Some(return_type),
        };
        let fun_name = fun.name.clone();
        let fun_type = fun.type_();
        let result = Result::Ok(fun);
        self.context.pop_local_stack();
        let ns = self.context.get_current_namespace_mut()?;
        ns.register_value(fun_name, fun_type.unwrap());
        result
    }

    pub fn typed_struct(&mut self, s: TypedStruct) -> Result<TypedStruct> {
        let TypedStruct {
            annotations,
            package: _,
            name,
            type_params,
            init,                // TODO
            stored_properties,   // TODO
            computed_properties, // TODO
            member_functions,    // TODO
            static_function,     // TODO
        } = s;
        let current_namespace = self.context.current_namespace.clone();
        let ns = self.context.get_current_namespace_mut()?;
        let this_type = TypedType::Value(TypedValueType {
            package: Some(Package::new(current_namespace)),
            name: name.clone(),
            type_args: None,
        });
        let rs = ns.get_type_mut(&name).ok_or(ResolverError::from(format!(
            "Struct {:?} not exist. Maybe before preload",
            name
        )))?;
        for sp in stored_properties.iter() {
            rs.stored_properties
                .insert(sp.name.clone(), sp.type_.clone());
        }
        for cp in computed_properties.iter() {
            rs.computed_properties
                .insert(cp.name.clone(), cp.type_.clone());
        }
        for mf in member_functions.iter() {
            rs.member_functions
                .insert(mf.name.clone(), mf.type_().unwrap());
        }
        for sf in static_function.iter() {
            rs.static_functions
                .insert(sf.name.clone(), sf.type_().unwrap());
        }
        for ini in init.iter() {
            rs.static_functions.insert(
                String::from("init"),
                TypedType::Function(Box::new(TypedFunctionType {
                    arguments: ini.args.clone(),
                    return_type: this_type.clone(),
                })),
            );
        }
        self.context.set_current_type(this_type);
        let init = init
            .into_iter()
            .map(|i| self.typed_initializer(i))
            .collect::<Result<Vec<TypedInitializer>>>()?;
        let stored_properties = stored_properties.into_iter().collect();
        let computed_properties = computed_properties.into_iter().collect();
        let member_functions = member_functions
            .into_iter()
            .map(|m| self.typed_member_function(m))
            .collect::<Result<Vec<TypedMemberFunction>>>()?;
        let static_function = static_function.into_iter().collect();
        self.context.clear_current_type();
        Result::Ok(TypedStruct {
            annotations,
            package: Some(Package::new(self.context.current_namespace.clone())),
            name,
            type_params,
            init,
            stored_properties,
            computed_properties,
            member_functions,
            static_function,
        })
    }

    fn typed_initializer(&mut self, i: TypedInitializer) -> Result<TypedInitializer> {
        let self_type = self.context.get_current_type();
        let ns = self.context.get_current_namespace_mut()?;
        ns.register_value(
            String::from("self"),
            self_type.ok_or(ResolverError::from("Can not resolve 'self type'"))?,
        );
        Result::Ok(TypedInitializer {
            args: i
                .args
                .into_iter()
                .map(|a| {
                    let a = self.typed_arg_def(a)?;
                    let ns = self.context.get_current_namespace_mut()?;
                    ns.register_value(
                        a.name(),
                        a.type_()
                            .ok_or(ResolverError::from("Can not resolve 'self type'"))?,
                    );
                    Result::Ok(a)
                })
                .collect::<Result<Vec<TypedArgDef>>>()?,
            body: self.typed_fun_body(i.body)?,
        })
    }

    fn typed_member_function(&mut self, mf: TypedMemberFunction) -> Result<TypedMemberFunction> {
        self.context.push_local_stack();
        let result = Result::Ok(TypedMemberFunction {
            name: mf.name,
            args: mf
                .args
                .into_iter()
                .map(|a| {
                    let a = self.typed_arg_def(a)?;
                    self.context.register_to_env(
                        a.name(),
                        EnvValue::from(a.type_()
                            .ok_or(ResolverError::from("Can not resolve `self`"))?),
                    );
                    Result::Ok(a)
                })
                .collect::<Result<Vec<TypedArgDef>>>()?,
            type_params: mf.type_params,
            body: match mf.body {
                None => None,
                Some(body) => Some(self.typed_fun_body(body)?),
            },
            return_type: match mf.return_type {
                Some(b) => Some(self.context.full_type_name(b.clone())?),
                None => {
                    todo!()
                }
            },
        });
        self.context.pop_local_stack();
        result
    }

    fn typed_fun_body(&mut self, b: TypedFunBody) -> Result<TypedFunBody> {
        Result::Ok(match b {
            TypedFunBody::Expr(e) => TypedFunBody::Expr(self.expr(e)?),
            TypedFunBody::Block(b) => TypedFunBody::Block(self.typed_block(b)?),
        })
    }

    fn typed_block(&mut self, b: TypedBlock) -> Result<TypedBlock> {
        Result::Ok(TypedBlock {
            body: b
                .body
                .into_iter()
                .map(|s| self.stmt(s))
                .collect::<Result<Vec<TypedStmt>>>()?,
        })
    }

    pub fn expr(&mut self, e: TypedExpr) -> Result<TypedExpr> {
        Result::Ok(match e {
            TypedExpr::Name(n) => TypedExpr::Name(self.typed_name(n)?),
            TypedExpr::Literal(l) => TypedExpr::Literal(l),
            TypedExpr::BinOp(b) => TypedExpr::BinOp(self.typed_binop(b)?),
            TypedExpr::UnaryOp(u) => TypedExpr::UnaryOp(self.typed_unary_op(u)?),
            TypedExpr::Subscript(s) => TypedExpr::Subscript(self.typed_subscript(s)?),
            TypedExpr::Member(m) => TypedExpr::Member(self.typed_instance_member(m)?),
            TypedExpr::Array(a) => TypedExpr::Array(self.typed_array(a)?),
            TypedExpr::Tuple => TypedExpr::Tuple,
            TypedExpr::Dict => TypedExpr::Dict,
            TypedExpr::StringBuilder => TypedExpr::StringBuilder,
            TypedExpr::Call(c) => TypedExpr::Call(self.typed_call(c)?),
            TypedExpr::If(i) => TypedExpr::If(self.typed_if(i)?),
            TypedExpr::When => TypedExpr::When,
            TypedExpr::Lambda(l) => TypedExpr::Lambda(l),
            TypedExpr::Return(r) => TypedExpr::Return(self.typed_return(r)?),
            TypedExpr::TypeCast(t) => TypedExpr::TypeCast(self.typed_type_cast(t)?),
        })
    }

    pub fn typed_name(&mut self, n: TypedName) -> Result<TypedName> {
        let (type_, package) = self.context.resolve_name_type(
            match n.package {
                None => {
                    vec![]
                }
                Some(p) => p.names,
            },
            n.name.clone(),
        )?;
        Result::Ok(TypedName {
            package,
            type_: Some(type_),
            name: n.name,
        })
    }

    pub fn typed_unary_op(&mut self, u: TypedUnaryOp) -> Result<TypedUnaryOp> {
        Result::Ok(match u {
            TypedUnaryOp::Prefix(p) => TypedUnaryOp::Prefix(self.typed_prefix_unary_op(p)?),
            TypedUnaryOp::Postfix(p) => TypedUnaryOp::Postfix(self.typed_postfix_unary_op(p)?),
        })
    }

    pub fn typed_prefix_unary_op(&mut self, u: TypedPrefixUnaryOp) -> Result<TypedPrefixUnaryOp> {
        let target = Box::new(self.expr(*u.target)?);
        Result::Ok(TypedPrefixUnaryOp {
            kind: u.kind,
            type_: target.type_(),
            target,
        })
    }

    pub fn typed_postfix_unary_op(
        &mut self,
        u: TypedPostfixUnaryOp,
    ) -> Result<TypedPostfixUnaryOp> {
        let target = Box::new(self.expr(*u.target)?);
        Result::Ok(TypedPostfixUnaryOp {
            kind: u.kind,
            type_: target.type_(),
            target,
        })
    }

    pub fn typed_binop(&mut self, b: TypedBinOp) -> Result<TypedBinOp> {
        let left = self.expr(*b.left)?;
        let right = self.expr(*b.right)?;
        let (left, right) = match (left, right) {
            (
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: left_value,
                    type_: left_type,
                }),
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: right_value,
                    type_: right_type,
                }),
            ) => (
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: left_value,
                    type_: left_type,
                }),
                TypedExpr::Literal(TypedLiteral::Integer {
                    value: right_value,
                    type_: right_type,
                }),
            ),
            (left, TypedExpr::Literal(TypedLiteral::Integer { value, type_ })) => {
                let left_type = left.type_();
                let is_integer = match &left_type {
                    None => false,
                    Some(t) => t.is_integer(),
                };
                if is_integer {
                    (
                        left,
                        TypedExpr::Literal(TypedLiteral::Integer {
                            value,
                            type_: left_type,
                        }),
                    )
                } else {
                    (
                        left,
                        TypedExpr::Literal(TypedLiteral::Integer { value, type_ }),
                    )
                }
            }
            (left, right) => (left, right),
        };
        let type_ = self.context.resolve_binop_type(
            left.type_().unwrap(),
            &*b.kind,
            right.type_().unwrap(),
        )?;
        Result::Ok(TypedBinOp {
            left: Box::new(left),
            kind: b.kind,
            right: Box::new(right),
            type_: Some(type_),
        })
    }

    pub fn typed_instance_member(&mut self, m: TypedInstanceMember) -> Result<TypedInstanceMember> {
        let target = self.expr(*m.target)?;
        let type_ = self
            .context
            .resolve_member_type(target.type_().unwrap(), m.name.clone())?;
        Result::Ok(TypedInstanceMember {
            target: Box::new(target),
            name: m.name,
            is_safe: m.is_safe,
            type_: Some(type_),
        })
    }

    pub fn typed_subscript(&mut self, s: TypedSubscript) -> Result<TypedSubscript> {
        let target = self.expr(*s.target)?;
        let target_type = target.type_().unwrap();
        if let TypedType::Value(v) = target_type {
            if v.is_unsafe_pointer() {
                if let Some(mut ags) = v.type_args {
                    if ags.len() == 1 {
                        return Result::Ok(TypedSubscript {
                            target: Box::new(target),
                            indexes: s
                                .indexes
                                .into_iter()
                                .map(|i| self.expr(i))
                                .collect::<Result<Vec<TypedExpr>>>()?,
                            type_: Some(ags.remove(0)),
                        });
                    }
                }
            }
        }
        Result::Ok(TypedSubscript {
            target: Box::new(target),
            indexes: s
                .indexes
                .into_iter()
                .map(|i| self.expr(i))
                .collect::<Result<Vec<TypedExpr>>>()?,
            type_: s.type_,
        })
    }

    pub fn typed_array(&mut self, a: TypedArray) -> Result<TypedArray> {
        let elements = a
            .elements
            .into_iter()
            .map(|e| self.expr(e))
            .collect::<Result<Vec<TypedExpr>>>()?;
        Result::Ok(if let Some(e) = elements.get(0) {
            let e_type = e.type_();
            if elements.iter().all(|e| e.type_() == e_type) {
                TypedArray {
                    elements,
                    type_: todo!(),
                }
            } else {
                return Result::Err(ResolverError::from("Array elements must be same type."));
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
        let target = Box::new(self.expr(*c.target)?);
        let c_type = match target.type_().unwrap() {
            TypedType::Value(v) | TypedType::Type(v) | TypedType::Reference(v) => {
                Result::Err(ResolverError::from(format!("{:?} is not callable.", v)))
            }
            TypedType::Function(f) => Result::Ok(f.return_type),
        }?;
        Result::Ok(TypedCall {
            target,
            args: c
                .args
                .into_iter()
                .map(|c| self.typed_call_arg(c))
                .collect::<Result<Vec<TypedCallArg>>>()?,
            type_: Some(c_type),
        })
    }

    pub fn typed_call_arg(&mut self, a: TypedCallArg) -> Result<TypedCallArg> {
        Result::Ok(TypedCallArg {
            label: a.label,
            arg: Box::new(self.expr(*a.arg)?),
            is_vararg: a.is_vararg,
        })
    }

    pub fn typed_if(&mut self, i: TypedIf) -> Result<TypedIf> {
        let condition = Box::new(self.expr(*i.condition)?);
        let body = self.typed_block(i.body)?;
        let else_body = match i.else_body {
            Some(b) => Some(self.typed_block(b)?),
            None => None,
        };
        let type_ = i.type_;
        Result::Ok(TypedIf {
            condition,
            body,
            else_body,
            type_,
        })
    }

    pub fn typed_return(&mut self, r: TypedReturn) -> Result<TypedReturn> {
        let value = match r.value {
            Some(v) => Some(Box::new(self.expr(*v)?)),
            None => None,
        };
        Result::Ok(TypedReturn { value })
    }

    pub fn typed_type_cast(&mut self, t: TypedTypeCast) -> Result<TypedTypeCast> {
        Result::Ok(TypedTypeCast {
            target: Box::new(self.expr(*t.target)?),
            is_safe: t.is_safe,
            type_: Some(self.context.full_type_name(t.type_.unwrap())?),
        })
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Result<TypedStmt> {
        Result::Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(self.expr(e)?),
            TypedStmt::Decl(d) => TypedStmt::Decl(self.decl(d)?),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(self.assignment_stmt(a)?),
            TypedStmt::Loop(l) => TypedStmt::Loop(self.typed_loop_stmt(l)?),
        })
    }

    pub fn assignment_stmt(&mut self, a: TypedAssignmentStmt) -> Result<TypedAssignmentStmt> {
        Result::Ok(match a {
            TypedAssignmentStmt::Assignment(a) => {
                TypedAssignmentStmt::Assignment(self.typed_assignment(a)?)
            }
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(self.typed_assignment_and_operation(a)?)
            }
        })
    }

    pub fn typed_assignment(&mut self, a: TypedAssignment) -> Result<TypedAssignment> {
        Result::Ok(TypedAssignment {
            target: self.expr(a.target)?,
            value: self.expr(a.value)?,
        })
    }

    pub fn typed_assignment_and_operation(
        &mut self,
        a: TypedAssignmentAndOperation,
    ) -> Result<TypedAssignmentAndOperation> {
        Result::Ok(TypedAssignmentAndOperation {
            target: self.expr(a.target)?,
            operator: a.operator, // TODO
            value: self.expr(a.value)?,
        })
    }

    pub fn typed_loop_stmt(&mut self, l: TypedLoopStmt) -> Result<TypedLoopStmt> {
        Result::Ok(match l {
            TypedLoopStmt::While(w) => TypedLoopStmt::While(self.typed_while_loop_stmt(w)?),
            TypedLoopStmt::For(f) => TypedLoopStmt::For(self.typed_for_loop_stmt(f)?),
        })
    }

    pub fn typed_while_loop_stmt(&mut self, w: TypedWhileLoopStmt) -> Result<TypedWhileLoopStmt> {
        let TypedWhileLoopStmt { condition, block } = w;
        let condition = self.expr(condition)?;
        if !condition.type_().unwrap().is_boolean() {
            return Result::Err(ResolverError::from("while loop condition must be boolean"));
        };
        Result::Ok(TypedWhileLoopStmt {
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
        Result::Ok(TypedForStmt {
            values: values,
            iterator: self.expr(iterator)?,
            block: self.typed_block(block)?,
        })
    }
}
