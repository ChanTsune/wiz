use crate::ast::block::Block;
use crate::ast::decl::{Decl, FunSyntax, VarSyntax};
use crate::ast::expr::{Expr, ReturnSyntax};
use crate::ast::file::{FileSyntax, WizFile};
use crate::ast::fun::body_def::FunBody;
use crate::ast::literal::Literal;
use crate::ast::stmt::{AssignmentStmt, LoopStmt, Stmt};
use crate::ast::type_name::TypeName;
use crate::high_level_ir::typed_decl::{TypedArgDef, TypedDecl, TypedFun, TypedFunBody, TypedVar};
use crate::high_level_ir::typed_expr::{
    TypedCallArg, TypedExpr, TypedIf, TypedLiteral, TypedName, TypedReturn,
};
use crate::high_level_ir::typed_file::TypedFile;
use crate::high_level_ir::typed_stmt::{
    TypedAssignmentStmt, TypedBlock, TypedForStmt, TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use crate::high_level_ir::typed_type::{Package, TypedType};
use std::collections::HashMap;
use std::option::Option::Some;
use std::process::exit;

pub mod typed_decl;
pub mod typed_expr;
pub mod typed_file;
pub mod typed_stmt;
pub mod typed_type;

pub struct Ast2HLIR {
    name_environment: Vec<HashMap<String, TypedType>>,
    type_environment: HashMap<String, TypedType>,
    decl_environment: HashMap<String, TypedDecl>,
}

impl Ast2HLIR {
    pub fn new() -> Self {
        let mut builtin_types = HashMap::new();
        builtin_types.insert(String::from("Int8"), TypedType::int8());
        builtin_types.insert(String::from("Int16"), TypedType::int16());
        builtin_types.insert(String::from("Int32"), TypedType::int32());
        builtin_types.insert(String::from("Int64"), TypedType::int64());
        builtin_types.insert(
            String::from("UInt8"),
            TypedType {
                package: Package { names: vec![] },
                name: "UInt8".to_string(),
            },
        );
        builtin_types.insert(
            String::from("UInt16"),
            TypedType {
                package: Package { names: vec![] },
                name: "UInt16".to_string(),
            },
        );
        builtin_types.insert(
            String::from("UInt32"),
            TypedType {
                package: Package { names: vec![] },
                name: "UInt32".to_string(),
            },
        );
        builtin_types.insert(
            String::from("UInt64"),
            TypedType {
                package: Package { names: vec![] },
                name: "UInt64".to_string(),
            },
        );
        builtin_types.insert(
            String::from("String"),
            TypedType {
                package: Package { names: vec![] },
                name: "String".to_string(),
            },
        );
        builtin_types.insert(String::from("Noting"), TypedType::noting());
        builtin_types.insert(
            String::from("Unit"),
            TypedType {
                package: Package { names: vec![] },
                name: "Unit".to_string(),
            },
        );
        Ast2HLIR {
            name_environment: vec![HashMap::new()],
            type_environment: builtin_types,
            decl_environment: HashMap::new(),
        }
    }

    pub fn preload_types(&mut self, ast: WizFile) {
        for decl in ast.syntax.body {
            match decl {
                Decl::Var(v) => {
                    let var = self.var_syntax(v);
                    self.put_type_by(var.name, &var.type_.unwrap())
                }
                Decl::Fun(f) => {
                    self.put_type_by(f.name, &self.resolve_by_type_name(f.return_type).unwrap())
                }
                Decl::Struct {} => {}
                Decl::Class {} => {}
                Decl::Enum {} => {}
                Decl::Protocol {} => {}
                Decl::Extension {} => {}
            }
        }
    }

    fn get_type_by(&self, name: String) -> Option<TypedType> {
        for env in self.name_environment.iter().rev() {
            if let Some(t) = env.get(&*name) {
                return Some(t.clone());
            }
        }
        None
    }

    fn put_type_by(&mut self, name: String, type_: &TypedType) {
        let last_index = self.name_environment.len() - 1;
        self.name_environment[last_index].insert(name, type_.clone());
    }

    fn push_name_environment(&mut self) {
        self.name_environment.push(HashMap::new());
    }

    fn pop_name_environment(&mut self) {
        self.name_environment.pop();
    }

    fn resolve_by_type_name(&self, type_name: TypeName) -> Option<TypedType> {
        self.type_environment.get(&*type_name.name).map(|a| {
            println!("TypeResolver :: {:?}", a);
            a.clone()
        })
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

    pub fn file(&mut self, f: FileSyntax) -> TypedFile {
        TypedFile {
            body: f.body.into_iter().map(|d| self.decl(d)).collect(),
        }
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
        TypedAssignmentStmt {
            target: a.target,
            value: self.expr(a.value),
        }
    }

    pub fn loop_stmt(&mut self, l: LoopStmt) -> TypedLoopStmt {
        match l {
            LoopStmt::While { condition, block } => TypedLoopStmt::While(TypedWhileLoopStmt {
                condition: self.expr(condition),
                block: self.block(block),
            }),
            LoopStmt::For {
                values,
                iterator,
                block,
            } => TypedLoopStmt::For(TypedForStmt {
                values: values,
                iterator: self.expr(iterator),
                block: self.block(block),
            }),
        }
    }

    pub fn decl(&mut self, d: Decl) -> TypedDecl {
        match d {
            Decl::Var(v) => TypedDecl::Var(self.var_syntax(v)),
            Decl::Fun(f) => TypedDecl::Fun(self.fun_syntax(f)),
            Decl::Struct { .. } => TypedDecl::Struct,
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
                let var_type = self.resolve_by_type_name(tn.clone());
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
                if let Some(tt) = self.resolve_by_type_name(t.clone()) {
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
        self.put_type_by(v.name.clone(), &type_);
        TypedVar {
            is_mut: v.is_mut,
            name: v.name,
            type_: Some(type_),
            value: expr,
        }
    }

    pub fn fun_syntax(&mut self, f: FunSyntax) -> TypedFun {
        let f = TypedFun {
            modifiers: f.modifiers,
            name: f.name,
            arg_defs: f
                .arg_defs
                .into_iter()
                .map(|a| TypedArgDef {
                    label: a.label,
                    name: a.name,
                    type_: self.resolve_by_type_name(a.type_name).unwrap(),
                })
                .collect(),
            body: f.body.map(|b| match b {
                FunBody::Block { block } => TypedFunBody::Block(self.block(block)),
                FunBody::Expr { expr } => TypedFunBody::Expr(self.expr(expr)),
            }),
            return_type: self.resolve_by_type_name(f.return_type).unwrap(),
        };
        self.put_type_by(f.name.clone(), &f.return_type);
        f
    }

    pub fn expr(&mut self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name { name } => TypedExpr::Name(TypedName {
                name: name.clone(),
                type_: self.get_type_by(name),
            }),
            Expr::Literal { literal } => match literal {
                Literal::IntegerLiteral { value } => TypedExpr::Literal(TypedLiteral::Integer {
                    value,
                    type_: TypedType {
                        package: Package { names: vec![] },
                        name: "Int64".to_string(),
                    },
                }),
                Literal::FloatingPointLiteral { value } => {
                    TypedExpr::Literal(TypedLiteral::FloatingPoint {
                        value,
                        type_: TypedType {
                            package: Package { names: vec![] },
                            name: "Double".to_string(),
                        },
                    })
                }
                Literal::StringLiteral { value } => TypedExpr::Literal(TypedLiteral::String {
                    value,
                    type_: TypedType {
                        package: Package { names: vec![] },
                        name: "String".to_string(),
                    },
                }),
                Literal::BooleanLiteral { value } => TypedExpr::Literal(TypedLiteral::Boolean {
                    value,
                    type_: TypedType {
                        package: Package { names: vec![] },
                        name: "Bool".to_string(),
                    },
                }),
                Literal::NullLiteral => TypedExpr::Literal(TypedLiteral::NullLiteral {
                    type_: TypedType {
                        package: Package { names: vec![] },
                        name: "Option<*>".to_string(),
                    },
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
            Expr::List { .. } => TypedExpr::List,
            Expr::Tuple { .. } => TypedExpr::Tuple,
            Expr::Dict { .. } => TypedExpr::Dict,
            Expr::StringBuilder { .. } => TypedExpr::StringBuilder,
            Expr::Call {
                target,
                args,
                tailing_lambda,
            } => {
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
                TypedExpr::Call {
                    target: Box::new(e),
                    args: args,
                    type_: return_type,
                }
            }
            Expr::If {
                condition,
                body,
                else_body,
            } => {
                let block = self.block(body);
                let type_ = block.type_();
                TypedExpr::If(TypedIf {
                    condition: Box::new(self.expr(*condition)),
                    body: block,
                    else_body: else_body.map(|b| self.block(b)),
                    type_: type_,
                })
            }
            Expr::When { .. } => TypedExpr::When,
            Expr::Lambda { .. } => TypedExpr::Lambda,
            Expr::Return(r) => TypedExpr::Return(self.return_syntax(r)),
            Expr::TypeCast { .. } => TypedExpr::TypeCast,
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
        self.push_name_environment();
        let b = TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        };
        self.pop_name_environment();
        b
    }
}
