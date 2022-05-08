use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_expr::TypedExpr;
use crate::high_level_ir::typed_stmt::TypedBlock;
use crate::high_level_ir::typed_type::{
    TypedArgType, TypedFunctionType, TypedPackage, TypedType, TypedTypeParam,
};
use crate::high_level_ir::typed_type_constraint::TypedTypeConstraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedDeclKind {
    Var(TypedVar),
    Fun(TypedFun),
    Struct(TypedStruct),
    Class,
    Enum,
    Protocol(TypedProtocol),
    Extension(TypedExtension),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedVar {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: TypedPackage,
    pub(crate) is_mut: bool,
    pub(crate) name: String,
    pub(crate) type_: Option<TypedType>,
    pub(crate) value: TypedExpr,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedFun {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: TypedPackage,
    pub(crate) modifiers: Vec<String>,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) type_constraints: Option<Vec<TypedTypeConstraint>>,
    pub(crate) arg_defs: Vec<TypedArgDef>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct TypedArgDef {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

impl TypedArgDef {
    pub(crate) fn to_arg_type(&self) -> TypedArgType {
        TypedArgType {
            label: self.label.clone(),
            typ: self.type_.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedFunBody {
    Expr(TypedExpr),
    Block(TypedBlock),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStruct {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: TypedPackage,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) initializers: Vec<TypedInitializer>,
    pub(crate) stored_properties: Vec<TypedStoredProperty>,
    pub(crate) computed_properties: Vec<TypedComputedProperty>,
    pub(crate) member_functions: Vec<TypedMemberFunction>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedInitializer {
    pub(crate) args: Vec<TypedArgDef>,
    pub(crate) body: TypedFunBody,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedStoredProperty {
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedComputedProperty {
    pub(crate) name: String,
    pub(crate) type_: TypedType,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedMemberFunction {
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) arg_defs: Vec<TypedArgDef>,
    pub(crate) body: Option<TypedFunBody>,
    pub(crate) return_type: Option<TypedType>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedExtension {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) name: TypedType,
    pub(crate) protocol: Option<TypedType>,
    pub(crate) computed_properties: Vec<TypedComputedProperty>,
    pub(crate) member_functions: Vec<TypedMemberFunction>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypedProtocol {
    pub(crate) annotations: TypedAnnotations,
    pub(crate) package: TypedPackage,
    pub(crate) name: String,
    pub(crate) type_params: Option<Vec<TypedTypeParam>>,
    pub(crate) computed_properties: Vec<TypedComputedProperty>,
    pub(crate) member_functions: Vec<TypedMemberFunction>,
}

impl TypedFun {
    pub fn type_(&self) -> Option<TypedType> {
        self.return_type.clone().map(|return_type| {
            TypedType::Function(Box::new(TypedFunctionType {
                arguments: self.arg_defs.iter().map(|a| a.to_arg_type()).collect(),
                return_type,
            }))
        })
    }

    pub fn is_generic(&self) -> bool {
        self.type_params.is_some()
    }
}

impl TypedMemberFunction {
    pub(crate) fn type_(&self) -> Option<TypedType> {
        match &self.return_type {
            Some(return_type) => Some(TypedType::Function(Box::new(TypedFunctionType {
                arguments: self.arg_defs.iter().map(|a| a.to_arg_type()).collect(),
                return_type: return_type.clone(),
            }))),
            None => None,
        }
    }
}
