use crate::constants;
use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedExtension, TypedFun, TypedFunBody, TypedMemberFunction,
    TypedProtocol, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedArray, TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedIf,
    TypedInstanceMember, TypedLiteral, TypedName, TypedPrefixUnaryOperator, TypedReturn,
    TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::{TypedFile, TypedSourceSet};
use crate::high_level_ir::typed_stmt::{
    TypedAssignmentAndOperator, TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt,
};
use crate::high_level_ir::typed_type::{
    TypedFunctionType, TypedPackage, TypedType, TypedValueType,
};
use core::result;
use std::collections::HashMap;
use std::error::Error;
use wiz_mir::builder::{BuilderError, FunBuilder, MLIRModule};
use wiz_mir::expr::{
    MLArray, MLBinOp, MLBinOpKind, MLBlock, MLCall, MLCallArg, MLExpr, MLIf, MLLiteral, MLMember,
    MLName, MLSubscript, MLTypeCast, MLUnaryOp, MLUnaryOpKind,
};
use wiz_mir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLAssignmentStmt, MLLoopStmt, MLReturn, MLStmt};

#[cfg(test)]
mod tests;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub fn hlir2mlir(
    target: TypedSourceSet,
    dependencies: &[MLFile],
    annotations: HashMap<String, TypedAnnotations>,
) -> Result<(MLFile, HashMap<String, TypedAnnotations>)> {
    let mut converter = HLIR2MLIR::new();
    converter.load_dependencies(dependencies)?;
    converter
        .context
        .declaration_annotations
        .extend(annotations);
    Ok((
        converter.convert_from_source_set(target),
        converter.context.declaration_annotations,
    ))
}

struct HLIR2MLIRContext {
    declaration_annotations: HashMap<String, TypedAnnotations>,
    structs: HashMap<MLValueType, MLStruct>,
    current_name_space: Vec<String>,
}

impl HLIR2MLIRContext {
    fn new() -> Self {
        Self {
            declaration_annotations: Default::default(),
            structs: Default::default(),
            current_name_space: vec![],
        }
    }

    pub(crate) fn set_declaration_annotations(&mut self, name: String, a: TypedAnnotations) {
        self.declaration_annotations.insert(name, a);
    }

    pub(crate) fn declaration_has_annotation(
        &self,
        declaration_name: &str,
        annotation: &str,
    ) -> bool {
        let an = self.declaration_annotations.get(declaration_name);
        an.map(|a| a.has_annotate(annotation))
            .unwrap_or_else(|| false)
    }

    pub(crate) fn get_struct(&self, typ: &MLValueType) -> &MLStruct {
        self.structs.get(typ).unwrap()
    }

    pub(crate) fn struct_has_field(&self, typ: &MLValueType, field_name: &str) -> bool {
        self.get_struct(typ)
            .fields
            .iter()
            .any(|f| f.name == *field_name)
    }

    pub(crate) fn add_struct(&mut self, typ: MLValueType, struct_: MLStruct) {
        self.structs.insert(typ, struct_);
    }

    pub(crate) fn push_name_space(&mut self, name: String) {
        self.current_name_space.push(name)
    }

    pub(crate) fn pop_name_space(&mut self) {
        self.current_name_space.pop();
    }
}

pub struct HLIR2MLIR {
    context: HLIR2MLIRContext,
    module: MLIRModule,
}

impl HLIR2MLIR {
    pub fn new() -> Self {
        HLIR2MLIR {
            context: HLIR2MLIRContext::new(),
            module: MLIRModule::new(),
        }
    }

    pub(crate) fn annotations(self) -> HashMap<String, TypedAnnotations> {
        self.context.declaration_annotations
    }

    pub fn load_dependencies(&mut self, dependencies: &[MLFile]) -> Result<()> {
        for dependency in dependencies {
            self.load_dependencies_file(dependency)?;
        }
        Ok(())
    }

    fn load_dependencies_file(&mut self, f: &MLFile) -> Result<()> {
        f.body
            .iter()
            .map(|d| self.load_dependencies_decl(d))
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn load_dependencies_decl(&mut self, d: &MLDecl) -> Result<()> {
        match d {
            MLDecl::Var(v) => self.load_dependencies_var(v),
            MLDecl::Fun(f) => self.load_dependencies_function(f),
            MLDecl::Struct(s) => self.load_dependencies_struct(s),
        }
    }

    fn load_dependencies_var(&mut self, v: &MLVar) -> Result<()> {
        self.module.add_global_var(v.clone());
        Ok(())
    }

    fn load_dependencies_struct(&mut self, s: &MLStruct) -> Result<()> {
        self.module.add_struct(s.clone());
        self.context
            .add_struct(MLValueType::Struct(s.name.clone()), s.clone());
        Ok(())
    }

    fn load_dependencies_function(&mut self, f: &MLFun) -> Result<()> {
        if f.body.is_none() {
            self.module._add_function(FunBuilder::from(f.clone()));
        };
        Ok(())
    }

    pub fn convert_from_source_set(&mut self, s: TypedSourceSet) -> MLFile {
        self.source_set(s).unwrap()
    }

    fn type_(&self, t: TypedType) -> MLType {
        match t {
            TypedType::Value(t) => MLType::Value(self.value_type(t)),
            TypedType::Function(f) => MLType::Function(self.function_type(*f)),
            _ => panic!("Invalid Type convert  {:?}", t),
        }
    }

    fn value_type(&self, t: TypedValueType) -> MLValueType {
        match t {
            TypedValueType::Value(t) => {
                let mut pkg = t.package.clone().into_resolved().names;
                if pkg.is_empty() {
                    match &*t.name {
                        constants::NOTING => MLValueType::Primitive(MLPrimitiveType::Noting),
                        constants::UNIT => MLValueType::Primitive(MLPrimitiveType::Unit),
                        constants::INT8 => MLValueType::Primitive(MLPrimitiveType::Int8),
                        constants::UINT8 => MLValueType::Primitive(MLPrimitiveType::UInt8),
                        constants::INT16 => MLValueType::Primitive(MLPrimitiveType::Int16),
                        constants::UINT16 => MLValueType::Primitive(MLPrimitiveType::UInt16),
                        constants::INT32 => MLValueType::Primitive(MLPrimitiveType::Int32),
                        constants::UINT32 => MLValueType::Primitive(MLPrimitiveType::UInt32),
                        constants::INT64 => MLValueType::Primitive(MLPrimitiveType::Int64),
                        constants::UINT64 => MLValueType::Primitive(MLPrimitiveType::UInt64),
                        constants::INT128 => MLValueType::Primitive(MLPrimitiveType::Int128),
                        constants::UINT128 => MLValueType::Primitive(MLPrimitiveType::UInt128),
                        constants::SIZE => MLValueType::Primitive(MLPrimitiveType::Size),
                        constants::USIZE => MLValueType::Primitive(MLPrimitiveType::USize),
                        constants::BOOL => MLValueType::Primitive(MLPrimitiveType::Bool),
                        constants::F32 => MLValueType::Primitive(MLPrimitiveType::Float),
                        constants::F64 => MLValueType::Primitive(MLPrimitiveType::Double),
                        constants::STRING => MLValueType::Primitive(MLPrimitiveType::String),
                        other => {
                            pkg.push(String::from(other));
                            MLValueType::Struct(pkg.join("::"))
                        }
                    }
                } else {
                    pkg.push(t.name);
                    MLValueType::Struct(pkg.join("::"))
                }
            }
            TypedValueType::Array(t, len) => {
                MLValueType::Array(Box::new(self.type_(*t).into_value_type()), len)
            }
            TypedValueType::Tuple(_) => {
                todo!()
            }
            TypedValueType::Pointer(t) => MLValueType::Pointer(Box::new(self.type_(*t))),
            TypedValueType::Reference(t) => MLValueType::Reference(Box::new(self.type_(*t))),
        }
    }

    fn function_type(&self, t: TypedFunctionType) -> MLFunctionType {
        MLFunctionType {
            arguments: t
                .arguments
                .into_iter()
                .map(|a| match self.type_(a.typ) {
                    MLType::Value(v) => v,
                    MLType::Function(f) => todo!("{:?}", f),
                })
                .collect(),
            return_type: match self.type_(t.return_type) {
                MLType::Value(v) => v,
                MLType::Function(f) => todo!("{:?}", f),
            },
        }
    }

    fn source_set(&mut self, s: TypedSourceSet) -> Result<MLFile> {
        let name = match s {
            TypedSourceSet::File(f) => {
                let name = f.name.clone();
                self.file(f)?;
                name
            }
            TypedSourceSet::Dir { name, mut items } => {
                self.context.push_name_space(name.clone());
                items.sort();
                let _: Vec<_> = items
                    .into_iter()
                    .map(|i| self.source_set(i))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .map(|i| i.body)
                    .flatten()
                    .collect();
                self.context.pop_name_space();
                name
            }
        };
        Ok(self.module.to_mlir_file(name))
    }

    fn file(&mut self, f: TypedFile) -> Result<()> {
        self.context.push_name_space(f.name.clone());
        for d in f.body.into_iter() {
            self.decl(d)?;
        }
        self.context.pop_name_space();
        Ok(())
    }

    fn stmt(&mut self, s: TypedStmt) -> Vec<MLStmt> {
        match s {
            TypedStmt::Expr(e) => vec![MLStmt::Expr(self.expr(e))],
            TypedStmt::Decl(d) => match d {
                TypedDecl::Var(v) => {
                    vec![MLStmt::Var(self.var(v))]
                }
                TypedDecl::Fun(_) => todo!("local function"),
                TypedDecl::Struct(_) => todo!("local struct"),
                TypedDecl::Class => {
                    todo!()
                }
                TypedDecl::Enum => {
                    todo!()
                }
                TypedDecl::Protocol(_) => todo!("local protocol"),
                TypedDecl::Extension(_) => todo!("local extension"),
            },
            TypedStmt::Assignment(a) => vec![MLStmt::Assignment(self.assignment(a))],
            TypedStmt::Loop(l) => vec![MLStmt::Loop(self.loop_stmt(l))],
        }
    }

    fn assignment(&mut self, a: TypedAssignmentStmt) -> MLAssignmentStmt {
        match a {
            TypedAssignmentStmt::Assignment(a) => MLAssignmentStmt {
                target: self.expr(a.target),
                value: self.expr(a.value),
            },
            TypedAssignmentStmt::AssignmentAndOperation(a) => {
                let target = self.expr(a.target.clone());
                let value = TypedExpr::BinOp(TypedBinOp {
                    left: Box::new(a.target.clone()),
                    operator: match a.operator {
                        TypedAssignmentAndOperator::Add => TypedBinaryOperator::Add,
                        TypedAssignmentAndOperator::Sub => TypedBinaryOperator::Sub,
                        TypedAssignmentAndOperator::Mul => TypedBinaryOperator::Mul,
                        TypedAssignmentAndOperator::Div => TypedBinaryOperator::Div,
                        TypedAssignmentAndOperator::Mod => TypedBinaryOperator::Mod,
                    },
                    right: Box::new(a.value),
                    type_: a.target.type_(),
                });
                MLAssignmentStmt {
                    target,
                    value: self.expr(value),
                }
            }
        }
    }

    fn loop_stmt(&mut self, l: TypedLoopStmt) -> MLLoopStmt {
        match l {
            TypedLoopStmt::While(w) => MLLoopStmt {
                condition: self.expr(w.condition),
                block: self.block(w.block),
            },
            TypedLoopStmt::For(_) => todo!(),
        }
    }

    fn decl(&mut self, d: TypedDecl) -> result::Result<(), BuilderError> {
        match d {
            TypedDecl::Var(v) => {
                let v = self.var(v);
                self.module.add_global_var(v);
            }
            TypedDecl::Fun(f) => {
                let f = FunBuilder::from(self.fun(f));
                self.module._add_function(f);
            }
            TypedDecl::Struct(s) => {
                let (st, fns) = self.struct_(s);
                self.module.add_struct(st);
                for f in fns {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
            TypedDecl::Class => todo!(),
            TypedDecl::Enum => todo!(),
            TypedDecl::Protocol(p) => {
                let functions = self.protocol(p);
                for f in functions {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
            TypedDecl::Extension(e) => {
                let functions = self.extension(e);
                for f in functions {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
        };
        Ok(())
    }

    fn var(&mut self, v: TypedVar) -> MLVar {
        let expr = self.expr(v.value);
        MLVar {
            is_mute: v.is_mut,
            name: v.name,
            type_: self.type_(v.type_.unwrap()),
            value: expr,
        }
    }

    fn fun(&mut self, f: TypedFun) -> MLFun {
        let TypedFun {
            annotations,
            package,
            modifiers,
            name,
            type_params,
            type_constraints,
            arg_defs,
            body,
            return_type,
        } = f;
        // TODO: use type_params
        let package_mangled_name = self.package_name_mangling(&package, &name);
        let mangled_name = if annotations.has_annotate("no_mangle") {
            name
        } else {
            let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&arg_defs);
            if fun_arg_label_type_mangled_name.is_empty() {
                package_mangled_name
            } else {
                package_mangled_name + "##" + &*fun_arg_label_type_mangled_name
            }
        };
        self.context
            .set_declaration_annotations(mangled_name.clone(), annotations);
        let args = arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        MLFun {
            modifiers,
            name: mangled_name,
            arg_defs: args,
            return_type: self.type_(return_type.unwrap()).into_value_type(),
            body: body.map(|b| self.fun_body(b)),
        }
    }

    fn struct_(&mut self, s: TypedStruct) -> (MLStruct, Vec<MLFun>) {
        let TypedStruct {
            annotations,
            package,
            name,
            type_params,
            initializers,
            stored_properties,
            computed_properties,
            member_functions,
        } = s;
        let struct_ = MLStruct {
            name: self.package_name_mangling(&package, &name),
            fields: stored_properties
                .into_iter()
                .map(|p| MLField {
                    name: p.name,
                    type_: self.type_(p.type_).into_value_type(),
                })
                .collect(),
        };
        let value_type = MLValueType::Struct(struct_.name.clone());
        self.context.add_struct(value_type.clone(), struct_.clone());

        let init: Vec<MLFun> = initializers
            .into_iter()
            .map(|i| {
                let type_ = MLType::Value(value_type.clone());
                let mut body = self.fun_body(i.body).body;
                body.insert(
                    0,
                    MLStmt::Var(MLVar {
                        is_mute: true,
                        name: String::from("self"),
                        value: MLExpr::Literal(MLLiteral::Struct {
                            type_: type_.clone().into_value_type(),
                        }),
                        type_: type_.clone(),
                    }),
                );
                body.push(MLStmt::Expr(MLExpr::Return(MLReturn {
                    value: Some(Box::new(MLExpr::Name(MLName {
                        name: String::from("self"),
                        type_: type_.clone(),
                    }))),
                })));
                MLFun {
                    modifiers: vec![],
                    name: self.package_name_mangling(&package, &name)
                        + "::init"
                        + &*if i.args.is_empty() {
                            String::new()
                        } else {
                            String::from("##") + &*self.fun_arg_label_type_name_mangling(&i.args)
                        },
                    arg_defs: i.args.into_iter().map(|a| self.arg_def(a)).collect(),
                    return_type: type_.into_value_type(),
                    body: Some(MLFunBody { body }),
                }
            })
            .collect();
        let members: Vec<MLFun> = member_functions
            .into_iter()
            .map(|mf| {
                let TypedMemberFunction {
                    name: fname,
                    arg_defs: args,
                    type_params,
                    body,
                    return_type,
                } = mf;
                let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&args);
                let args = args.into_iter().map(|a| self.arg_def(a)).collect();
                MLFun {
                    modifiers: vec![],
                    name: self.package_name_mangling(&package, &name)
                        + "::"
                        + &fname
                        + &*if fun_arg_label_type_mangled_name.is_empty() {
                            String::new()
                        } else {
                            String::from("##") + &*fun_arg_label_type_mangled_name
                        },
                    arg_defs: args,
                    return_type: self.type_(return_type.unwrap()).into_value_type(),
                    body: body.map(|body| self.fun_body(body)),
                }
            })
            .collect();
        (struct_, init.into_iter().chain(members).collect())
    }

    fn extension(&mut self, e: TypedExtension) -> Vec<MLFun> {
        let TypedExtension {
            annotations,
            name,
            protocol: type_params,
            computed_properties,
            member_functions,
        } = e;
        member_functions
            .into_iter()
            .map(|mf| {
                let TypedMemberFunction {
                    name: fname,
                    arg_defs: args,
                    type_params,
                    body,
                    return_type,
                } = mf;
                let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&args);
                let args = args.into_iter().map(|a| self.arg_def(a)).collect();
                MLFun {
                    modifiers: vec![],
                    name: self.package_name_mangling(&name.package(), &name.name())
                        + "::"
                        + &fname
                        + &*if fun_arg_label_type_mangled_name.is_empty() {
                            String::new()
                        } else {
                            String::from("##") + &*fun_arg_label_type_mangled_name
                        },
                    arg_defs: args,
                    return_type: self.type_(return_type.unwrap()).into_value_type(),
                    body: body.map(|body| self.fun_body(body)),
                }
            })
            .collect()
    }

    fn protocol(&mut self, p: TypedProtocol) -> Vec<MLFun> {
        vec![]
    }

    fn expr(&mut self, e: TypedExpr) -> MLExpr {
        match e {
            TypedExpr::Name(name) => MLExpr::Name(self.name(name)),
            TypedExpr::Literal(l) => MLExpr::Literal(self.literal(l)),
            TypedExpr::BinOp(b) => MLExpr::PrimitiveBinOp(self.binop(b)),
            TypedExpr::UnaryOp(u) => MLExpr::PrimitiveUnaryOp(self.unary_op(u)),
            TypedExpr::Subscript(s) => self.subscript(s),
            TypedExpr::Member(m) => self.member(m),
            TypedExpr::Array(a) => MLExpr::Array(self.array(a)),
            TypedExpr::Tuple => todo!(),
            TypedExpr::Dict => todo!(),
            TypedExpr::StringBuilder => todo!(),
            TypedExpr::Call(c) => MLExpr::Call(self.call(c)),
            TypedExpr::If(i) => MLExpr::If(self.if_expr(i)),
            TypedExpr::When => todo!(),
            TypedExpr::Lambda(l) => todo!(),
            TypedExpr::Return(r) => MLExpr::Return(self.return_expr(r)),
            TypedExpr::TypeCast(t) => MLExpr::PrimitiveTypeCast(self.type_cast(t)),
        }
    }

    fn name(&self, n: TypedName) -> MLName {
        let mangled_name = if self
            .context
            .declaration_has_annotation(&n.name, "no_mangle")
        {
            n.name
        } else {
            self.package_name_mangling(&n.package, &*n.name)
        };
        MLName {
            name: mangled_name,
            type_: self.type_(n.type_.unwrap()),
        }
    }

    fn literal(&self, l: TypedLiteral) -> MLLiteral {
        match l {
            TypedLiteral::Integer { value, type_ } => MLLiteral::Integer {
                value,
                type_: self.type_(type_.unwrap()).into_value_type(),
            },
            TypedLiteral::FloatingPoint { value, type_ } => MLLiteral::FloatingPoint {
                value,
                type_: self.type_(type_.unwrap()).into_value_type(),
            },
            TypedLiteral::String { value, type_ } => MLLiteral::String {
                value,
                type_: self.type_(type_.unwrap()).into_value_type(),
            },
            TypedLiteral::Boolean { value, type_ } => MLLiteral::Boolean {
                value,
                type_: self.type_(type_.unwrap()).into_value_type(),
            },
            TypedLiteral::NullLiteral { type_ } => MLLiteral::Null {
                type_: self.type_(type_.unwrap()).into_value_type(),
            },
        }
    }

    fn binop(&mut self, b: TypedBinOp) -> MLBinOp {
        let TypedBinOp {
            left,
            operator: kind,
            right,
            type_,
        } = b;
        MLBinOp {
            left: Box::new(self.expr(*left)),
            kind: match kind {
                TypedBinaryOperator::Add => MLBinOpKind::Plus,
                TypedBinaryOperator::Sub => MLBinOpKind::Minus,
                TypedBinaryOperator::Mul => MLBinOpKind::Mul,
                TypedBinaryOperator::Div => MLBinOpKind::Div,
                TypedBinaryOperator::Mod => MLBinOpKind::Mod,
                TypedBinaryOperator::Equal => MLBinOpKind::Equal,
                TypedBinaryOperator::GrateThanEqual => MLBinOpKind::GrateThanEqual,
                TypedBinaryOperator::GrateThan => MLBinOpKind::GrateThan,
                TypedBinaryOperator::LessThanEqual => MLBinOpKind::LessThanEqual,
                TypedBinaryOperator::LessThan => MLBinOpKind::LessThan,
                TypedBinaryOperator::NotEqual => MLBinOpKind::NotEqual,
                TypedBinaryOperator::InfixFunctionCall(call) => {
                    todo!("infix function call {:?}", call)
                }
            },
            right: Box::new(self.expr(*right)),
            type_: self.type_(type_.unwrap()).into_value_type(),
        }
    }

    fn unary_op(&mut self, u: TypedUnaryOp) -> MLUnaryOp {
        match u {
            TypedUnaryOp::Prefix(p) => {
                let target = self.expr(*p.target);
                MLUnaryOp {
                    kind: match p.operator {
                        TypedPrefixUnaryOperator::Positive => MLUnaryOpKind::Positive,
                        TypedPrefixUnaryOperator::Negative => MLUnaryOpKind::Negative,
                        TypedPrefixUnaryOperator::Dereference => MLUnaryOpKind::DeRef,
                        TypedPrefixUnaryOperator::Reference => MLUnaryOpKind::Ref,
                        TypedPrefixUnaryOperator::Not => MLUnaryOpKind::Not,
                    },
                    type_: self.type_(p.type_.unwrap()).into_value_type(),
                    target: Box::new(target),
                }
            }
            TypedUnaryOp::Postfix(p) => {
                todo!()
            }
        }
    }

    fn subscript(&mut self, s: TypedSubscript) -> MLExpr {
        let t = s.target.type_().unwrap();
        if t.is_pointer_type() && s.indexes.len() == 1 {
            match t {
                TypedType::Value(v) => match v {
                    TypedValueType::Value(v) => {
                        if v.type_args.as_ref().unwrap()[0].is_primitive() {
                            MLExpr::PrimitiveSubscript(MLSubscript {
                                target: Box::new(self.expr(*s.target)),
                                index: Box::new(self.expr(s.indexes[0].clone())),
                                type_: self
                                    .type_(v.type_args.unwrap()[0].clone())
                                    .into_value_type(),
                            })
                        } else {
                            self.subscript_for_user_defined(s)
                        }
                    }
                    TypedValueType::Array(_, _) => {
                        todo!()
                    }
                    TypedValueType::Tuple(_) => {
                        todo!()
                    }
                    TypedValueType::Pointer(p) => MLExpr::PrimitiveSubscript(MLSubscript {
                        target: Box::new(self.expr(*s.target)),
                        index: Box::new(self.expr(s.indexes[0].clone())),
                        type_: self.type_(*p).into_value_type(),
                    }),
                    TypedValueType::Reference(_) => {
                        todo!()
                    }
                },
                t => panic!("function pointer detected. {:?}", t),
            }
        } else if t.is_string() || t.is_string_ref() {
            MLExpr::PrimitiveSubscript(MLSubscript {
                target: Box::new(self.expr(*s.target)),
                index: Box::new(self.expr(s.indexes[0].clone())),
                type_: MLValueType::Primitive(MLPrimitiveType::UInt8),
            })
        } else if t.is_array_type() {
            MLExpr::PrimitiveSubscript(MLSubscript {
                target: Box::new(self.expr(*s.target)),
                index: Box::new(self.expr(s.indexes[0].clone())),
                type_: match t {
                    TypedType::Value(TypedValueType::Array(e, _)) => {
                        self.type_(*e).into_value_type()
                    }
                    _ => panic!("Never execution branch executed!!"),
                },
            })
        } else {
            self.subscript_for_user_defined(s)
        }
    }

    fn subscript_for_user_defined(&mut self, s: TypedSubscript) -> MLExpr {
        let target = self.expr(*s.target);
        match target {
            MLExpr::Name(target) => MLExpr::Call(MLCall {
                target,
                args: s
                    .indexes
                    .into_iter()
                    .map(|i| MLCallArg { arg: self.expr(i) })
                    .collect(),
                type_: self.type_(s.type_.unwrap()).into_value_type(),
            }),
            a => panic!("{:?}", a),
        }
    }

    fn member(&mut self, m: TypedInstanceMember) -> MLExpr {
        let TypedInstanceMember {
            target,
            name,
            is_safe,
            type_,
        } = m;
        let target = self.expr(*target);
        let type_ = self.type_(type_.unwrap());
        MLExpr::Member(MLMember {
            target: Box::new(target),
            name,
            type_,
        })
    }

    fn array(&mut self, a: TypedArray) -> MLArray {
        MLArray {
            elements: a.elements.into_iter().map(|e| self.expr(e)).collect(),
            type_: self.type_(a.type_.unwrap()).into_value_type(),
        }
    }

    fn call(&mut self, c: TypedCall) -> MLCall {
        let TypedCall {
            target,
            mut args,
            type_,
        } = c;
        let target = match *target {
            TypedExpr::Member(m) => {
                let TypedInstanceMember {
                    target,
                    name,
                    is_safe,
                    type_,
                } = m;
                match target.type_().unwrap() {
                    TypedType::Self_ => {
                        todo!()
                    }
                    TypedType::Value(v) => {
                        let target_type = self.value_type(v);
                        let is_stored = self.context.struct_has_field(&target_type, &name);
                        if is_stored {
                            let target = self.expr(*target);
                            let type_ = self.type_(type_.unwrap());
                            MLExpr::Member(MLMember {
                                target: Box::new(target),
                                name,
                                type_,
                            })
                        } else {
                            args.insert(
                                0,
                                TypedCallArg {
                                    label: None,
                                    arg: target,
                                    is_vararg: false,
                                },
                            );
                            MLExpr::Name(MLName {
                                name: target_type.name() + "::" + &*name,
                                type_: self.type_(type_.unwrap()),
                            })
                        }
                    }
                    TypedType::Function(_) => {
                        todo!()
                    }
                    TypedType::Type(t) => match *t {
                        TypedType::Self_ => {
                            todo!()
                        }
                        TypedType::Value(t) => match t {
                            TypedValueType::Value(t) => {
                                let type_ = self.type_(type_.unwrap());
                                MLExpr::Name(MLName {
                                    name: self.package_name_mangling(&t.package, &*t.name)
                                        + "::"
                                        + &*name,
                                    type_,
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
                            TypedValueType::Reference(_) => {
                                todo!()
                            }
                        },
                        TypedType::Function(_) => {
                            todo!()
                        }
                        TypedType::Type(_) => {
                            todo!()
                        }
                    },
                }
            }
            t => self.expr(t),
        };
        let target = match target {
            MLExpr::Name(MLName { name, type_ }) => {
                let fun_arg_label_type_mangled_name =
                    if self.context.declaration_has_annotation(&name, "no_mangle") {
                        name
                    } else {
                        if args.is_empty() {
                            name
                        } else {
                            name + "##"
                                + &*self.fun_arg_label_type_name_mangling(
                                    &args
                                        .iter()
                                        .map(|a| TypedArgDef {
                                            label: match &a.label {
                                                None => "_".to_string(),
                                                Some(l) => l.to_string(),
                                            },
                                            name: "".to_string(),
                                            type_: a.arg.type_().unwrap(),
                                        })
                                        .collect(),
                                )
                        }
                    };
                MLName {
                    name: fun_arg_label_type_mangled_name,
                    type_,
                }
            }
            a => panic!("{:?}", a),
        };
        MLCall {
            target,
            args: args
                .into_iter()
                .map(|a| MLCallArg {
                    arg: self.expr(*a.arg),
                })
                .collect(),
            type_: self.type_(type_.unwrap()).into_value_type(),
        }
    }

    fn if_expr(&mut self, i: TypedIf) -> MLIf {
        MLIf {
            condition: Box::new(self.expr(*i.condition)),
            body: self.block(i.body),
            else_body: i.else_body.map(|b| self.block(b)),
            type_: self.type_(i.type_.unwrap()).into_value_type(),
        }
    }

    fn return_expr(&mut self, r: TypedReturn) -> MLReturn {
        MLReturn {
            value: r.value.map(|v| Box::new(self.expr(*v))),
        }
    }

    fn type_cast(&mut self, t: TypedTypeCast) -> MLTypeCast {
        MLTypeCast {
            target: Box::new(self.expr(*t.target)),
            type_: self.type_(t.type_.unwrap()).into_value_type(),
        }
    }

    fn arg_def(&self, e: TypedArgDef) -> MLArgDef {
        MLArgDef {
            name: e.name,
            type_: self.type_(e.type_).into_value_type(),
        }
    }

    fn fun_body(&mut self, b: TypedFunBody) -> MLFunBody {
        match b {
            TypedFunBody::Expr(e) => MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn::new(Some(
                    self.expr(e),
                ))))],
            },
            TypedFunBody::Block(b) => MLFunBody {
                body: self.block(b).body,
            },
        }
    }

    fn block(&mut self, b: TypedBlock) -> MLBlock {
        MLBlock {
            body: b.body.into_iter().map(|s| self.stmt(s)).flatten().collect(),
        }
    }

    fn package_name_mangling(&self, package: &TypedPackage, name: &str) -> String {
        if let TypedPackage::Resolved(pkg) = package {
            if pkg.is_global() || name == "main" {
                String::from(name)
            } else {
                pkg.to_string() + "::" + name
            }
        } else {
            panic!("pkg name mangling failed => {:?}, {}", package, name)
        }
    }

    fn fun_arg_label_type_name_mangling(&self, args: &Vec<TypedArgDef>) -> String {
        args.iter()
            .map(|arg| format!("{}#{}", arg.label, arg.type_.to_string()))
            .collect::<Vec<String>>()
            .join("##")
    }
}
