use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::{TypedFunctionType, TypedType, TypedTypeParam};
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
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedVar {
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
    pub(crate) value: TypedExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct TypedFun {
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) arg_defs: Vec<TypedArgDef>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: TypedType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypedArgDef {
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
    pub(crate) type_: TypedType,
    pub(crate) args: Vec<TypedArgDef>,
    pub(crate) block: TypedBlock,
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
    pub(crate) body: TypedFunBody,
    pub(crate) return_type: TypedType,
    pub(crate) type_: TypedType,
}

impl TypedFun {
    pub fn type_(&self) -> TypedType {
        TypedType::Function(Box::new(TypedFunctionType {
            arguments: self.arg_defs.clone(),
            return_type: self.return_type.clone(),
        }))
    }
}
