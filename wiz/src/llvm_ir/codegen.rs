use crate::middle_level_ir::ml_decl::{MLDecl, MLFun, MLStruct, MLVar};
use crate::middle_level_ir::ml_expr::{
    MLBinOp, MLBinopKind, MLCall, MLExpr, MLIf, MLLiteral, MLMember, MLReturn, MLUnaryOp,
};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLAssignmentStmt, MLBlock, MLLoopStmt, MLStmt};
use crate::middle_level_ir::ml_type::MLType;
use either::Either;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use nom::lib::std::convert::TryFrom;
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use crate::utils::stacked_hash_map::StackedHashMap;

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
    pub(crate) local_environments: StackedHashMap<String, AnyValueEnum<'ctx>>,
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    fn get_from_environment(&self, name: String) -> Option<AnyValueEnum<'ctx>> {
        match self.local_environments.get(&name) {
            Some(v) => Some(*v),
            None =>         match self.module.get_function(&*name) {
                Some(f) => Some(AnyValueEnum::FunctionValue(f)),
                None => None,
            }
        }
    }

    fn set_to_environment(&mut self, name: String, value: AnyValueEnum<'ctx>) {
        self.local_environments.insert(name, value);
    }

    fn push_environment(&mut self) {
        self.local_environments.push(HashMap::new())
    }

    fn pop_environment(&mut self) {
        self.local_environments.pop();
    }

    fn get_struct_field_index_by_name(&self, m: MLType, n: String) -> u32 {
        0
    }

    pub fn expr(&mut self, e: MLExpr) -> AnyValueEnum<'ctx> {
        println!("{:?}", e);
        match e {
            MLExpr::Name(n) => self.get_from_environment(n.name).unwrap(),
            MLExpr::Literal(literal) => self.literal(literal),
            MLExpr::PrimitiveBinOp(b) => self.binop(b),
            MLExpr::PrimitiveUnaryOp(u) => self.unaryop(u),
            MLExpr::Call(c) => self.call(c),
            MLExpr::Member(m) => self.member(m),
            MLExpr::If(i) => self.if_expr(i),
            MLExpr::When => exit(-1),
            MLExpr::Return(r) => self.return_expr(r),
            MLExpr::TypeCast => exit(-1),
        }
    }

    pub fn literal(&self, l: MLLiteral) -> AnyValueEnum<'ctx> {
        match l {
            MLLiteral::Integer { value, type_ } => {
                let i: u64 = value.parse().unwrap();
                let type_ = type_.into_value_type();
                let int_type = match &*(type_.name) {
                    "Int8" | "UInt8" => self.context.i8_type(),
                    "Int16" | "UInt16" => self.context.i16_type(),
                    "Int32" | "UInt32" => self.context.i32_type(),
                    "Int64" | "UInt64" => self.context.i64_type(),
                    _ => {
                        eprintln!("Invalid MLIR Literal {:}", type_.name);
                        exit(-1)
                    }
                };
                int_type.const_int(i, false).as_any_value_enum()
            }
            MLLiteral::FloatingPoint { value, type_ } => {
                let f: f64 = value.parse().unwrap();
                let type_ = type_.into_value_type();
                let float_type = match &*(type_.name) {
                    "Float" => self.context.f32_type(),
                    "Double" => self.context.f64_type(),
                    _ => {
                        eprintln!("Invalid MLIR Literal {:}", type_.name);
                        exit(-1)
                    }
                };
                float_type.const_float(f).as_any_value_enum()
            }
            MLLiteral::String { value, type_ } => unsafe {
                let str = self
                    .builder
                    .build_global_string(value.as_ref(), value.as_str());
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
                let str =
                    self.builder
                        .build_bitcast(str.as_pointer_value(), i8_ptr_type, value.as_str());
                str.as_any_value_enum()
            },
            MLLiteral::Boolean { value, type_ } => {
                let b: bool = value.parse().unwrap();
                let bool_type = self.context.bool_type();
                bool_type
                    .const_int(if b { 1 } else { 0 }, false)
                    .as_any_value_enum()
            }
            MLLiteral::Null { .. } => {
                println!("Literall::Null");
                exit(-2)
            }
            MLLiteral::Struct { type_ } => {
                let type_ = type_.into_value_type();
                let struct_type = self.module.get_struct_type(&*type_.name);
                let struct_type = struct_type.unwrap();
                struct_type.const_zero().as_any_value_enum()
            }
        }
    }

    pub fn call(&mut self, c: MLCall) -> AnyValueEnum<'ctx> {
        let target = self.expr(*c.target);
        println!("{:?}", &(c.args));
        let args = c.args.into_iter().map(|arg| {
            // TODO: change stored pointer value load method!
            if arg.arg.type_().into_value_type().name != String::from("String") {
                let e = self.expr(arg.arg);
                self.load_if_pointer_value(e)
            } else {
                self.expr(arg.arg)
            }
        });
        let args: Vec<BasicValueEnum> = args
            .filter_map(|arg| BasicValueEnum::try_from(arg).ok())
            .collect();
        match target {
            AnyValueEnum::FunctionValue(function) => {
                let bv = self
                    .builder
                    .build_call(function, &args, "f_call")
                    .try_as_basic_value();
                match bv {
                    Either::Left(vb) => AnyValueEnum::from(vb),
                    Either::Right(iv) => AnyValueEnum::from(iv),
                }
            }
            AnyValueEnum::PointerValue(p) => {
                println!("{:?}", p);
                exit(-12)
            }
            _ => exit(-12),
        }
    }

    pub fn binop(&mut self, b: MLBinOp) -> AnyValueEnum<'ctx> {
        let lft = self.expr(*b.left);
        let rit = self.expr(*b.right);
        let lft = self.load_if_pointer_value(lft);
        let rit = self.load_if_pointer_value(rit);

        match (lft, rit) {
            (AnyValueEnum::IntValue(left), AnyValueEnum::IntValue(right)) => match b.kind {
                MLBinopKind::Plus => {
                    let v = self.builder.build_int_add(left, right, "sum");
                    v.as_any_value_enum()
                }
                MLBinopKind::Minus => {
                    let v = self.builder.build_int_sub(left, right, "sub");
                    v.as_any_value_enum()
                }
                MLBinopKind::Mul => {
                    let v = self.builder.build_int_mul(left, right, "mul");
                    v.as_any_value_enum()
                }
                MLBinopKind::Div => {
                    let v = self.builder.build_int_signed_div(left, right, "sdiv");
                    v.as_any_value_enum()
                }
                MLBinopKind::Mod => {
                    let v = self.builder.build_int_signed_rem(left, right, "srem");
                    v.as_any_value_enum()
                }
                MLBinopKind::Equal => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, left, right, "eq");
                    v.as_any_value_enum()
                }
                MLBinopKind::GrateThanEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SGE, left, right, "gte");
                    v.as_any_value_enum()
                }
                MLBinopKind::GrateThan => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SGT, left, right, "gt");
                    v.as_any_value_enum()
                }
                MLBinopKind::LessThanEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SLE, left, right, "lte");
                    v.as_any_value_enum()
                }
                MLBinopKind::LessThan => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SLT, left, right, "lt");
                    v.as_any_value_enum()
                }
                MLBinopKind::NotEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::NE, left, right, "neq");
                    v.as_any_value_enum()
                }
            },
            (AnyValueEnum::FloatValue(left), AnyValueEnum::FloatValue(right)) => match b.kind {
                MLBinopKind::Plus => {
                    let v = self.builder.build_float_add(left, right, "sum");
                    v.as_any_value_enum()
                }
                MLBinopKind::Minus => {
                    let v = self.builder.build_float_sub(left, right, "sub");
                    v.as_any_value_enum()
                }
                MLBinopKind::Mul => {
                    let v = self.builder.build_float_mul(left, right, "mul");
                    v.as_any_value_enum()
                }
                MLBinopKind::Div => {
                    let v = self.builder.build_float_div(left, right, "div");
                    v.as_any_value_enum()
                }
                MLBinopKind::Mod => {
                    let v = self.builder.build_float_rem(left, right, "rem");
                    v.as_any_value_enum()
                }
                MLBinopKind::Equal => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OEQ, left, right, "eq");
                    v.as_any_value_enum()
                }
                MLBinopKind::GrateThanEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OGE, left, right, "gte");
                    v.as_any_value_enum()
                }
                MLBinopKind::GrateThan => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OGT, left, right, "gt");
                    v.as_any_value_enum()
                }
                MLBinopKind::LessThanEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OLE, left, right, "lte");
                    v.as_any_value_enum()
                }
                MLBinopKind::LessThan => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OLT, left, right, "lt");
                    v.as_any_value_enum()
                }
                MLBinopKind::NotEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::ONE, left, right, "neq");
                    v.as_any_value_enum()
                }
            },
            (r, l) => {
                println!("{:?},{:?}", r, l);
                exit(-5)
            }
        }
    }

    pub fn unaryop(&self, u: MLUnaryOp) -> AnyValueEnum<'ctx> {
        println!("Unsupported unaryop {:?}", &u);
        exit(-1)
    }

    pub fn member(&mut self, m: MLMember) -> AnyValueEnum<'ctx> {
        let target = self.expr(*m.target);
        let field_index = self.get_struct_field_index_by_name(m.type_, m.name);
        let ep = self
            .builder
            .build_struct_gep(target.into_pointer_value(), field_index, "struct_gep")
            .unwrap();
        ep.as_any_value_enum()
    }

    pub fn if_expr(&mut self, i: MLIf) -> AnyValueEnum<'ctx> {
        let condition = i.condition;
        let body = i.body;
        let else_body = i.else_body;
        match else_body {
            None => {
                let if_block = self
                    .context
                    .append_basic_block(self.current_function.unwrap(), "if");
                let after_if_block = self
                    .context
                    .append_basic_block(self.current_function.unwrap(), "else");
                let cond = self.expr(*condition);
                self.builder.build_conditional_branch(
                    cond.into_int_value(),
                    if_block,
                    after_if_block,
                );
                self.builder.position_at_end(if_block);
                for stmt in body.body {
                    self.stmt(stmt);
                }
                self.builder.position_at_end(after_if_block);

                self.context
                    .i64_type()
                    .const_int(0, false)
                    .as_any_value_enum() // mean Void value
            }
            Some(else_body) => {
                let i64_type = self.context.i64_type();
                let if_block = self
                    .context
                    .append_basic_block(self.current_function.unwrap(), "if");
                let else_block = self
                    .context
                    .append_basic_block(self.current_function.unwrap(), "else");
                let after_if_block = self
                    .context
                    .append_basic_block(self.current_function.unwrap(), "after_if");
                let cond = self.expr(*condition);
                self.builder
                    .build_conditional_branch(cond.into_int_value(), if_block, else_block);
                self.builder.position_at_end(if_block);
                let stmt_last_expr = self.block(body);
                self.builder.build_unconditional_branch(after_if_block);
                self.builder.position_at_end(else_block);
                let else_stmt_last_expr = self.block(else_body);
                self.builder.build_unconditional_branch(after_if_block);
                self.builder.position_at_end(after_if_block);
                match (
                    BasicValueEnum::try_from(stmt_last_expr),
                    BasicValueEnum::try_from(else_stmt_last_expr),
                ) {
                    (Ok(if_), Ok(else_)) => {
                        let if_value = self.builder.build_phi(i64_type, "if_value");
                        if_value.add_incoming(&[(&if_, if_block), (&else_, else_block)]);
                        if_value.as_any_value_enum()
                    }
                    _ => i64_type.const_int(0, false).as_any_value_enum(),
                }
            }
        }
    }

    pub fn return_expr(&mut self, r: MLReturn) -> AnyValueEnum<'ctx> {
        let v = match r.value {
            Some(e) => match BasicValueEnum::try_from(self.expr(*e)) {
                Ok(b) => Some(b),
                Err(_) => None,
            },
            None => None,
        };

        AnyValueEnum::from(self.builder.build_return(match &v {
            None => None,
            Some(b) => Some(b),
        }))
    }

    pub fn block(&mut self, b: MLBlock) -> AnyValueEnum<'ctx> {
        let i64_type = self.context.i64_type(); // Void
        let last_index = b.body.len() - 1;
        for (i, stmt) in b.body.into_iter().enumerate() {
            if i == last_index {
                return self.stmt(stmt);
            } else {
                self.stmt(stmt)
            };
        }
        AnyValueEnum::from(i64_type.const_int(0, false))
    }

    pub fn load_if_pointer_value(&self, v: AnyValueEnum<'ctx>) -> AnyValueEnum<'ctx> {
        if v.is_pointer_value() {
            let p = v.into_pointer_value();
            self.builder.build_load(p, "v").as_any_value_enum()
        } else {
            v
        }
    }

    pub fn decl(&mut self, d: MLDecl) -> AnyValueEnum<'ctx> {
        println!("{:?}", &d);
        match d {
            MLDecl::Var(v) => self.var(v),
            MLDecl::Fun(f) => self.fun(f),
            MLDecl::Struct(s) => self.struct_(s),
        }
    }

    pub fn var(&mut self, v: MLVar) -> AnyValueEnum<'ctx> {
        let MLVar {
            is_mute,
            name,
            type_,
            value,
        } = v;
        let value = self.expr(value);
        match value {
            AnyValueEnum::IntValue(i) => {
                let int_type = i.get_type();
                let ptr = self.builder.build_alloca(int_type, &*name);
                self.set_to_environment(name, ptr.as_any_value_enum());
                self.builder.build_store(ptr, i).as_any_value_enum()
            }
            AnyValueEnum::FloatValue(f) => {
                let float_type = f.get_type();
                let ptr = self.builder.build_alloca(float_type, &*name);
                self.set_to_environment(name, ptr.as_any_value_enum());
                self.builder.build_store(ptr, f).as_any_value_enum()
            }
            AnyValueEnum::StructValue(s) => {
                let struct_type = s.get_type();
                let ptr = self.builder.build_alloca(struct_type, &*name);
                self.set_to_environment(name, ptr.as_any_value_enum());
                self.builder.build_store(ptr, s).as_any_value_enum()
            }
            _ => exit(-14),
        }
    }

    pub fn fun(&mut self, f: MLFun) -> AnyValueEnum<'ctx> {
        let MLFun {
            modifiers,
            name,
            arg_defs,
            return_type,
            body,
        } = f;
        let args: Vec<BasicTypeEnum<'ctx>> = arg_defs
            .iter()
            .map(|a| {
                let a = a.type_.clone();
                self.type_name_to_type(&*a.into_value_type().name)
            })
            .map(|a| {
                println!("{:?}", &a);
                BasicTypeEnum::try_from(a).unwrap()
            })
            .collect();
        let return_type_name: &str = &*return_type.into_value_type().name; // NOTE: for debug
        let return_type = self.type_name_to_type(return_type_name);
        if let Some(body) = body {
            self.push_environment();
            let func = match return_type {
                // AnyTypeEnum::ArrayType(_) => {}
                // AnyTypeEnum::FloatType(_) => {}
                // AnyTypeEnum::FunctionType(_) => {}
                AnyTypeEnum::IntType(int_type) => {
                    let fn_type = int_type.fn_type(&args, false);
                    let function = self.module.add_function(&*name, fn_type, None);
                    for (v, a) in function.get_params().iter().zip(arg_defs) {
                        self.set_to_environment(a.name, v.as_any_value_enum());
                    }
                    self.current_function = Some(function);
                    let basic_block = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(basic_block);
                    for stmt in body.body {
                        self.stmt(stmt);
                    }
                    function
                }
                // AnyTypeEnum::PointerType(_) => {}
                AnyTypeEnum::StructType(struct_type) => {
                    let fn_type = struct_type.fn_type(&args, false);
                    let function = self.module.add_function(&*name, fn_type, None);
                    for (v, a) in function.get_params().iter().zip(arg_defs) {
                        self.set_to_environment(a.name, v.as_any_value_enum());
                    }
                    self.current_function = Some(function);
                    let basic_block = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(basic_block);
                    for stmt in body.body {
                        self.stmt(stmt);
                    }
                    function
                }
                // AnyTypeEnum::VectorType(_) => {}
                AnyTypeEnum::VoidType(void_type) => {
                    let fn_type = void_type.fn_type(&args, false);
                    let function = self.module.add_function(&*name, fn_type, None);
                    for (v, a) in function.get_params().iter().zip(arg_defs) {
                        self.set_to_environment(a.name, v.as_any_value_enum());
                    }
                    self.current_function = Some(function);
                    let basic_block = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(basic_block);
                    for stmt in body.body {
                        self.stmt(stmt);
                    }
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
                AnyTypeEnum::IntType(int_type) => {
                    let fn_type = int_type.fn_type(&args, false);
                    let f = self.module.add_function(&*name, fn_type, None);
                    self.current_function = Some(f);
                    f
                }
                // AnyTypeEnum::PointerType(_) => {}
                AnyTypeEnum::StructType(struct_type) => {
                    let fn_type = struct_type.fn_type(&args, false);
                    let f = self.module.add_function(&*name, fn_type, None);
                    self.current_function = Some(f);
                    f
                }
                // AnyTypeEnum::VectorType(_) => {}
                AnyTypeEnum::VoidType(void_type) => {
                    let fn_type = void_type.fn_type(&args, false);
                    let f = self.module.add_function(&*name, fn_type, None);
                    self.current_function = Some(f);
                    f
                }
                _ => {
                    println!("{}", return_type_name);
                    exit(-1)
                }
            };
            AnyValueEnum::from(func)
        }
    }

    pub fn struct_(&self, s: MLStruct) -> AnyValueEnum<'ctx> {
        let struct_ = self.context.opaque_struct_type(&*s.name);
        let struct_fields: Vec<BasicTypeEnum<'ctx>> = s
            .fields
            .into_iter()
            .map(|f| {
                let any_type = self.type_name_to_type(&*(f.type_.into_value_type().name));
                BasicTypeEnum::try_from(any_type).unwrap()
            })
            .collect();
        struct_.set_body(&struct_fields, false);
        struct_.const_zero().as_any_value_enum()
    }

    pub fn stmt(&mut self, s: MLStmt) -> AnyValueEnum<'ctx> {
        match s {
            MLStmt::Expr(expr) => self.expr(expr),
            MLStmt::Decl(decl) => self.decl(decl),
            MLStmt::Assignment(a) => self.assignment_stmt(a),
            MLStmt::Loop(l) => self.loop_stmt(l),
        }
    }

    pub fn assignment_stmt(&mut self, assignment: MLAssignmentStmt) -> AnyValueEnum<'ctx> {
        let value = self.expr(assignment.value);
        println!("{:?}", &assignment.target);
        match value {
            AnyValueEnum::IntValue(i) => {
                let target = self.expr(assignment.target);
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, i));
                }
                exit(-3)
            }
            AnyValueEnum::FloatValue(f) => {
                let target = self.expr(assignment.target);
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, f));
                }
                exit(-3)
            }
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::ArrayValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            // AnyValueEnum::PointerValue(_) => {}
            AnyValueEnum::StructValue(s) => {
                let target = self.expr(assignment.target);
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, s));
                }
                exit(-3)
            }
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            _ => exit(-3),
        }
    }

    pub fn loop_stmt(&mut self, lop: MLLoopStmt) -> AnyValueEnum<'ctx> {
        let condition = lop.condition;
        let block = lop.block;
        let loop_body_block = self
            .context
            .append_basic_block(self.current_function.unwrap(), "loop");
        let after_loop_block = self
            .context
            .append_basic_block(self.current_function.unwrap(), "after_loop");
        // loop に入るかの検査
        let cond = self.expr(condition.clone());
        self.builder.build_conditional_branch(
            cond.into_int_value(),
            loop_body_block,
            after_loop_block,
        );
        self.builder.position_at_end(loop_body_block);
        for stmt in block.body {
            self.stmt(stmt);
        }
        // loop を継続するかの検査
        let cond = self.expr(condition);
        let i = self.builder.build_conditional_branch(
            cond.into_int_value(),
            loop_body_block,
            after_loop_block,
        );
        self.builder.position_at_end(after_loop_block);
        i.as_any_value_enum()
    }

    pub fn file(&mut self, f: MLFile) {
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

    fn type_name_to_type(&self, type_name: &str) -> AnyTypeEnum<'ctx> {
        println!("{}", type_name);
        match type_name {
            "Unit" => AnyTypeEnum::from(self.context.void_type()),
            "Int8" | "UInt8" => AnyTypeEnum::from(self.context.i8_type()),
            "Int16" | "UInt16" => AnyTypeEnum::from(self.context.i16_type()),
            "Int32" | "UInt32" => AnyTypeEnum::from(self.context.i32_type()),
            "Int64" | "UInt64" => AnyTypeEnum::from(self.context.i64_type()),
            "Bool" => AnyTypeEnum::from(self.context.bool_type()),
            "Float" => AnyTypeEnum::from(self.context.f32_type()),
            "Double" => AnyTypeEnum::from(self.context.f64_type()),
            "String" => AnyTypeEnum::from(self.context.i8_type().ptr_type(AddressSpace::Generic)),
            t => AnyTypeEnum::from(self.module.get_struct_type(t).unwrap()),
        }
    }
}
