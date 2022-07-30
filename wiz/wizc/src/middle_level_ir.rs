use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::declaration::{DeclarationItem, DeclarationItemKind};
use crate::middle_level_ir::context::HLIR2MLIRContext;
use crate::result::Result;
use std::collections::HashMap;
use wiz_constants as constants;
use wiz_constants::annotation::{BUILTIN, ENTRY, NO_MANGLE, TEST};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_decl::{
    TypedArgDef, TypedDecl, TypedDeclKind, TypedExtension, TypedFun, TypedFunBody, TypedProtocol,
    TypedStruct, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedArray, TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedExprKind,
    TypedIf, TypedInstanceMember, TypedLiteralKind, TypedName, TypedPrefixUnaryOperator,
    TypedReturn, TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use wiz_hir::typed_file::{TypedFile, TypedSourceSet};
use wiz_hir::typed_stmt::{
    TypedAssignmentAndOperator, TypedAssignmentStmt, TypedBlock, TypedLoopStmt, TypedStmt,
};
use wiz_hir::typed_type::{
    Package, TypedFunctionType, TypedPackage, TypedType, TypedTypeParam, TypedValueType,
};
use wiz_mir::builder::{FunBuilder, MLIRModule};
use wiz_mir::expr::{
    MLArray, MLBinOp, MLBinOpKind, MLBlock, MLCall, MLCallArg, MLExpr, MLIf, MLLiteral,
    MLLiteralKind, MLMember, MLName, MLSubscript, MLTypeCast, MLUnaryOp, MLUnaryOpKind,
};
use wiz_mir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLAssignmentStmt, MLLoopStmt, MLReturn, MLStmt};
use wizc_cli::{BuildType, Config, ConfigExt};

mod context;
#[cfg(test)]
mod tests;

pub fn hlir2mlir<'a, 'c>(
    target: TypedSourceSet,
    dependencies: &'a [MLFile],
    arena: &'a ResolverArena,
    config: &'a Config<'c>,
    generate_test_harness_if_needed: bool,
) -> Result<MLFile> {
    let mut converter = HLIR2MLIR::new(config, arena);
    converter.load_dependencies(dependencies)?;
    Ok(converter.convert_from_source_set(target, generate_test_harness_if_needed))
}

#[derive(Debug)]
pub struct HLIR2MLIR<'a, 'c> {
    config: &'a Config<'c>,
    arena: &'a ResolverArena,
    context: HLIR2MLIRContext,
    module: MLIRModule,
    tests: Vec<MLFun>,
}

impl<'a, 'c> HLIR2MLIR<'a, 'c> {
    pub fn new(config: &'a Config<'c>, arena: &'a ResolverArena) -> Self {
        Self {
            config,
            arena,
            context: Default::default(),
            module: Default::default(),
            tests: Default::default(),
        }
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
            .try_for_each(|d| self.load_dependencies_decl(d))
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

    pub fn convert_from_source_set(
        &mut self,
        s: TypedSourceSet,
        generate_test_harness_if_needed: bool,
    ) -> MLFile {
        let name = s.name().to_string();
        self.source_set(s).unwrap();
        if generate_test_harness_if_needed && BuildType::Test == self.config.type_() {
            let test_harness = self.generate_test_harness();
            self.module._add_function(FunBuilder::from(test_harness));
            for test in self.tests.clone() {
                self.module._add_function(FunBuilder::from(test));
            }
        };
        self.module.to_mlir_file(name)
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

    fn source_set(&mut self, s: TypedSourceSet) -> Result<()> {
        match s {
            TypedSourceSet::File(f) => self.file(f),
            TypedSourceSet::Dir { items, .. } => {
                items.into_iter().try_for_each(|i| self.source_set(i))
            }
        }
    }

    fn file(&mut self, f: TypedFile) -> Result<()> {
        f.body.into_iter().try_for_each(|d| self.decl(d))
    }

    fn stmt(&mut self, s: TypedStmt) -> Vec<MLStmt> {
        match s {
            TypedStmt::Expr(e) => vec![MLStmt::Expr(self.expr(e))],
            TypedStmt::Decl(d) => match d.kind {
                TypedDeclKind::Var(v) => {
                    vec![MLStmt::Var(self.var(v))]
                }
                TypedDeclKind::Fun(_) => todo!("local function"),
                TypedDeclKind::Struct(_) => todo!("local struct"),
                TypedDeclKind::Class => {
                    todo!()
                }
                TypedDeclKind::Enum => {
                    todo!()
                }
                TypedDeclKind::Protocol(_) => todo!("local protocol"),
                TypedDeclKind::Extension(_) => todo!("local extension"),
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
                let value = TypedExpr::new(
                    TypedExprKind::BinOp(TypedBinOp {
                        left: Box::new(a.target.clone()),
                        operator: match a.operator {
                            TypedAssignmentAndOperator::Add => TypedBinaryOperator::Add,
                            TypedAssignmentAndOperator::Sub => TypedBinaryOperator::Sub,
                            TypedAssignmentAndOperator::Mul => TypedBinaryOperator::Mul,
                            TypedAssignmentAndOperator::Div => TypedBinaryOperator::Div,
                            TypedAssignmentAndOperator::Mod => TypedBinaryOperator::Mod,
                        },
                        right: Box::new(a.value),
                    }),
                    a.target.ty,
                );
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

    fn decl(&mut self, d: TypedDecl) -> Result<()> {
        let TypedDecl {
            annotations,
            package,
            modifiers,
            kind,
        } = d;
        match kind {
            TypedDeclKind::Var(v) => {
                let v = self.global_var(v, &package);
                self.module.add_global_var(v);
            }
            TypedDeclKind::Fun(f) => {
                if BuildType::Test == self.config.type_() && annotations.has_annotate(TEST) {
                    if !f.is_generic() {
                        let f = self.fun(f, annotations, package, None);
                        self.tests.push(f);
                    }
                } else {
                    if !f.is_generic() {
                        let f = FunBuilder::from(self.fun(f, annotations, package, None));
                        self.module._add_function(f);
                    }
                }
            }
            TypedDeclKind::Struct(s) => {
                let (st, fns) = self.struct_(s, package);
                self.module.add_struct(st);
                for f in fns {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
            TypedDeclKind::Class => todo!(),
            TypedDeclKind::Enum => todo!(),
            TypedDeclKind::Protocol(p) => {
                let functions = self.protocol(p);
                for f in functions {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
            TypedDeclKind::Extension(e) => {
                let functions = self.extension(e);
                for f in functions {
                    self.module._add_function(FunBuilder::from(f));
                }
            }
        };
        Ok(())
    }

    fn global_var(&mut self, v: TypedVar, package: &Package) -> MLVar {
        let expr = self.expr(v.value);
        MLVar {
            is_mute: v.is_mut,
            name: self.package_name_mangling_(package, &v.name),
            type_: self.type_(v.type_.unwrap()),
            value: expr,
        }
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

    fn fun(
        &mut self,
        f: TypedFun,
        annotations: TypedAnnotations,
        package: Package,
        type_arguments: Option<HashMap<TypedTypeParam, TypedType>>,
    ) -> MLFun {
        let TypedFun {
            name,
            type_params: _,
            type_constraints: _,
            arg_defs,
            body,
            return_type,
        } = f;
        let mangled_name = if annotations.has_annotate(NO_MANGLE) {
            name
        } else if annotations.has_annotate(ENTRY) && self.config.type_() == BuildType::Binary {
            String::from("main")
        } else {
            let package_mangled_name = self.package_name_mangling_(&package, &name);
            let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&arg_defs);
            if fun_arg_label_type_mangled_name.is_empty() {
                package_mangled_name
            } else {
                package_mangled_name + "##" + &*fun_arg_label_type_mangled_name
            }
        };
        let args = arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        MLFun {
            name: mangled_name,
            arg_defs: args,
            return_type: self.type_(return_type.unwrap()).into_value_type(),
            body: body.map(|b| self.fun_body(b, type_arguments)),
        }
    }

    fn struct_(&mut self, s: TypedStruct, package: Package) -> (MLStruct, Vec<MLFun>) {
        let TypedStruct {
            name,
            type_params,
            stored_properties,
            computed_properties,
            member_functions,
        } = s;
        let struct_ = MLStruct {
            name: self.package_name_mangling_(&package, &name),
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

        let members: Vec<MLFun> = member_functions
            .into_iter()
            .map(|mf| {
                let TypedFun {
                    name: fname,
                    type_constraints,
                    arg_defs: args,
                    type_params,
                    body,
                    return_type,
                } = mf;
                let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&args);
                let args = args.into_iter().map(|a| self.arg_def(a)).collect();
                MLFun {
                    name: self.package_name_mangling_(&package, &name)
                        + "::"
                        + &fname
                        + &*if fun_arg_label_type_mangled_name.is_empty() {
                            String::new()
                        } else {
                            String::from("##") + &*fun_arg_label_type_mangled_name
                        },
                    arg_defs: args,
                    return_type: self.type_(return_type.unwrap()).into_value_type(),
                    body: body.map(|body| self.fun_body(body, None)),
                }
            })
            .collect();
        (struct_, members)
    }

    fn extension(&mut self, e: TypedExtension) -> Vec<MLFun> {
        let TypedExtension {
            name,
            protocol,
            computed_properties,
            member_functions,
        } = e;
        member_functions
            .into_iter()
            .map(|mf| {
                let TypedFun {
                    name: fname,
                    type_constraints,
                    arg_defs: args,
                    type_params,
                    body,
                    return_type,
                } = mf;
                let fun_arg_label_type_mangled_name = self.fun_arg_label_type_name_mangling(&args);
                let args = args.into_iter().map(|a| self.arg_def(a)).collect();
                MLFun {
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
                    body: body.map(|body| self.fun_body(body, None)),
                }
            })
            .collect()
    }

    fn protocol(&mut self, p: TypedProtocol) -> Vec<MLFun> {
        vec![]
    }

    fn expr(&mut self, e: TypedExpr) -> MLExpr {
        let TypedExpr { kind, ty } = e;
        match kind {
            TypedExprKind::Name(name) => self.name(name, ty),
            TypedExprKind::Literal(l) => MLExpr::Literal(self.literal(l, ty)),
            TypedExprKind::BinOp(b) => MLExpr::PrimitiveBinOp(self.binop(b, ty)),
            TypedExprKind::UnaryOp(u) => MLExpr::PrimitiveUnaryOp(self.unary_op(u, ty)),
            TypedExprKind::Subscript(s) => self.subscript(s, ty),
            TypedExprKind::Member(m) => self.member(m, ty),
            TypedExprKind::Array(a) => MLExpr::Array(self.array(a, ty)),
            TypedExprKind::Tuple => todo!(),
            TypedExprKind::Dict => todo!(),
            TypedExprKind::StringBuilder => todo!(),
            TypedExprKind::Call(c) => self.call(c, ty),
            TypedExprKind::If(i) => MLExpr::If(self.if_expr(i, ty)),
            TypedExprKind::When => todo!(),
            TypedExprKind::Lambda(l) => todo!(),
            TypedExprKind::Return(r) => MLExpr::Return(self.return_expr(r)),
            TypedExprKind::TypeCast(t) => MLExpr::PrimitiveTypeCast(self.type_cast(t)),
            TypedExprKind::SizeOf(t) => MLExpr::SizeOf(self.type_(t)),
        }
    }

    fn name(&self, n: TypedName, ty: Option<TypedType>) -> MLExpr {
        if let TypedType::Type(t) = ty.as_ref().unwrap() {
            let package = t.package().into_resolved();
            let name = t.name();
            let has_no_mangle = if let Some(i) = self.arena.get(&package.names, &name) {
                i.has_annotation(NO_MANGLE)
            } else {
                false
            };
            let mut mangled_name = if has_no_mangle {
                name
            } else {
                self.package_name_mangling_(&package, &name)
            };
            if let Some(type_arguments) = n.type_arguments {
                mangled_name += format!(
                    "<{}>",
                    type_arguments
                        .into_iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .as_str()
            };
            MLExpr::Literal(MLLiteral {
                kind: MLLiteralKind::Struct(vec![]),
                type_: MLValueType::Struct(mangled_name),
            })
        } else {
            let package = n.package.clone().into_resolved();
            let has_no_mangle = if let Some(i) = self.arena.get(&package.names, &n.name) {
                i.has_annotation(NO_MANGLE)
            } else {
                false
            };
            let mut mangled_name = if has_no_mangle {
                n.name
            } else {
                self.package_name_mangling_(&package, &*n.name)
            };
            if let Some(type_arguments) = n.type_arguments {
                mangled_name += format!(
                    "<{}>",
                    type_arguments
                        .into_iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .as_str()
            };
            if !has_no_mangle {
                if let Some(TypedType::Function(fun_type)) = &ty {
                    if !fun_type.arguments.is_empty() {
                        mangled_name += "##";
                        mangled_name += &*self.fun_arg_label_type_name_mangling(
                            &fun_type
                                .arguments
                                .iter()
                                .map(|a| TypedArgDef {
                                    label: a.label.to_string(),
                                    name: "".to_string(),
                                    type_: a.typ.clone(),
                                })
                                .collect(),
                        )
                    }
                }
            }
            MLExpr::Name(MLName {
                name: mangled_name,
                type_: self.type_(ty.unwrap()),
            })
        }
    }

    fn literal(&self, l: TypedLiteralKind, type_: Option<TypedType>) -> MLLiteral {
        let (kind, type_) = match l {
            TypedLiteralKind::Integer(value) => (
                MLLiteralKind::Integer(value),
                self.type_(type_.unwrap()).into_value_type(),
            ),
            TypedLiteralKind::FloatingPoint(value) => (
                MLLiteralKind::FloatingPoint(value),
                self.type_(type_.unwrap()).into_value_type(),
            ),
            TypedLiteralKind::String(value) => (
                MLLiteralKind::String(value),
                self.type_(type_.unwrap()).into_value_type(),
            ),
            TypedLiteralKind::Boolean(value) => (
                MLLiteralKind::Boolean(value),
                self.type_(type_.unwrap()).into_value_type(),
            ),
            TypedLiteralKind::NullLiteral => (
                MLLiteralKind::Null,
                self.type_(type_.unwrap()).into_value_type(),
            ),
        };
        MLLiteral { kind, type_ }
    }

    fn binop(&mut self, b: TypedBinOp, ty: Option<TypedType>) -> MLBinOp {
        let TypedBinOp {
            left,
            operator: kind,
            right,
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
            type_: self.type_(ty.unwrap()).into_value_type(),
        }
    }

    fn unary_op(&mut self, u: TypedUnaryOp, ty: Option<TypedType>) -> MLUnaryOp {
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
                    type_: self.type_(ty.unwrap()).into_value_type(),
                    target: Box::new(target),
                }
            }
            TypedUnaryOp::Postfix(p) => {
                todo!()
            }
        }
    }

    fn subscript(&mut self, s: TypedSubscript, ty: Option<TypedType>) -> MLExpr {
        let t = s.target.ty.clone().unwrap();
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
                            self.subscript_for_user_defined(s, ty)
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
                    _ => unreachable!(),
                },
            })
        } else {
            self.subscript_for_user_defined(s, ty)
        }
    }

    fn subscript_for_user_defined(&mut self, s: TypedSubscript, ty: Option<TypedType>) -> MLExpr {
        let target = self.expr(*s.target);
        match target {
            MLExpr::Name(target) => MLExpr::Call(MLCall {
                target,
                args: s
                    .indexes
                    .into_iter()
                    .map(|i| MLCallArg { arg: self.expr(i) })
                    .collect(),
                type_: self.type_(ty.unwrap()).into_value_type(),
            }),
            a => panic!("{:?}", a),
        }
    }

    fn member(&mut self, m: TypedInstanceMember, ty: Option<TypedType>) -> MLExpr {
        let TypedInstanceMember {
            target,
            name,
            is_safe,
        } = m;
        let target = self.expr(*target);
        let type_ = self.type_(ty.unwrap());
        MLExpr::Member(MLMember {
            target: Box::new(target),
            name,
            type_,
        })
    }

    fn array(&mut self, a: TypedArray, ty: Option<TypedType>) -> MLArray {
        MLArray {
            elements: a.elements.into_iter().map(|e| self.expr(e)).collect(),
            type_: self.type_(ty.unwrap()).into_value_type(),
        }
    }

    fn call(&mut self, c: TypedCall, ty: Option<TypedType>) -> MLExpr {
        let TypedCall { target, mut args } = c;
        let target = match *target {
            TypedExpr {
                kind: TypedExprKind::Member(m),
                ty,
            } => {
                let TypedInstanceMember {
                    target,
                    name,
                    is_safe,
                } = m;
                match target.ty.clone().unwrap() {
                    TypedType::Self_ => unreachable!(),
                    TypedType::Value(v) => {
                        let target_type = self.value_type(v);
                        let type_ = ty.unwrap();
                        if let TypedType::Function(fun_type) = &type_ {
                            args.insert(
                                0,
                                TypedCallArg {
                                    label: None,
                                    arg: target,
                                    is_vararg: false,
                                },
                            );
                            let mut mangled_name = target_type.name() + "::" + &*name;
                            if !fun_type.arguments.is_empty() {
                                mangled_name += "##";
                                mangled_name += &*self.fun_arg_label_type_name_mangling(
                                    &fun_type
                                        .arguments
                                        .iter()
                                        .map(|a| TypedArgDef {
                                            label: a.label.to_string(),
                                            name: "".to_string(),
                                            type_: a.typ.clone(),
                                        })
                                        .collect(),
                                )
                            }
                            MLExpr::Name(MLName {
                                name: mangled_name,
                                type_: self.type_(type_),
                            })
                        } else {
                            let is_stored = self.context.struct_has_field(&target_type, &name);
                            if is_stored {
                                let target = self.expr(*target);
                                let type_ = self.type_(type_);
                                MLExpr::Member(MLMember {
                                    target: Box::new(target),
                                    name,
                                    type_,
                                })
                            } else {
                                panic!("struct has no member {}", name)
                            }
                        }
                    }
                    TypedType::Function(_) => {
                        todo!()
                    }
                    TypedType::Type(t) => match *t {
                        TypedType::Self_ => unreachable!(),
                        TypedType::Value(t) => match t {
                            TypedValueType::Value(t) => {
                                let type_ = self.type_(ty.unwrap());
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
            MLExpr::Name(name) => name,
            MLExpr::Literal(MLLiteral {
                kind: MLLiteralKind::Struct(mut s),
                type_,
            }) => {
                for arg in args.into_iter() {
                    let v = self.expr(*arg.arg);
                    s.push((arg.label.unwrap(), v));
                }
                return MLExpr::Literal(MLLiteral {
                    kind: MLLiteralKind::Struct(s),
                    type_,
                });
            }
            a => panic!("{:?}", a),
        };
        MLExpr::Call(MLCall {
            target,
            args: args
                .into_iter()
                .map(|a| MLCallArg {
                    arg: self.expr(*a.arg),
                })
                .collect(),
            type_: self.type_(ty.unwrap()).into_value_type(),
        })
    }

    fn if_expr(&mut self, i: TypedIf, ty: Option<TypedType>) -> MLIf {
        MLIf {
            condition: Box::new(self.expr(*i.condition)),
            body: self.block(i.body),
            else_body: i.else_body.map(|b| self.block(b)),
            type_: self.type_(ty.unwrap()).into_value_type(),
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
            type_: self.type_(t.type_).into_value_type(),
        }
    }

    fn arg_def(&self, e: TypedArgDef) -> MLArgDef {
        MLArgDef {
            name: e.name,
            type_: self.type_(e.type_).into_value_type(),
        }
    }

    fn fun_body(
        &mut self,
        b: TypedFunBody,
        type_arguments: Option<HashMap<TypedTypeParam, TypedType>>,
    ) -> MLFunBody {
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
            body: b.body.into_iter().flat_map(|s| self.stmt(s)).collect(),
        }
    }

    fn package_name_mangling_(&self, package: &Package, name: &str) -> String {
        if package.is_global() || (name == "main" && self.config.type_() == BuildType::Binary) {
            String::from(name)
        } else {
            package.to_string() + "::" + name
        }
    }

    fn package_name_mangling(&self, package: &TypedPackage, name: &str) -> String {
        if let TypedPackage::Resolved(pkg) = package {
            self.package_name_mangling_(pkg, name)
        } else {
            panic!("pkg name mangling failed => {:?}, {}", package, name)
        }
    }

    fn fun_arg_label_type_name_mangling(&self, args: &Vec<TypedArgDef>) -> String {
        args.iter()
            .map(|arg| format!("{}#{}", arg.label, arg.type_.to_string()))
            .collect::<Vec<_>>()
            .join("##")
    }

    fn generate_test_harness(&self) -> MLFun {
        MLFun {
            name: "main".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Size),
            body: Some(MLFunBody {
                body: {
                    let mut tests: Vec<_> = self
                        .tests
                        .iter()
                        .map(|n| {
                            [
                                MLStmt::Expr(MLExpr::Call(MLCall {
                                    target: MLName {
                                        name: "puts".to_string(),
                                        type_: MLType::Function(MLFunctionType {
                                            arguments: vec![MLValueType::Pointer(Box::new(
                                                MLType::Value(MLValueType::Primitive(
                                                    MLPrimitiveType::UInt8,
                                                )),
                                            ))],
                                            return_type: MLValueType::Primitive(
                                                MLPrimitiveType::Size,
                                            ),
                                        }),
                                    },
                                    args: vec![MLCallArg {
                                        arg: MLExpr::Literal(MLLiteral {
                                            kind: MLLiteralKind::String(n.name.clone()),
                                            type_: MLValueType::Pointer(Box::new(MLType::Value(
                                                MLValueType::Primitive(MLPrimitiveType::UInt8),
                                            ))),
                                        }),
                                    }],
                                    type_: MLValueType::Primitive(MLPrimitiveType::Unit),
                                })),
                                MLStmt::Expr(MLExpr::Call(MLCall {
                                    target: MLName {
                                        name: n.name.clone(),
                                        type_: MLType::Function(MLFunctionType {
                                            arguments: n
                                                .arg_defs
                                                .iter()
                                                .map(|i| i.type_.clone())
                                                .collect(),
                                            return_type: n.return_type.clone(),
                                        }),
                                    },
                                    args: vec![],
                                    type_: n.return_type.clone(),
                                })),
                            ]
                        })
                        .flatten()
                        .collect();
                    tests.push(MLStmt::Return(MLReturn::new(Some(MLExpr::Literal(
                        MLLiteral {
                            kind: MLLiteralKind::Integer("0".to_string()),
                            type_: MLValueType::Primitive(MLPrimitiveType::Size),
                        },
                    )))));
                    tests
                },
            }),
        }
    }
}
