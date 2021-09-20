use crate::ast::block::Block;
use crate::ast::decl::{
    Decl, FunSyntax, InitializerSyntax, MethodSyntax, StoredPropertySyntax, StructPropertySyntax,
    StructSyntax, VarSyntax,
};
use crate::ast::expr::{CallExprSyntax, Expr, NameExprSyntax, ReturnSyntax, SubscriptSyntax};
use crate::ast::file::{FileSyntax, WizFile};
use crate::ast::fun::arg_def::ArgDef;
use crate::ast::fun::body_def::FunBody;
use crate::ast::literal::Literal;
use crate::ast::stmt::{AssignmentStmt, LoopStmt, Stmt};
use crate::ast::type_name::{TypeName, TypeParam};
use crate::constants::UNSAFE_POINTER;
use crate::high_level_ir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDecl, TypedFun, TypedFunBody, TypedInitializer,
    TypedMemberFunction, TypedStoredProperty, TypedStruct, TypedValueArgDef, TypedVar,
};
use crate::high_level_ir::typed_expr::{
    TypedBinOp, TypedCall, TypedCallArg, TypedExpr, TypedIf, TypedInstanceMember, TypedLiteral,
    TypedName, TypedReturn, TypedStaticMember, TypedSubscript, TypedUnaryOp,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentStmt, TypedBlock, TypedForStmt,
    TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{
    Package, TypedFunctionType, TypedType, TypedTypeParam, TypedValueType,
};
use crate::utils::path_string_to_page_name;
use crate::utils::stacked_hash_map::StackedHashMap;
use std::collections::HashMap;
use std::fmt;
use std::option::Option::Some;
use std::process::exit;

pub mod type_resolver;
pub mod typed_decl;
pub mod typed_expr;
pub mod typed_file;
pub mod typed_stmt;
pub mod typed_type;

#[derive(fmt::Debug, Clone)]
struct Ast2HLIRTypeParam {
    name: String,
    type_constraints: Vec<String>,
}

#[derive(fmt::Debug, Clone)]
struct Ast2HLIRType {
    name: String,
    type_params: Option<Vec<Ast2HLIRTypeParam>>,
}

#[derive(fmt::Debug, Clone)]
struct Ast2HLIRContext {
    name_environment: StackedHashMap<String, Ast2HLIRName>,
    struct_environment: StackedHashMap<String, TypedStruct>,
}

#[derive(fmt::Debug, Clone)]
enum Ast2HLIRName {
    Type(Ast2HLIRType),
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
        let name = s.name.clone();
        let t = Ast2HLIRType {
            name: name.clone(),
            type_params: s.type_params.as_ref().map(|v| {
                v.iter()
                    .map(|tp| {
                        Ast2HLIRTypeParam {
                            name: tp.name.clone(),
                            type_constraints: vec![], // TODO: Type constraint
                        }
                    })
                    .collect()
            }),
        };
        self.name_environment
            .insert(name.clone(), Ast2HLIRName::Type(t));
        self.struct_environment.insert(name, s.clone());
    }

    fn resolve_type(&self, type_name: Option<TypeName>) -> Option<TypedType> {
        let t = if let Some(type_name) = &type_name {
            match self.name_environment.get(&type_name.name) {
                Some(Ast2HLIRName::Type(y)) => Some(y.clone()),
                _ => None,
            }
        } else {
            None
        };
        match (&t, &type_name) {
            (None, None) => {
                eprintln!("Unresolved type {:?}.", type_name);
                None
            }
            (None, Some(t)) => {
                eprintln!("Unresolved type {:?}.", t);
                None
            }
            (Some(a2ht), Some(t)) => match (&a2ht.type_params, &t.type_args) {
                (None, None) => Some(TypedType::Value(TypedValueType {
                    package: Package { names: vec![] },
                    name: a2ht.name.clone(),
                    type_args: None,
                })),
                (None, Some(args)) => {
                    eprintln!(
                        "{:?} is not take type args. but passed {:?}",
                        a2ht.name, args
                    );
                    None
                }
                (Some(_), None) => {
                    eprintln!("{:?} is take type args. but not passed", a2ht.name);
                    None
                }
                (Some(params), Some(args)) => {
                    if params.len() != args.len() {
                        eprintln!(
                            "{:?} take {} type arguments. but {} given",
                            a2ht.name,
                            params.len(),
                            args.len()
                        );
                        None
                    } else {
                        Some(TypedType::Value(TypedValueType {
                            package: Package { names: vec![] },
                            name: a2ht.name.clone(),
                            type_args: Some(
                                params
                                    .into_iter()
                                    .zip(args)
                                    .map(|(p, t)| self.resolve_type(Some(t.clone())).unwrap())
                                    .collect(),
                            ),
                        }))
                    }
                }
            },
            _ => {
                eprintln!("Never execution branch executed!!");
                exit(-1)
            }
        }
    }

    fn resolve_subscript(
        &self,
        target_type: Option<TypedType>,
        indexes: Vec<Option<TypedType>>,
    ) -> Option<TypedType> {
        match target_type {
            Some(TypedType::Value(v)) => {
                if v.name == UNSAFE_POINTER {
                    if indexes.len() == 1 {
                        let t = v.type_args.unwrap()[0].clone();
                        Some(t)
                    } else {
                        eprintln!("{:?} is not support subscript by {:?}", v, indexes);
                        exit(-1)
                    }
                } else {
                    eprintln!("{:?} is not support subscript by {:?}", v, indexes);
                    exit(-1)
                }
            }
            Some(a) => {
                eprintln!("{:?} is not support subscript by {:?}", a, indexes);
                exit(-1)
            }
            None => {
                eprintln!(
                    "Can not resolve subscript. {:?}[{:?}]",
                    target_type, indexes
                );
                exit(-1)
            }
        }
    }
}

pub struct Ast2HLIR {
    context: Ast2HLIRContext,
}

impl Ast2HLIR {
    pub fn new() -> Self {
        let builtin_types = vec![
            String::from("Int8"),
            String::from("Int16"),
            String::from("Int32"),
            String::from("Int64"),
            String::from("UInt8"),
            String::from("UInt16"),
            String::from("UInt32"),
            String::from("UInt64"),
            String::from("String"),
            String::from("Noting"),
            String::from("Unit"),
        ];
        let mut names = HashMap::new();
        for t in builtin_types.into_iter() {
            names.insert(
                t.clone(),
                Ast2HLIRName::Type(Ast2HLIRType {
                    name: t,
                    type_params: None,
                }),
            );
        }
        names.insert(
            String::from(UNSAFE_POINTER),
            Ast2HLIRName::Type(Ast2HLIRType {
                name: String::from(UNSAFE_POINTER),
                type_params: Some(vec![Ast2HLIRTypeParam {
                    name: "T".to_string(),
                    type_constraints: vec![],
                }]),
            }),
        );
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
                println!("env :: {:?}", self.context.struct_environment);
                let s = self.context.struct_environment.get(&t.name)?;
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
                    return Some(TypedType::Function(Box::new(TypedFunctionType {
                        arguments: vec![],
                        return_type: TypedType::Value(t.clone()),
                    })));
                }
                None
            }
            _ => None,
        }
    }

    pub fn file(&mut self, f: WizFile) -> TypedFile {
        TypedFile {
            name: path_string_to_page_name(f.name),
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
            AssignmentStmt::AssignmentAndOperator(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(TypedAssignmentAndOperation {
                    target: self.expr(a.target),
                    operator: a.operator,
                    value: self.expr(a.value),
                })
            }
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
        match a {
            ArgDef::Value(a) => TypedArgDef::Value(TypedValueArgDef {
                label: a.label,
                name: a.name,
                type_: self.context.resolve_type(Some(a.type_name)).unwrap(),
            }),
            ArgDef::Self_ => TypedArgDef::Self_(None),
        }
    }

    pub fn fun_body(&mut self, body: FunBody) -> TypedFunBody {
        match body {
            FunBody::Block { block } => TypedFunBody::Block(self.block(block)),
            FunBody::Expr { expr } => TypedFunBody::Expr(self.expr(expr)),
        }
    }

    pub fn fun_syntax(&mut self, f: FunSyntax) -> TypedFun {
        let args: Vec<TypedArgDef> = f.arg_defs.into_iter().map(|a| self.arg_def(a)).collect();
        self.context.push();
        for arg in args.iter() {
            self.context.put_name(arg.name(), &arg.type_().unwrap())
        }
        let body = match f.body {
            None => None,
            Some(b) => Some(self.fun_body(b)),
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
            type_params: f.type_params.map(|v| {
                v.into_iter()
                    .map(|p| TypedTypeParam {
                        name: p.name,
                        type_constraint: match p.type_constraints {
                            None => {
                                vec![]
                            }
                            Some(tn) => {
                                vec![self.type_(tn)]
                            }
                        },
                    })
                    .collect()
            }),
            arg_defs: args,
            body: body,
            return_type: Some(return_type),
        };
        self.context.pop();
        self.context.put_name(f.name.clone(), &f.type_().unwrap());
        f
    }

    pub fn type_(&self, tn: TypeName) -> TypedType {
        TypedType::Value(TypedValueType {
            package: Package { names: vec![] },
            name: tn.name,
            type_args: tn
                .type_args
                .map(|v| v.into_iter().map(|t| self.type_(t)).collect()),
        })
    }

    fn type_param(&self, tp: TypeParam) -> TypedTypeParam {
        TypedTypeParam {
            name: tp.name,
            type_constraint: tp.type_constraints.map_or(vec![], |v| vec![self.type_(v)]),
        }
    }

    pub fn struct_syntax(&mut self, s: StructSyntax) -> TypedStruct {
        self.context.push();
        if let Some(type_params) = &s.type_params {
            for type_param in type_params.iter() {
                self.context.put_type(&TypedStruct {
                    name: type_param.name.clone(),
                    type_params: None,
                    init: vec![],
                    stored_properties: vec![],
                    computed_properties: vec![],
                    member_functions: vec![],
                    static_function: vec![],
                });
            }
        };
        self.context.put_name(
            String::from("self"),
            &TypedType::Value(TypedValueType {
                package: Package { names: vec![] },
                name: s.name.to_string(),
                type_args: None,
            }),
        );
        let mut stored_properties: Vec<TypedStoredProperty> = vec![];
        let mut computed_properties: Vec<TypedComputedProperty> = vec![];
        let mut initializers: Vec<TypedInitializer> = vec![];
        let mut member_functions: Vec<TypedMemberFunction> = vec![];
        for p in s.properties {
            match p {
                StructPropertySyntax::StoredProperty(v) => {
                    stored_properties.push(self.stored_property_syntax(v));
                }
                StructPropertySyntax::ComputedProperty => {}
                StructPropertySyntax::Init(init) => {
                    initializers.push(self.initializer_syntax(init))
                }
                StructPropertySyntax::Method(method) => {
                    member_functions.push(self.member_function(method))
                }
            };
        }
        self.context.pop();
        TypedStruct {
            name: s.name,
            type_params: s
                .type_params
                .map(|v| v.into_iter().map(|tp| self.type_param(tp)).collect()),
            init: initializers,
            stored_properties,
            computed_properties,
            member_functions,
            static_function: vec![],
        }
    }

    fn default_init_if_needed(&self, mut s: TypedStruct) -> TypedStruct {
        let args: Vec<TypedArgDef> = s
            .stored_properties
            .iter()
            .map(|p| {
                TypedArgDef::Value(TypedValueArgDef {
                    label: p.name.clone(),
                    name: p.name.clone(),
                    type_: p.type_.clone(),
                })
            })
            .collect();
        if s.init.is_empty() {
            let struct_type = TypedValueType {
                package: Package { names: vec![] },
                name: s.name.clone(),
                type_args: None,
            };
            s.init.push(TypedInitializer {
                args,
                body: TypedFunBody::Block(TypedBlock {
                    body: s
                        .stored_properties
                        .iter()
                        .map(|p| {
                            TypedStmt::Assignment(TypedAssignmentStmt::Assignment(
                                TypedAssignment {
                                    target: TypedExpr::Member(TypedInstanceMember {
                                        target: Box::new(TypedExpr::Name(TypedName {
                                            name: "self".to_string(),
                                            type_: Some(TypedType::Value(struct_type.clone())),
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
                }),
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

    pub fn initializer_syntax(&mut self, init: InitializerSyntax) -> TypedInitializer {
        TypedInitializer {
            args: init.args.into_iter().map(|a| self.arg_def(a)).collect(),
            body: self.fun_body(init.body),
        }
    }

    pub fn member_function(&mut self, member_function: MethodSyntax) -> TypedMemberFunction {
        let MethodSyntax {
            name,
            args,
            type_params,
            body,
            return_type,
        } = member_function;

        let rt = return_type.map(|r| self.type_(r));
        let fb = body.map(|b| self.fun_body(b));
        let (return_type, body) = match (rt, fb) {
            (Some(rt), Some(TypedFunBody::Expr(fb))) => {
                if rt != fb.type_().unwrap() {
                    eprintln!("Type miss match error");
                    exit(-1)
                } else {
                    (rt, TypedFunBody::Expr(fb))
                }
            }
            (Some(rt), Some(TypedFunBody::Block(fb))) => (rt, TypedFunBody::Block(fb)),
            (Some(rt), None) => exit(-1),
            (None, Some(TypedFunBody::Expr(fb))) => (fb.type_().unwrap(), TypedFunBody::Expr(fb)),
            (None, Some(TypedFunBody::Block(fb))) => (TypedType::unit(), TypedFunBody::Block(fb)),
            (None, None) => {
                eprintln!("Error fun body and return type must be specify.");
                exit(-1)
            }
        };
        TypedMemberFunction {
            name: name,
            args: args.into_iter().map(|a| self.arg_def(a)).collect(),
            type_params: type_params
                .map(|tps| tps.into_iter().map(|p| self.type_param(p)).collect()),
            body: body,
            return_type: return_type.clone(),
            type_: TypedType::Function(Box::new(TypedFunctionType {
                arguments: vec![],
                return_type: return_type,
            })),
        }
    }

    pub fn expr(&mut self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name(n) => TypedExpr::Name(self.name_syntax(n)),
            Expr::Literal(literal) => match literal {
                Literal::Integer { value } => TypedExpr::Literal(TypedLiteral::Integer {
                    value,
                    type_: Some(TypedType::int64()),
                }),
                Literal::FloatingPoint { value } => {
                    TypedExpr::Literal(TypedLiteral::FloatingPoint {
                        value,
                        type_: Some(TypedType::double()),
                    })
                }
                Literal::String { value } => TypedExpr::Literal(TypedLiteral::String {
                    value,
                    type_: Some(TypedType::string()),
                }),
                Literal::Boolean { value } => TypedExpr::Literal(TypedLiteral::Boolean {
                    value,
                    type_: Some(TypedType::bool()),
                }),
                Literal::Null => TypedExpr::Literal(TypedLiteral::NullLiteral { type_: None }),
            },
            Expr::BinOp { left, kind, right } => {
                let left = Box::new(self.expr(*left));
                let right = Box::new(self.expr(*right));
                let type_ = self.resolve_by_binop(&left.type_(), &kind, &right.type_());
                TypedExpr::BinOp(TypedBinOp {
                    left: left,
                    kind: kind,
                    right: right,
                    type_: type_,
                })
            }
            Expr::UnaryOp {
                target,
                prefix,
                kind,
            } => {
                let target = self.expr(*target);
                let type_ = self.resolve_by_unaryop(&target.type_(), &kind);
                TypedExpr::UnaryOp(TypedUnaryOp {
                    target: Box::new(target),
                    prefix: prefix,
                    kind: kind,
                    type_: type_,
                })
            }
            Expr::Subscript(s) => TypedExpr::Subscript(self.subscript_syntax(s)),
            Expr::Member {
                target,
                name,
                is_safe,
            } => {
                let target = self.expr(*target);
                println!("target Expr -> {:?}", target);
                let target_type = target.type_().unwrap();
                let type_ = self.resolve_member_type(&target_type, name.clone());
                TypedExpr::Member(TypedInstanceMember {
                    target: Box::new(target),
                    name,
                    is_safe,
                    type_,
                })
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
                let type_ = if else_body == None {
                    TypedType::noting()
                } else {
                    block.type_().unwrap_or(TypedType::noting())
                };
                TypedExpr::If(TypedIf {
                    condition: Box::new(self.expr(*condition)),
                    body: block,
                    else_body: else_body.map(|b| self.block_with_env(b)),
                    type_: Some(type_),
                })
            }
            Expr::When { .. } => TypedExpr::When,
            Expr::Lambda { .. } => TypedExpr::Lambda,
            Expr::Return(r) => TypedExpr::Return(self.return_syntax(r)),
            Expr::TypeCast { .. } => TypedExpr::TypeCast,
        }
    }

    pub fn name_syntax(&self, n: NameExprSyntax) -> TypedName {
        let NameExprSyntax { name } = n;
        TypedName { name, type_: None }
    }

    pub fn subscript_syntax(&mut self, s: SubscriptSyntax) -> TypedSubscript {
        let target = Box::new(self.expr(*s.target));
        println!("target -> {:?}", target);
        let indexes: Vec<TypedExpr> = s.idx_or_keys.into_iter().map(|i| self.expr(i)).collect();
        println!("indexes -> {:?}", indexes);
        // let type_ = self.context.resolve_subscript(target.type_(), indexes.iter().map(|i|i.type_()).collect());
        TypedSubscript {
            target,
            indexes,
            type_: None,
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
