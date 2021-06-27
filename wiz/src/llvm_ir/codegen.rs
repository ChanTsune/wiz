use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::AddressSpace;
use inkwell::support::{LLVMString};
use std::path::Path;
use either::Either;
use crate::ast::expr::{Expr, CallArg};
use crate::ast::literal::Literal;
use inkwell::types::{StringRadix, AnyTypeEnum, BasicTypeEnum};
use std::process::exit;
use inkwell::values::{AnyValueEnum, BasicValueEnum, CallSiteValue, InstructionValue, PointerValue, AnyValue, GlobalValue, BasicValue};
use crate::ast::decl::Decl;
use crate::ast::type_name::TypeName;
use crate::ast::fun::body_def::FunBody;
use crate::ast::stmt::{Stmt, AssignmentStmt};
use crate::ast::file::File;
use nom::Parser;
use std::iter::Map;
use nom::lib::std::convert::TryFrom;
use std::ffi::CString;
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) local_environments: Vec<HashMap<String, AnyValueEnum<'ctx>>>,
}

impl<'ctx> CodeGen<'ctx> {
    fn get_from_environment(&self, name: String) -> Option<AnyValueEnum<'ctx>> {
        for e in self.local_environments.iter().rev() {
            if let Some(v) = e.get(&*name) {
                return Some(*v)
            }
        }
        match self.module.get_function(&*name) {
            Some(f) => {
                Some(AnyValueEnum::FunctionValue(f))
            }
            None => None
        }
    }

    fn set_to_environment(&mut self, name: String, value: AnyValueEnum<'ctx>) {
        let len = self.local_environments.len();
        self.local_environments[len - 1].insert(name, value);
    }

    fn push_environment(&mut self) {
        self.local_environments.push(HashMap::new())
    }

    fn pop_environment(&mut self) {
        self.local_environments.pop();
    }
    /**
    * Generate main function as entry point.
    */
    pub fn initialize(&self) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        let sum_function = self.module.get_function("sum").unwrap();
        let x = self.context.i64_type().const_int(1, false);
        let y = self.context.i64_type().const_int(2, false);

        let sum = self.builder.build_call(sum_function, &[x.into(), y.into()], "sum");


        let put_function = self.module.get_function("puts").unwrap();

        self.builder.build_call(put_function, &[sum.try_as_basic_value().left().unwrap().into()], "_");

        self.builder.build_return(None);
    }

    pub fn expr(&self, e: Expr) -> AnyValueEnum<'ctx> {
        println!("{:?}", e);
        match e {
            Expr::Name { name } => {
                self.get_from_environment(name).unwrap()
            }
            Expr::Literal { literal } => {
                match literal {
                    Literal::IntegerLiteral { value } => {
                        let i: u64 = value.parse().unwrap();
                        let i64_type = self.context.i64_type();
                        i64_type.const_int(i, false).as_any_value_enum()
                    }
                    Literal::FloatingPointLiteral { value } => {
                        let f: f64 = value.parse().unwrap();
                        let f64_type = self.context.f64_type();
                        f64_type.const_float(f).as_any_value_enum()
                    }
                    Literal::StringLiteral { value } => unsafe {
                        let str = self.builder.build_global_string(value.as_ref(), value.as_str());
                        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
                        let str = self.builder.build_bitcast(str.as_pointer_value(), i8_ptr_type, value.as_str());
                        str.as_any_value_enum()

                    }
                    Literal::BooleanLiteral { value } => {
                        let b: bool = value.parse().unwrap();
                        let i8_type = self.context.i8_type();
                        i8_type.const_int(if b {1} else {0}, false).as_any_value_enum()
                    }
                    Literal::NullLiteral => {
                        println!("Literall::Null");
                        exit(-1)
                    }
                }
            }
            Expr::BinOp { left, kind, right } => {
                let lft = self.expr(*left);
                let rit = self.expr(*right);
                match (lft, rit) {
                    (AnyValueEnum::IntValue(left), AnyValueEnum::IntValue(right)) => {
                        match &*kind {
                            "+" => {
                                let v = self.builder.build_int_add(left, right, "sum");
                                v.as_any_value_enum()
                            },
                            "-" => {
                                let v = self.builder.build_int_sub(left, right,"sub");
                                v.as_any_value_enum()
                            },
                            "*" => {
                                let v = self.builder.build_int_mul(left, right, "mul");
                                v.as_any_value_enum()
                            },
                            "/" => {
                                let v = self.builder.build_int_signed_div(left, right, "sdiv");
                                v.as_any_value_enum()
                            },
                            "%" => {
                                let v = self.builder.build_int_signed_rem(left, right, "srem");
                                v.as_any_value_enum()
                            },
                            _ => {
                                exit(-1)
                            }
                        }
                    },
                    (AnyValueEnum::FloatValue(left), AnyValueEnum::FloatValue(right)) => {
                        match &*kind {
                            "+" => {
                                let v = self.builder.build_float_add(left, right, "sum");
                                v.as_any_value_enum()
                            },
                            "-" => {
                                let v = self.builder.build_float_sub(left, right, "sub");
                                v.as_any_value_enum()
                            },
                            "*" => {
                                let v = self.builder.build_float_mul(left, right, "sub");
                                v.as_any_value_enum()
                            },
                            "/" => {
                                let v = self.builder.build_float_div(left, right, "sub");
                                v.as_any_value_enum()
                            },
                            "%" => {
                                let v = self.builder.build_float_rem(left, right, "sub");
                                v.as_any_value_enum()
                            },
                            _ => {
                                exit(-1)
                            }
                        }
                    }
                    (AnyValueEnum::PointerValue(left), AnyValueEnum::PointerValue(right)) => {
                        self.builder.build_load(left, "left");
                        self.builder.build_load(right, "right").as_any_value_enum()
                    }
                    (r, l) => {
                        println!("{:?},{:?}", r, l);
                        exit(-5)
                    }
                }
            }
            Expr::UnaryOp { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Subscript { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::List { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Tuple { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Dict { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::StringBuilder { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Call { target, args, tailing_lambda } => {
                let target = self.expr(*target);
                println!("{:?}", &args);
                let args = args.into_iter().map(|arg|{ self.expr(*arg.arg) });
                let args: Vec<BasicValueEnum> = args.filter_map(|arg|{
                    BasicValueEnum::try_from(arg).ok()
                }).collect();
                match target {
                    AnyValueEnum::FunctionValue(function) => {
                        let bv = self.builder.build_call(function, &args, "f_call").try_as_basic_value();
                        match bv {
                            Either::Left(vb) => {
                                AnyValueEnum::from(vb)
                            }
                            Either::Right(iv) => {
                                AnyValueEnum::from(iv)
                            }
                        }
                    }
                    AnyValueEnum::PointerValue(p) => {
                        println!("{:?}", p);
                        exit(-12)
                    }
                    _ => {
                        exit(-1)
                    }
                }
            }
            Expr::If { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::When { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Lambda { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::Return { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
            Expr::TypeCast { .. } => {
                println!("{:?}", e);
                exit(-1)
            }
        }
    }

    pub fn decl(&mut self, d: Decl) -> AnyValueEnum<'ctx> {
        println!("{:?}", &d);
        match d {
            Decl::Var { is_mut, name, type_, value } => {
                let value = self.expr(value);
                match value {
                    AnyValueEnum::IntValue(i) => {
                        let i64_type = self.context.i64_type();
                        let ptr = self.builder.build_alloca(i64_type, &*name);
                        self.set_to_environment(name, ptr.as_any_value_enum());
                        self.builder.build_store(ptr, i).as_any_value_enum()
                    }
                    AnyValueEnum::FloatValue(f) => {
                        let f64_type = self.context.f64_type();
                        let ptr = self.builder.build_alloca(f64_type, &*name);
                        self.set_to_environment(name, ptr.as_any_value_enum());
                        self.builder.build_store(ptr, f).as_any_value_enum()
                    }
                    _ => {
                        exit(-1)
                    }
                }
            }
            Decl::Fun { modifiers, name, arg_defs, return_type, body } => {
                let args: Vec<BasicTypeEnum<'ctx>> = arg_defs.into_iter()
                    .map(|a|{type_name_to_type(self.context,&*a.type_name.name)})
                    .map(|a|{
                        println!("{:?}", &a);
                        BasicTypeEnum::try_from(a).unwrap()}
                    ).collect();
                let return_type_name: &str = &*return_type.name; // NOTE: for debug
                let return_type = type_name_to_type(self.context, &*return_type.name);
                if let Some(body) = body {
                    self.push_environment();
                    let func = match return_type {
                        // AnyTypeEnum::ArrayType(_) => {}
                        // AnyTypeEnum::FloatType(_) => {}
                        // AnyTypeEnum::FunctionType(_) => {}
                        // AnyTypeEnum::IntType(_) => {}
                        // AnyTypeEnum::PointerType(_) => {}
                        // AnyTypeEnum::StructType(_) => {}
                        // AnyTypeEnum::VectorType(_) => {}
                        AnyTypeEnum::VoidType(void_type) => {
                            let fn_type = void_type.fn_type(&args, false);
                            let function = self.module.add_function(&*name, fn_type,None);
                            let basic_block = self.context.append_basic_block(function, "entry");
                            self.builder.position_at_end(basic_block);
                            match body {
                                FunBody::Expr { expr } => {
                                    self.expr(expr);
                                }
                                FunBody::Block { block } => {
                                    for stmt in block.body {
                                        self.stmt(stmt);
                                    }
                                }
                            };
                            self.builder.build_return(None);
                            function
                        }
                        _ => {
                            println!("{}", return_type_name);
                            exit(-1)
                        }
                    };
                    self.pop_environment();
                    AnyValueEnum::from(func)
                } else {
                    let func = match return_type {
                        // AnyTypeEnum::ArrayType(_) => {}
                        // AnyTypeEnum::FloatType(_) => {}
                        // AnyTypeEnum::FunctionType(_) => {}
                        // AnyTypeEnum::IntType(_) => {}
                        // AnyTypeEnum::PointerType(_) => {}
                        // AnyTypeEnum::StructType(_) => {}
                        // AnyTypeEnum::VectorType(_) => {}
                        AnyTypeEnum::VoidType(void_type) => {
                            let fn_type = void_type.fn_type(&args, false);
                            self.module.add_function(&*name, fn_type,None)
                        }
                        _ => {
                            println!("{}", return_type_name);
                            exit(-1)
                        }
                    };
                    AnyValueEnum::from(func)
                }
            }
            Decl::Struct { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
            Decl::Class { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
            Decl::Enum { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
            Decl::Protocol { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
            Decl::Extension { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
        }
    }

    pub fn stmt(&mut self, s:Stmt) -> AnyValueEnum {
        match s {
            Stmt::Decl { decl } => { self.decl(decl) }
            Stmt::Assignment(a) => { self.assignment_stmt(a) }
            Stmt::Loop(_) => { exit(-9) }
            Stmt::Expr { expr } => { self.expr(expr) }
        }
    }

    pub fn assignment_stmt(&mut self, assignment: AssignmentStmt) -> AnyValueEnum<'ctx> {
        let value = self.expr(assignment.value);
        match value {
            AnyValueEnum::IntValue(i) => {
                let target = self.get_from_environment(assignment.target).unwrap();
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, i))
                }
                exit(-3)
            }
            AnyValueEnum::FloatValue(f) => {
                let target = self.get_from_environment(assignment.target).unwrap();
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, f))
                }
                exit(-3)
            }
            _ => {
                exit(-3)
            }
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::ArrayValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            // AnyValueEnum::PointerValue(_) => {}
            // AnyValueEnum::StructValue(_) => {}
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
        }
    }

    pub fn file(&mut self, f:File) {
        for d in f.body {
            self.decl(d);
        }
    }

    pub fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
    /**
    * Write the LLVM IR to a file in the path.
    */
    pub fn print_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), LLVMString> {
        self.module.print_to_file(path)
    }
}

fn type_name_to_type<'ctx>(context: &'ctx Context, type_name: &str) -> AnyTypeEnum<'ctx> {
    match type_name {
        "Unit" => {
            AnyTypeEnum::from(context.void_type())
        }
        "Int32" | "UInt32" => {
            AnyTypeEnum::from(context.i32_type())
        }
        "Float" => {
            AnyTypeEnum::from(context.f32_type())
        }
        "Double" => {
            AnyTypeEnum::from(context.f64_type())
        }
        "String" => {
            AnyTypeEnum::from(context.i8_type().ptr_type(AddressSpace::Generic))
        }
        _ => {
            AnyTypeEnum::from(context.void_type())
        }
    }
}
