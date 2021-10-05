use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedTypeParam};
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedDecl {
    Var(TypedVar),
    Fun(TypedFun),
    Struct(TypedStruct),
    Class,
    Enum,
    Protocol,
    Extension,
    Use(TypedUse),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedVar {
    pub(crate) package: Option<Package>,
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
    pub(crate) value: TypedExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedFun {
    pub(crate) package: Option<Package>,
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) arg_defs: Vec<TypedArgDef>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub enum TypedArgDef {
    Value(TypedValueArgDef),
    Self_(Option<TypedType>),
    RefSelf(Option<TypedType>),
}

impl TypedArgDef {
    pub(crate) fn label(&self) -> String {
        match self {
            TypedArgDef::Value(v) => v.name.clone(),
            TypedArgDef::Self_(_) | TypedArgDef::RefSelf(_) => String::from("_"),
        }
    }
    pub(crate) fn name(&self) -> String {
        match self {
            TypedArgDef::Value(v) => v.name.clone(),
            TypedArgDef::Self_(_) | TypedArgDef::RefSelf(_) => String::from("self"),
        }
    }
    pub(crate) fn type_(&self) -> Option<TypedType> {
        match self {
            TypedArgDef::Value(v) => Some(v.type_.clone()),
            TypedArgDef::Self_(s) | TypedArgDef::RefSelf(s) => s.clone(),
        }
    }
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedValueArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum TypedFunBody {
    Expr(TypedExpr),
    Block(TypedBlock),
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedStruct {
    pub(crate) package: Option<Package>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) init: Vec<TypedInitializer>,
    pub(crate) stored_properties: Vec<TypedStoredProperty>,
    pub(crate) computed_properties: Vec<TypedComputedProperty>,
    pub(crate) member_functions: Vec<TypedMemberFunction>,
    pub(crate) static_function: Vec<TypedFun>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedInitializer {
    pub(crate) args: Vec<TypedArgDef>,
    pub(crate) body: TypedFunBody,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedStoredProperty {
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedComputedProperty {
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedMemberFunction {
    pub(crate) name: String,
    pub(crate) args: Vec<TypedArgDef>,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: Option<TypedType>,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedUse {
    pub(crate) package: Package,
    pub(crate) alias: Option<String>,
}

impl TypedFun {
    pub fn type_(&self) -> Option<TypedType> {
        self.return_type.clone().map(|return_type| {
            TypedType::Function(Box::new(TypedFunctionType {
                arguments: self.arg_defs.clone(),
                return_type,
            }))
        })
    }
}

impl TypedMemberFunction {
    pub(crate) fn type_(&self) -> Option<TypedType> {
        match &self.return_type {
            Some(return_type) => Some(TypedType::Function(Box::new(TypedFunctionType {
                arguments: self.args.clone(),
                return_type: return_type.clone(),
            }))),
            None => None,
        }
    }
}
