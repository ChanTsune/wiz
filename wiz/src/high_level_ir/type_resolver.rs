pub mod context;
pub mod error;
pub mod result;

use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::type_resolver::context::{ResolverContext, ResolverStruct};
use crate::high_level_ir::type_resolver::error::ResolverError;
use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::typed_decl::{TypedDecl, TypedFun, TypedFunBody, TypedStruct, TypedVar};
use crate::high_level_ir::typed_expr::{TypedExpr, TypedSubscript};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{TypedBlock, TypedStmt};
use crate::high_level_ir::typed_type::TypedType;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct TypeResolver {
    context: ResolverContext,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self {
            context: ResolverContext::new(),
        }
    }

    pub fn detect_type(&mut self, f: TypedFile) -> Result<()> {
        self.context.push_name_space(f.name);
        for d in f.body {
            match d {
                TypedDecl::Struct(s) => {
                    let ns = self
                        .context
                        .get_current_namespace_mut()
                        .ok_or(ResolverError::from("Context NameSpace Error"))?;
                    ns.types.insert(s.name, ResolverStruct::new());
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

    pub fn preload_file(&mut self, f: TypedFile) -> Result<()> {
        self.context.push_name_space(f.name);
        for d in f.body {
            self.preload_decl(d)?;
        }
        self.context.pop_name_space();
        Result::Ok(())
    }

    fn preload_decl(&mut self, d: TypedDecl) -> Result<()> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.typed_var(v)?;
                let namespace = self
                    .context
                    .get_current_namespace_mut()
                    .ok_or(ResolverError::from("NameSpace not exist"))?;
                namespace.values.insert(
                    v.name,
                    v.type_
                        .ok_or(ResolverError::from("Cannot resolve variable type"))?,
                );
            }
            TypedDecl::Fun(_) => {}
            TypedDecl::Struct(_) => {}
            TypedDecl::Class => {}
            TypedDecl::Enum => {}
            TypedDecl::Protocol => {}
            TypedDecl::Extension => {}
        }
        Result::Ok(())
    }

    pub fn file(&self, f: TypedFile) -> Result<TypedFile> {
        Result::Ok(TypedFile {
            name: f.name,
            body: f
                .body
                .into_iter()
                .map(|s| self.decl(s))
                .collect::<Result<Vec<TypedDecl>>>()?,
        })
    }

    pub fn decl(&self, d: TypedDecl) -> Result<TypedDecl> {
        Result::Ok(match d {
            TypedDecl::Var(v) => TypedDecl::Var(self.typed_var(v)?),
            TypedDecl::Fun(f) => TypedDecl::Fun(self.typed_fun(f)?),
            TypedDecl::Struct(s) => TypedDecl::Struct(self.typed_struct(s)?),
            TypedDecl::Class => TypedDecl::Class,
            TypedDecl::Enum => TypedDecl::Enum,
            TypedDecl::Protocol => TypedDecl::Protocol,
            TypedDecl::Extension => TypedDecl::Extension,
        })
    }

    pub fn typed_var(&self, t: TypedVar) -> Result<TypedVar> {
        Result::Ok(TypedVar {
            is_mut: t.is_mut,
            name: t.name,
            type_: t.type_,
            value: self.expr(t.value)?,
        })
    }

    pub fn typed_fun(&self, f: TypedFun) -> Result<TypedFun> {
        Result::Ok(TypedFun {
            modifiers: f.modifiers,
            name: f.name,
            type_params: f.type_params, // TODO
            arg_defs: f.arg_defs,       // TODO
            body: match f.body {
                Some(b) => Some(self.typed_fun_body(b)?),
                None => None,
            },
            return_type: f.return_type, // TODO
        })
    }

    pub fn typed_struct(&self, s: TypedStruct) -> Result<TypedStruct> {
        Result::Ok(TypedStruct {
            name: s.name,
            type_params: s.type_params,
            init: s.init,                               // TODO
            stored_properties: s.stored_properties,     // TODO
            computed_properties: s.computed_properties, // TODO
            member_functions: s.member_functions,       // TODO
            static_function: s.static_function,         // TODO
        })
    }

    fn typed_fun_body(&self, b: TypedFunBody) -> Result<TypedFunBody> {
        Result::Ok(match b {
            TypedFunBody::Expr(e) => TypedFunBody::Expr(self.expr(e)?),
            TypedFunBody::Block(b) => TypedFunBody::Block(self.typed_block(b)?),
        })
    }

    fn typed_block(&self, b: TypedBlock) -> Result<TypedBlock> {
        Result::Ok(TypedBlock {
            body: b
                .body
                .into_iter()
                .map(|s| self.stmt(s))
                .collect::<Result<Vec<TypedStmt>>>()?,
        })
    }

    pub fn expr(&self, e: TypedExpr) -> Result<TypedExpr> {
        Result::Ok(match e {
            TypedExpr::Name(n) => TypedExpr::Name(n),
            TypedExpr::Literal(l) => TypedExpr::Literal(l),
            TypedExpr::BinOp(b) => TypedExpr::BinOp(b),
            TypedExpr::UnaryOp(u) => TypedExpr::UnaryOp(u),
            TypedExpr::Subscript(s) => TypedExpr::Subscript(self.typed_subscript(s)?),
            TypedExpr::Member(m) => TypedExpr::Member(m),
            TypedExpr::StaticMember(s) => TypedExpr::StaticMember(s),
            TypedExpr::List => TypedExpr::List,
            TypedExpr::Tuple => TypedExpr::Tuple,
            TypedExpr::Dict => TypedExpr::Dict,
            TypedExpr::StringBuilder => TypedExpr::StringBuilder,
            TypedExpr::Call(c) => TypedExpr::Call(c),
            TypedExpr::If(i) => TypedExpr::If(i),
            TypedExpr::When => TypedExpr::When,
            TypedExpr::Lambda => TypedExpr::Lambda,
            TypedExpr::Return(r) => TypedExpr::Return(r),
            TypedExpr::TypeCast => TypedExpr::TypeCast,
            TypedExpr::Type(t) => TypedExpr::Type(t),
        })
    }

    pub fn typed_subscript(&self, s: TypedSubscript) -> Result<TypedSubscript> {
        let target = self.expr(*s.target)?;
        if let TypedType::Value(v) = target.type_().unwrap() {
            if v.name == UNSAFE_POINTER {
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

    pub fn stmt(&self, s: TypedStmt) -> Result<TypedStmt> {
        Result::Ok(match s {
            TypedStmt::Expr(e) => TypedStmt::Expr(self.expr(e)?),
            TypedStmt::Decl(d) => TypedStmt::Decl(self.decl(d)?),
            TypedStmt::Assignment(a) => TypedStmt::Assignment(a),
            TypedStmt::Loop(l) => TypedStmt::Loop(l),
        })
    }
}
