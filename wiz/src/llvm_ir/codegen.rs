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
use inkwell::types::{StringRadix, AnyTypeEnum};
use std::process::exit;
use inkwell::values::{AnyValueEnum, BasicValueEnum, CallSiteValue, InstructionValue, PointerValue, AnyValue, GlobalValue, BasicValue};
use crate::ast::decl::Decl;
use crate::ast::type_name::TypeName;
use crate::ast::fun::body_def::FunBody;
use crate::ast::stmt::Stmt;
use crate::ast::file::File;
use nom::Parser;
use std::iter::Map;
use nom::lib::std::convert::TryFrom;
use std::ffi::CString;

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
}

impl<'ctx> CodeGen<'ctx> {
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

    pub fn builtin_print(&self) -> inkwell::values::FunctionValue {
        let i32_type = self.context.i32_type();
        let i8p_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let fn_type = i32_type.fn_type(&[i8p_type.into()], false);
        self.module.add_function("puts", fn_type, Some(Linkage::External))
    }

    pub fn expr(&self, e: Expr) -> AnyValueEnum {
        println!("{:?}", e);
        match e {
            Expr::Name { name } => {
                match self.module.get_function(&*name) {
                    Some(f) => {
                        AnyValueEnum::from(f)
                    }
                    None => {
                        println!("Expr::Name");
                        exit(-1)
                    }
                }
            }
            Expr::Literal { literal } => {
                match literal {
                    Literal::IntegerLiteral { value } => {
                        let i: u64 = value.parse().unwrap();
                        let i64_type = self.context.i64_type();
                        let i = i64_type.const_int(i, false);
                        AnyValueEnum::from(i)
                    }
                    Literal::FloatingPointLiteral { .. } => {
                        println!("Literal::FloatingPoint");
                        exit(-1)
                    }
                    Literal::StringLiteral { value } => unsafe {
                        let str = self.builder.build_global_string(value.as_ref(), value.as_str());
                        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
                        let str = self.builder.build_bitcast(str.as_pointer_value(), i8_ptr_type, value.as_str());
                        str.as_any_value_enum()

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
                let v = self.builder.build_int_add(lft.into_int_value(), rit.into_int_value(), "sum");

                AnyValueEnum::from(v)
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
                    _ => {
                        // println!("{:?}", e);
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

    pub fn decl(&self, d: Decl) -> AnyValueEnum {
        match d {
            Decl::Var { .. } => {
                println!("{:?}", d);
                exit(-1)
            }
            Decl::Fun { modifiers, name, arg_defs, return_type, body } => {
                let return_type_name: &str = &*return_type.name;
                match return_type_name {
                    "Unit" => {
                        let void_type = self.context.void_type();
                        let fn_type = void_type.fn_type(&[], false);
                        let function = self.module.add_function(&*name, fn_type,None);
                        let basic_block = self.context.append_basic_block(function, "entry");
                        self.builder.position_at_end(basic_block);
                        match body {
                            None => {}
                            Some(FunBody::Expr { expr }) => {

                                self.expr(expr);
                            }
                            Some(FunBody::Block { block }) => {
                                for stmt in block.body {
                                    self.stmt(stmt);
                                }
                            }
                        };
                        self.builder.build_return(None);
                        AnyValueEnum::from(function)
                    }
                    _ => {
                        println!("{}", return_type_name);
                        exit(-1)
                    }
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

    pub fn stmt(&self, s:Stmt) -> AnyValueEnum {
        match s {
            Stmt::Decl { decl } => { self.decl(decl) }
            Stmt::Expr { expr } => { self.expr(expr) }
        }
    }

    pub fn file(&self, f:File) {
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

