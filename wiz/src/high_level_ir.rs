use crate::ast::block::Block;
use crate::ast::decl::{
    Decl, FunSyntax, StoredPropertySyntax, StructPropertySyntax, StructSyntax, VarSyntax,
};
use crate::ast::expr::{CallExprSyntax, Expr, NameExprSyntax, ReturnSyntax};
use crate::ast::file::{FileSyntax, WizFile};
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::literal::Literal;
use crate::ast::stmt::{AssignmentStmt, LoopStmt, Stmt};
use crate::ast::type_name::TypeName;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDecl, TypedFun, TypedFunBody, TypedInitializer,
    TypedStoredProperty, TypedStruct, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember, TypedLiteral, TypedName,
    TypedReturn, TypedStaticMember,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentStmt, TypedBlock, TypedForStmt, TypedLoopStmt, TypedStmt,
    TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedFunctionType, TypedType, TypedValueType};
use crate::utils::stacked_hash_map::StackedHashMap;
use either::Either;
use std::collections::HashMap;
use std::fmt;
use std::option::Option::Some;
use std::process::exit;

pub mod typed_decl;
pub mod typed_expr;
pub mod typed_file;
pub mod typed_stmt;
pub mod typed_type;

#[derive(fmt::Debug, Clone)]
struct Ast2HLIRContext {
    name_environment: StackedHashMap<String, Ast2HLIRName>,
    struct_environment: StackedHashMap<TypedValueType, TypedStruct>,
}

#[derive(fmt::Debug, Clone)]
enum Ast2HLIRName {
    Type(TypedType),
    Name(TypedType),
}

impl Ast2HLIRContext {
    pub(crate) fn push(&mut self) {
        self.name_environment.push(HashMap::new());
        self.struct_environment.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.name_environment.pop();
        self.struct_environment.pop();
    }

    pub(crate) fn resolve_name(&self, name: String) -> Option<Ast2HLIRName> {
        let v = self.name_environment.get(&name)?;
        Some(v.clone())
    }

    pub(crate) fn put_name(&mut self, name: String, type_: &TypedType) {
        self.name_environment
            .insert(name, Ast2HLIRName::Name(type_.clone()));
    }

    fn put_type(&mut self, s: &TypedStruct) {
        let typed_value_type = TypedValueType {
            package: Package { names: vec![] },
            name: s.name.clone(),
            type_params: None
        };
        let name = typed_value_type.name.clone();
        let t = TypedType::Value(typed_value_type.clone());
        self.name_environment.insert(name, Ast2HLIRName::Type(t));
        self.struct_environment.insert(typed_value_type, s.clone());
    }

    fn resolve_type(&self, type_name: Option<TypeName>) -> Option<TypedType> {
        if let Some(type_name) = type_name {
            match self.name_environment.get(&type_name.name) {
                Some(Ast2HLIRName::Type(y)) => Some(y.clone()),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub struct Ast2HLIR {
    context: Ast2HLIRContext,
}

impl Ast2HLIR {
    pub fn new() -> Self {
        let mut builtin_types = HashMap::new();
        builtin_types.insert(String::from("Int8"), TypedType::int8());
        builtin_types.insert(String::from("Int16"), TypedType::int16());
        builtin_types.insert(String::from("Int32"), TypedType::int32());
        builtin_types.insert(String::from("Int64"), TypedType::int64());
        builtin_types.insert(String::from("UInt8"), TypedType::uint8());
        builtin_types.insert(String::from("UInt16"), TypedType::uint16());
        builtin_types.insert(String::from("UInt32"), TypedType::uint32());
        builtin_types.insert(String::from("UInt64"), TypedType::uint64());
        builtin_types.insert(String::from("String"), TypedType::string());
        builtin_types.insert(String::from("Noting"), TypedType::noting());
        builtin_types.insert(String::from("Unit"), TypedType::unit());
        let mut names = HashMap::new();
        for (k, v) in builtin_types.into_iter() {
            names.insert(k, Ast2HLIRName::Type(v));
        }
        Ast2HLIR {
            context: Ast2HLIRContext {
                name_environment: StackedHashMap::from(names),
                struct_environment: StackedHashMap::from(HashMap::new()),
            },
        }
    }

    pub fn preload_types(&mut self, ast: WizFile) {
        for decl in ast.syntax.body {
            match decl {
                Decl::Var(v) => {
                    let var = self.var_syntax(v);
                    self.context.put_name(var.name, &var.type_.unwrap())
                }
                Decl::Fun(f) => {
                    let return_type = match f.body {
                        Some(FunBody::Block { .. }) => {
                            if let Some(r) = f.return_type {
                                self.context.resolve_type(Some(r)).unwrap()
                            } else {
                                TypedType::unit()
                            }
                        }
                        Some(FunBody::Expr { expr }) => self.expr(expr).type_().unwrap(),
                        None => TypedType::unit(),
                    };
                    self.context.put_name(
                        f.name,
                        &TypedType::Function(Box::new(TypedFunctionType {
                            arguments: f.arg_defs.into_iter().map(|a| self.arg_def(a)).collect(),
                            return_type: return_type,
                        })),
                    )
                }
                Decl::Struct(s) => {
                    let s = self.struct_syntax(s);
                    println!("Struct {:?}", &s);
                    self.context.put_type(&s)
                }
                Decl::Class {} => {}
                Decl::Enum {} => {}
                Decl::Protocol {} => {}
                Decl::Extension {} => {}
            }
        }
    }

    fn resolve_by_binop(
        &self,
        left_type: &Option<TypedType>,
        kind: &String,
        right_type: &Option<TypedType>,
    ) -> Option<TypedType> {
        match (left_type, right_type) {
            (Some(l), Some(r)) => Some(l.clone()),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (_, _) => None,
        }
    }

    fn resolve_by_unaryop(
        &self,
        target_type: &Option<TypedType>,
        kind: &String,
    ) -> Option<TypedType> {
        None
    }

    fn resolve_member_type(&self, t: &TypedType, member_name: String) -> Option<TypedType> {
        match t {
            TypedType::Value(t) => {
                let s = self.context.struct_environment.get(t)?;
                for p in s.stored_properties.iter() {
                    if p.name == member_name {
                        return Some(p.type_.clone());
                    }
                }
                for p in s.computed_properties.iter() {
                    if p.name == member_name {
                        return Some(p.type_.clone());
                    }
                }
                // TODO: change resolve method
                if member_name == "init" {
                    return Some(TypedType::Value(t.clone()));
                }
                None
            }
            TypedType::Function(_) => None,
        }
    }

    pub fn file(&mut self, f: WizFile) -> TypedFile {
        TypedFile {
            name: f.name,
            body: self.file_syntax(f.syntax),
        }
    }

    pub fn file_syntax(&mut self, f: FileSyntax) -> Vec<TypedDecl> {
        f.body.into_iter().map(|d| self.decl(d)).collect()
    }

    pub fn stmt(&mut self, s: Stmt) -> TypedStmt {
        match s {
            Stmt::Decl { decl } => TypedStmt::Decl(self.decl(decl)),
            Stmt::Expr { expr } => TypedStmt::Expr(self.expr(expr)),
            Stmt::Assignment(a) => TypedStmt::Assignment(self.assignment(a)),
            Stmt::Loop(l) => TypedStmt::Loop(self.loop_stmt(l)),
        }
    }

    pub fn assignment(&mut self, a: AssignmentStmt) -> TypedAssignmentStmt {
        match a {
            AssignmentStmt::Assignment(a) => TypedAssignmentStmt::Assignment(TypedAssignment {
                target: self.expr(a.target),
                value: self.expr(a.value),
            }),
            AssignmentStmt::AssignmentAndOperator(_) => exit(-1),
        }
    }

    pub fn loop_stmt(&mut self, l: LoopStmt) -> TypedLoopStmt {
        match l {
            LoopStmt::While { condition, block } => TypedLoopStmt::While(TypedWhileLoopStmt {
                condition: self.expr(condition),
                block: self.block_with_env(block),
            }),
            LoopStmt::For {
                values,
                iterator,
                block,
            } => TypedLoopStmt::For(TypedForStmt {
                values: values,
                iterator: self.expr(iterator),
                block: self.block_with_env(block),
            }),
        }
    }

    pub fn decl(&mut self, d: Decl) -> TypedDecl {
        match d {
            Decl::Var(v) => TypedDecl::Var(self.var_syntax(v)),
            Decl::Fun(f) => TypedDecl::Fun(self.fun_syntax(f)),
            Decl::Struct(s) => {
                let struct_ = self.struct_syntax(s);
                let struct_ = self.default_init_if_needed(struct_);
                TypedDecl::Struct(struct_)
            }
            Decl::Class { .. } => TypedDecl::Class,
            Decl::Enum { .. } => TypedDecl::Enum,
            Decl::Protocol { .. } => TypedDecl::Protocol,
            Decl::Extension { .. } => TypedDecl::Extension,
        }
    }

    pub fn var_syntax(&mut self, v: VarSyntax) -> TypedVar {
        println!("{:?}", &v.value);
        let expr = self.expr(v.value);
        let type_ = match (v.type_, expr.type_()) {
            (Some(tn), Some(expr_type)) => {
                let var_type = self.context.resolve_type(Some(tn.clone()));
                if let Some(var_type) = var_type {
                    if var_type == expr_type {
                        expr_type
                    } else {
                        eprintln!(
                            "Type miss match error => {:?} and {:?}",
                            var_type, expr_type
                        );
                        exit(-1);
                    }
                } else {
                    eprintln!("Can not resolve type {:?} error =>", tn);
                    exit(-1)
                }
            }
            (Some(t), None) => {
                if let Some(tt) = self.context.resolve_type(Some(t.clone())) {
                    tt
                } else {
                    eprintln!("Can not resolve type {:?} error =>", t);
                    exit(-1)
                }
            }
            (None, Some(t)) => t,
            (None, None) => {
                eprintln!("Can not resolve type error");
                exit(-1)
            }
        };
        self.context.put_name(v.name.clone(), &type_);
        TypedVar {
            is_mut: v.is_mut,
            name: v.name,
            type_: Some(type_),
            value: expr,
        }
    }

    pub fn arg_def(&self, a: ArgDef) -> TypedArgDef {
        TypedArgDef {
            label: a.label,
            name: a.name,
            type_: self.context.resolve_type(Some(a.type_name)).unwrap(),
        }
    }

    pub fn fun_syntax(&mut self, f: FunSyntax) -> TypedFun {
        println!("{:?}", &f);
        let args: Vec<TypedArgDef> = f.arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        self.context.push();
        for arg in args.iter() {
            self.context.put_name(arg.name.clone(), &arg.type_)
        }
        let body = match f.body {
            None => None,
            Some(b) => Some(match b {
                FunBody::Block { block } => TypedFunBody::Block(self.block(block)),
                FunBody::Expr { expr } => TypedFunBody::Expr(self.expr(expr)),
            }),
        };

        let return_type = self.context.resolve_type(f.return_type);

        let return_type = match return_type {
            None => match &body {
                Some(TypedFunBody::Expr(e)) => e.type_().unwrap(),
                Some(TypedFunBody::Block(b)) => TypedType::unit(),
                None => {
                    eprintln!("Can not resolve type...");
                    exit(-1)
                }
            },
            Some(t) => t,
        };

        let f = TypedFun {
            modifiers: f.modifiers,
            name: f.name,
            arg_defs: args,
            body: body,
            return_type: return_type,
        };
        self.context.pop();
        self.context.put_name(f.name.clone(), &f.type_());
        f
    }

    pub fn struct_syntax(&mut self, s: StructSyntax) -> TypedStruct {
        let mut stored_properties: Vec<TypedStoredProperty> = vec![];
        let mut computed_properties: Vec<TypedComputedProperty> = vec![];
        let mut initializers: Vec<TypedInitializer> = vec![];
        for p in s.properties {
            match p {
                StructPropertySyntax::StoredProperty(v) => {
                    stored_properties.push(self.stored_property_syntax(v));
                }
                StructPropertySyntax::ComputedProperty => {}
                StructPropertySyntax::Init => {}
                StructPropertySyntax::Method => {}
            };
        }
        TypedStruct {
            name: s.name,
            init: initializers,
            stored_properties,
            computed_properties,
            member_functions: vec![],
            static_function: vec![],
        }
    }

    fn default_init_if_needed(&self, mut s: TypedStruct) -> TypedStruct {
        let args: Vec<TypedArgDef> = s
            .stored_properties
            .iter()
            .map(|p| TypedArgDef {
                label: p.name.clone(),
                name: p.name.clone(),
                type_: p.type_.clone(),
            })
            .collect();
        if s.init.is_empty() {
            let struct_type = TypedValueType {
                package: Package { names: vec![] },
                name: s.name.clone(),
                type_params: None
            };
            s.init.push(TypedInitializer {
                type_: TypedType::Value(struct_type.clone()),
                args,
                block: TypedBlock {
                    body: s
                        .stored_properties
                        .iter()
                        .map(|p| {
                            let struct_type = TypedValueType {
                                package: Package { names: vec![] },
                                name: s.name.clone(),
                                type_params: None
                            };
                            TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(struct_type)),
                                        })),
                                        name: p.name.clone(),
                                        is_safe: false,
                                        type_: Some(p.type_.clone()),
                                    }),
                                    value: TypedExpr::Name(TypedName {
                                        name: p.name.clone(),
                                        type_: Some(p.type_.clone()),
                                    }),
                                },
                            ))
                        })
                        .collect(),
                },
            })
        }
        s
    }

    pub fn stored_property_syntax(&self, p: StoredPropertySyntax) -> TypedStoredProperty {
        TypedStoredProperty {
            name: p.name,
            type_: self.context.resolve_type(Some(p.type_)).unwrap(),
        }
    }

    pub fn expr(&mut self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name(n) => match self.name_syntax(n) {
                Either::Left(name) => TypedExpr::Name(name),
                Either::Right(n) => TypedExpr::Type(n),
            },
            Expr::Literal(literal) => match literal {
                Literal::IntegerLiteral { value } => TypedExpr::Literal(TypedLiteral::Integer {
                    value,
                    type_: TypedType::int64(),
                }),
                Literal::FloatingPointLiteral { value } => {
                    TypedExpr::Literal(TypedLiteral::FloatingPoint {
                        value,
                        type_: TypedType::double(),
                    })
                }
                Literal::StringLiteral { value } => TypedExpr::Literal(TypedLiteral::String {
                    value,
                    type_: TypedType::string(),
                }),
                Literal::BooleanLiteral { value } => TypedExpr::Literal(TypedLiteral::Boolean {
                    value,
                    type_: TypedType::bool(),
                }),
                Literal::NullLiteral => TypedExpr::Literal(TypedLiteral::NullLiteral {
                    type_: TypedType::Value(TypedValueType {
                        package: Package { names: vec![] },
                        name: "Option".to_string(),
                        type_params: None
                    }),
                }),
            },
            Expr::BinOp { left, kind, right } => {
                let left = Box::new(self.expr(*left));
                let right = Box::new(self.expr(*right));
                let type_ = self.resolve_by_binop(&left.type_(), &kind, &right.type_());
                TypedExpr::BinOp {
                    left: left,
                    kind: kind,
                    right: right,
                    type_: type_,
                }
            }
            Expr::UnaryOp {
                target,
                prefix,
                kind,
            } => {
                let target = self.expr(*target);
                let type_ = self.resolve_by_unaryop(&target.type_(), &kind);
                TypedExpr::UnaryOp {
                    target: Box::new(target),
                    prefix: prefix,
                    kind: kind,
                    type_: type_,
                }
            }
            Expr::Subscript { .. } => TypedExpr::Subscript,
            Expr::Member {
                target,
                name,
                is_safe,
            } => {
                let target = self.expr(*target);
                if let TypedExpr::Type(target) = target {
                    let type_ = self.resolve_member_type(&target, name.clone());
                    TypedExpr::StaticMember(TypedStaticMember {
                        target: target,
                        name: name,
                        type_: type_,
                    })
                } else {
                    let target_type = target.type_().unwrap();
                    let type_ = self.resolve_member_type(&target_type, name.clone());
                    TypedExpr::Member(TypedInstanceMember {
                        target: Box::new(target),
                        name,
                        is_safe,
                        type_,
                    })
                }
            }
            Expr::List { .. } => TypedExpr::List,
            Expr::Tuple { .. } => TypedExpr::Tuple,
            Expr::Dict { .. } => TypedExpr::Dict,
            Expr::StringBuilder { .. } => TypedExpr::StringBuilder,
            Expr::Call(c) => TypedExpr::Call(self.call_syntax(c)),
            Expr::If {
                condition,
                body,
                else_body,
            } => {
                let block = self.block_with_env(body);
                let type_ = block.type_();
                TypedExpr::If(TypedIf {
                    condition: Box::new(self.expr(*condition)),
                    body: block,
                    else_body: else_body.map(|b| self.block_with_env(b)),
                    type_: type_,
                })
            }
            Expr::When { .. } => TypedExpr::When,
            Expr::Lambda { .. } => TypedExpr::Lambda,
            Expr::Return(r) => TypedExpr::Return(self.return_syntax(r)),
            Expr::TypeCast { .. } => TypedExpr::TypeCast,
        }
    }

    pub fn name_syntax(&self, n: NameExprSyntax) -> Either<TypedName, TypedType> {
        let NameExprSyntax { name } = n;
        match self.context.resolve_name(name.clone()) {
            None => Either::Left(TypedName {
                name: name,
                type_: None,
            }),
            Some(Ast2HLIRName::Name(t)) => Either::Left(TypedName {
                name: name,
                type_: Some(t),
            }),
            Some(Ast2HLIRName::Type(t)) => Either::Right(t),
        }
    }

    pub fn call_syntax(&mut self, c: CallExprSyntax) -> TypedCall {
        let CallExprSyntax {
            target,
            args,
            tailing_lambda,
        } = c;
        let mut args: Vec<TypedCallArg> = args
            .into_iter()
            .map(|a| TypedCallArg {
                label: a.label,
                arg: Box::new(self.expr(*a.arg)),
                is_vararg: a.is_vararg,
            })
            .collect();
        if let Some(lambda) = tailing_lambda {
            args.insert(
                args.len(),
                TypedCallArg {
                    label: None,
                    arg: Box::new(TypedExpr::Lambda /* TODO: use lambda */),
                    is_vararg: false,
                },
            )
        }
        // TODO: resolve call type
        let e = self.expr(*target);
        let return_type = e.type_();

        TypedCall {
            target: Box::new(e),
            args: args,
            type_: return_type,
        }
    }

    pub fn return_syntax(&mut self, r: ReturnSyntax) -> TypedReturn {
        let value = r.value.map(|v| Box::new(self.expr(*v)));
        let t = match &value {
            Some(v) => v.type_(),
            None => None,
        };
        TypedReturn {
            value: value,
            type_: t,
        }
    }

    pub fn block(&mut self, block: Block) -> TypedBlock {
        TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }

    pub fn block_with_env(&mut self, block: Block) -> TypedBlock {
        self.context.push();
        let b = self.block(block);
        self.context.pop();
        b
    }
}
