use crate::high_level_ir::typed_annotation::TypedAnnotations;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedMemberFunction, TypedStruct,
    TypedValueArgDef, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedIf,
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
use crate::middle_level_ir::ml_decl::{
    MLArgDef, MLDecl, MLField, MLFun, MLFunBody, MLStruct, MLVar,
};
use crate::middle_level_ir::expr::{
    MLBinOp, MLBinOpKind, MLCall, MLCallArg, MLExpr, MLIf, MLLiteral, MLMember, MLName, MLReturn,
    MLSubscript, MLTypeCast, MLUnaryOp, MLUnaryOpKind,
};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLAssignmentStmt, MLBlock, MLLoopStmt, MLStmt};
use crate::middle_level_ir::ml_type::{MLFunctionType, MLPrimitiveType, MLType, MLValueType};
use std::collections::HashMap;
use std::option::Option::Some;
use std::process::exit;

pub mod builder;
pub mod format;
pub mod ml_decl;
pub mod expr;
pub mod ml_file;
pub mod ml_node;
pub mod ml_stmt;
pub mod ml_type;
#[cfg(test)]
mod tests;

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
        match an {
            None => false,
            Some(an) => an.has_annotate(annotation),
        }
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
}

impl HLIR2MLIR {
    pub fn new() -> Self {
        HLIR2MLIR {
            context: HLIR2MLIRContext::new(),
        }
    }

    pub fn type_(&self, t: TypedType) -> MLType {
        match t {
            TypedType::Value(t) => MLType::Value(self.value_type(t)),
            TypedType::Function(f) => MLType::Function(self.function_type(*f)),
            _ => panic!("Invalid Type convert  {:?}", t),
        }
    }

    pub fn value_type(&self, t: TypedValueType) -> MLValueType {
        if t.is_unsafe_pointer() {
            match self.type_(t.type_args.unwrap()[0].clone()) {
                MLType::Value(v) => MLValueType::Pointer(Box::new(v)),
                MLType::Function(f) => {
                    eprintln!("Function Pointer is unsupported {:?}", f);
                    exit(-1)
                }
            }
        } else {
            let mut pkg = t.package.clone().into_resolved().names;
            if pkg.is_empty() {
                match &*t.name {
                    "Noting" => MLValueType::Primitive(MLPrimitiveType::Noting),
                    "Unit" => MLValueType::Primitive(MLPrimitiveType::Unit),
                    "Int8" => MLValueType::Primitive(MLPrimitiveType::Int8),
                    "UInt8" => MLValueType::Primitive(MLPrimitiveType::UInt8),
                    "Int16" => MLValueType::Primitive(MLPrimitiveType::Int16),
                    "UInt16" => MLValueType::Primitive(MLPrimitiveType::UInt16),
                    "Int32" => MLValueType::Primitive(MLPrimitiveType::Int32),
                    "UInt32" => MLValueType::Primitive(MLPrimitiveType::UInt32),
                    "Int64" => MLValueType::Primitive(MLPrimitiveType::Int64),
                    "UInt64" => MLValueType::Primitive(MLPrimitiveType::UInt64),
                    "Size" => MLValueType::Primitive(MLPrimitiveType::Size),
                    "USize" => MLValueType::Primitive(MLPrimitiveType::USize),
                    "Bool" => MLValueType::Primitive(MLPrimitiveType::Bool),
                    "Float" => MLValueType::Primitive(MLPrimitiveType::Float),
                    "Double" => MLValueType::Primitive(MLPrimitiveType::Double),
                    "String" => MLValueType::Primitive(MLPrimitiveType::String),
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
    }

    pub fn function_type(&self, t: TypedFunctionType) -> MLFunctionType {
        MLFunctionType {
            arguments: t
                .arguments
                .into_iter()
                .map(|a| match self.type_(a.type_().unwrap()) {
                    MLType::Value(v) => v,
                    MLType::Function(_) => exit(-9),
                })
                .collect(),
            return_type: match self.type_(t.return_type) {
                MLType::Value(v) => v,
                MLType::Function(f) => {
                    println!("{:?}", f);
                    exit(-9)
                }
            },
        }
    }

    pub fn source_set(&mut self, s: TypedSourceSet) -> Vec<MLFile> {
        match s {
            TypedSourceSet::File(f) => {
                vec![self.file(f)]
            }
            TypedSourceSet::Dir { name, items } => {
                self.context.push_name_space(name);
                let i = items
                    .into_iter()
                    .map(|i| self.source_set(i))
                    .flatten()
                    .collect();
                self.context.pop_name_space();
                i
            }
        }
    }

    pub fn file(&mut self, f: TypedFile) -> MLFile {
        self.context.push_name_space(f.name.clone());
        let f = MLFile {
            name: f.name,
            body: f.body.into_iter().map(|d| self.decl(d)).flatten().collect(),
        };
        self.context.pop_name_space();
        f
    }

    pub fn stmt(&mut self, s: TypedStmt) -> Vec<MLStmt> {
        match s {
            TypedStmt::Expr(e) => vec![MLStmt::Expr(self.expr(e))],
            TypedStmt::Decl(d) => self
                .decl(d)
                .into_iter()
                .map(|dc| match dc {
                    MLDecl::Var(v) => MLStmt::Var(v),
                    MLDecl::Fun(_) => {
                        todo!("local function")
                    }
                    MLDecl::Struct(_) => {
                        todo!("local struct")
                    }
                })
                .collect(),
            TypedStmt::Assignment(a) => vec![MLStmt::Assignment(self.assignment(a))],
            TypedStmt::Loop(l) => vec![MLStmt::Loop(self.loop_stmt(l))],
        }
    }

    pub fn assignment(&mut self, a: TypedAssignmentStmt) -> MLAssignmentStmt {
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
                    target: target,
                    value: self.expr(value),
                }
            }
        }
    }

    pub fn loop_stmt(&mut self, l: TypedLoopStmt) -> MLLoopStmt {
        match l {
            TypedLoopStmt::While(w) => MLLoopStmt {
                condition: self.expr(w.condition),
                block: self.block(w.block),
            },
            TypedLoopStmt::For(_) => exit(-1),
        }
    }

    pub fn decl(&mut self, d: TypedDecl) -> Vec<MLDecl> {
        match d {
            TypedDecl::Var(v) => vec![MLDecl::Var(self.var(v))],
            TypedDecl::Fun(f) => vec![MLDecl::Fun(self.fun(f))],
            TypedDecl::Struct(s) => {
                let (st, fns) = self.struct_(s);
                vec![MLDecl::Struct(st)]
                    .into_iter()
                    .chain(fns.into_iter().map(|f| MLDecl::Fun(f)))
                    .collect()
            }
            TypedDecl::Class => exit(-1),
            TypedDecl::Enum => exit(-1),
            TypedDecl::Protocol => exit(-1),
            TypedDecl::Extension => exit(-1),
        }
    }

    pub fn var(&mut self, v: TypedVar) -> MLVar {
        let expr = self.expr(v.value);
        MLVar {
            is_mute: v.is_mut,
            name: v.name,
            type_: self.type_(v.type_.unwrap()),
            value: expr,
        }
    }

    pub fn fun(&mut self, f: TypedFun) -> MLFun {
        let TypedFun {
            annotations,
            package,
            modifiers,
            name,
            type_params,
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

    pub fn struct_(&mut self, s: TypedStruct) -> (MLStruct, Vec<MLFun>) {
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
                    body: match body {
                        None => None,
                        Some(body) => Some(self.fun_body(body)),
                    },
                }
            })
            .collect();
        (struct_, init.into_iter().chain(members).collect())
    }

    pub fn expr(&mut self, e: TypedExpr) -> MLExpr {
        match e {
            TypedExpr::Name(name) => MLExpr::Name(self.name(name)),
            TypedExpr::Literal(l) => MLExpr::Literal(self.literal(l)),
            TypedExpr::BinOp(b) => MLExpr::PrimitiveBinOp(self.binop(b)),
            TypedExpr::UnaryOp(u) => MLExpr::PrimitiveUnaryOp(self.unary_op(u)),
            TypedExpr::Subscript(s) => self.subscript(s),
            TypedExpr::Member(m) => self.member(m),
            TypedExpr::Array(a) => todo!(),
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

    pub fn name(&self, n: TypedName) -> MLName {
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

    pub fn literal(&self, l: TypedLiteral) -> MLLiteral {
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

    pub fn binop(&mut self, b: TypedBinOp) -> MLBinOp {
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

    pub fn unary_op(&mut self, u: TypedUnaryOp) -> MLUnaryOp {
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
                    type_: target.type_().into_value_type(),
                    target: Box::new(target),
                }
            }
            TypedUnaryOp::Postfix(p) => {
                todo!()
            }
        }
    }

    pub fn subscript(&mut self, s: TypedSubscript) -> MLExpr {
        let t = s.target.type_().unwrap();
        if t.is_pointer_type() && s.indexes.len() == 1 {
            match t {
                TypedType::Value(v) => {
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
                _ => {
                    eprintln!("function pointer detected");
                    exit(-1)
                }
            }
        } else {
            self.subscript_for_user_defined(s)
        }
    }

    fn subscript_for_user_defined(&mut self, s: TypedSubscript) -> MLExpr {
        MLExpr::Call(MLCall {
            target: Box::new(self.expr(*s.target)),
            args: s
                .indexes
                .into_iter()
                .map(|i| MLCallArg { arg: self.expr(i) })
                .collect(),
            type_: self.type_(s.type_.unwrap()),
        })
    }

    pub fn member(&mut self, m: TypedInstanceMember) -> MLExpr {
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

    pub fn call(&mut self, c: TypedCall) -> MLCall {
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
                                type_: MLType::Value(target_type),
                            })
                        }
                    }
                    TypedType::Function(_) => {
                        todo!()
                    }
                    TypedType::Type(t) => {
                        let type_ = self.type_(type_.unwrap());
                        MLExpr::Name(MLName {
                            name: self.package_name_mangling(&t.package, &*t.name) + "::" + &*name,
                            type_,
                        })
                    }
                    TypedType::Reference(v) => {
                        todo!()
                    }
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
                                        .map(|a| {
                                            TypedArgDef::Value(TypedValueArgDef {
                                                label: match &a.label {
                                                    None => "_".to_string(),
                                                    Some(l) => l.to_string(),
                                                },
                                                name: "".to_string(),
                                                type_: a.arg.type_().unwrap(),
                                            })
                                        })
                                        .collect(),
                                )
                        }
                    };
                MLExpr::Name(MLName {
                    name: fun_arg_label_type_mangled_name,
                    type_,
                })
            }
            a => a,
        };
        MLCall {
            target: Box::new(target),
            args: args
                .into_iter()
                .map(|a| MLCallArg {
                    arg: self.expr(*a.arg),
                })
                .collect(),
            type_: self.type_(type_.unwrap()),
        }
    }

    pub fn if_expr(&mut self, i: TypedIf) -> MLIf {
        MLIf {
            condition: Box::new(self.expr(*i.condition)),
            body: self.block(i.body),
            else_body: i.else_body.map(|b| self.block(b)),
            type_: self.type_(i.type_.unwrap()),
        }
    }

    pub fn return_expr(&mut self, r: TypedReturn) -> MLReturn {
        MLReturn {
            value: r.value.map(|v| Box::new(self.expr(*v))),
        }
    }

    pub fn type_cast(&mut self, t: TypedTypeCast) -> MLTypeCast {
        MLTypeCast {
            target: Box::new(self.expr(*t.target)),
            type_: self.type_(t.type_.unwrap()).into_value_type(),
        }
    }

    pub fn arg_def(&self, e: TypedArgDef) -> MLArgDef {
        MLArgDef {
            name: e.name(),
            type_: self.type_(e.type_().unwrap()).into_value_type(),
        }
    }

    pub fn fun_body(&mut self, b: TypedFunBody) -> MLFunBody {
        match b {
            TypedFunBody::Expr(e) => MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Return(MLReturn::new(self.expr(e))))],
            },
            TypedFunBody::Block(b) => MLFunBody {
                body: self.block(b).body,
            },
        }
    }

    pub fn block(&mut self, b: TypedBlock) -> MLBlock {
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
            .map(|arg| arg.label() + "#" + &*arg.type_().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("##")
    }
}
