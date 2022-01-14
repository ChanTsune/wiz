use crate::utils::stacked_hash_map::StackedHashMap;
use either::Either;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::targets::TargetTriple;
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{
    AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue,
};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate, OptimizationLevel};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use wiz_mir::expr::{
    MLArray, MLBinOp, MLBinOpKind, MLBlock, MLCall, MLExpr, MLIf, MLLiteral, MLMember, MLName,
    MLSubscript, MLTypeCast, MLUnaryOp, MLUnaryOpKind,
};
use wiz_mir::ml_decl::{MLDecl, MLFun, MLStruct, MLVar};
use wiz_mir::ml_file::MLFile;
use wiz_mir::ml_type::{MLPrimitiveType, MLType, MLValueType};
use wiz_mir::statement::{MLAssignmentStmt, MLLoopStmt, MLReturn, MLStmt};

pub(crate) struct MLContext<'ctx> {
    pub(crate) struct_environment: StackedHashMap<String, MLStruct>,
    pub(crate) local_environments: StackedHashMap<String, AnyValueEnum<'ctx>>,
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> MLContext<'ctx> {
    pub(crate) fn new() -> Self {
        Self {
            struct_environment: StackedHashMap::from(HashMap::new()),
            local_environments: StackedHashMap::from(HashMap::new()),
            current_function: None,
        }
    }
}

impl<'ctx> MLContext<'ctx> {
    pub fn push_environment(&mut self) {
        self.struct_environment.push(HashMap::new());
        self.local_environments.push(HashMap::new());
    }

    pub fn pop_environment(&mut self) {
        self.struct_environment.pop();
        self.local_environments.pop();
    }

    pub fn put_struct(&mut self, s: MLStruct) {
        self.struct_environment.insert(s.name.clone(), s);
    }

    pub fn get_struct(&self, name: &String) -> Option<MLStruct> {
        self.struct_environment.get(name).cloned()
    }
}

pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) ml_context: MLContext<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn new(context: &'ctx Context, name: &str) -> Self {
        let module: Module<'ctx> = context.create_module(name);
        let execution_engine: ExecutionEngine<'ctx> = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        Self {
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
            ml_context: MLContext::new(),
        }
    }

    fn get_from_environment(&self, name: &String) -> Option<AnyValueEnum<'ctx>> {
        match self.ml_context.local_environments.get(name) {
            Some(v) => Some(*v),
            None => self
                .module
                .get_function(name)
                .map(AnyValueEnum::FunctionValue),
        }
    }

    fn set_to_environment(&mut self, name: String, value: AnyValueEnum<'ctx>) {
        self.ml_context.local_environments.insert(name, value);
    }

    fn get_struct_field_index_by_name(&self, m: MLType, n: String) -> Option<u32> {
        match m {
            MLType::Value(m) => match m {
                MLValueType::Struct(type_name) => match self.ml_context.get_struct(&type_name) {
                    None => {
                        eprintln!("Type {:?} dose not defined.", type_name);
                        None
                    }
                    Some(s) => match s.fields.iter().position(|f| f.name == n) {
                        None => {
                            eprintln!("field '{:?}' dose not found in {:?}", n, type_name);
                            None
                        }
                        Some(i) => Some(i as u32),
                    },
                },
                p => {
                    eprintln!("Invalid type '{:?}'", p);
                    None
                }
            },
            MLType::Function(f) => {
                eprintln!("Invalid type '{:?}'", f);
                None
            }
        }
    }

    pub fn expr(&mut self, e: MLExpr) -> AnyValueEnum<'ctx> {
        match e {
            MLExpr::Name(n) => self.name_expr(n),
            MLExpr::Literal(literal) => self.literal(literal),
            MLExpr::PrimitiveBinOp(b) => self.binop(b),
            MLExpr::PrimitiveUnaryOp(u) => self.unary_op(u),
            MLExpr::PrimitiveSubscript(s) => self.subscript(s),
            MLExpr::Call(c) => self.call(c),
            MLExpr::Member(m) => self.member(m),
            MLExpr::Array(a) => self.array(a),
            MLExpr::If(i) => self.if_expr(i),
            MLExpr::When => todo!("when expr"),
            MLExpr::Return(r) => self.return_expr(r),
            MLExpr::PrimitiveTypeCast(t) => self.type_cast(t),
            MLExpr::Block(b) => self.block(b),
        }
    }

    pub fn name_expr(&self, n: MLName) -> AnyValueEnum<'ctx> {
        match self.get_from_environment(&n.name) {
            None => {
                panic!("Can not resolve name `{}`", n.name)
            }
            Some(n) => n,
        }
    }

    pub fn literal(&self, l: MLLiteral) -> AnyValueEnum<'ctx> {
        match l {
            MLLiteral::Integer { value, type_ } => {
                let i: u64 = value.parse().unwrap();
                let int_type = match type_ {
                    MLValueType::Primitive(name) => match name {
                        MLPrimitiveType::Int8 | MLPrimitiveType::UInt8 => self.context.i8_type(),
                        MLPrimitiveType::Int16 | MLPrimitiveType::UInt16 => self.context.i16_type(),
                        MLPrimitiveType::Int32 | MLPrimitiveType::UInt32 => self.context.i32_type(),
                        MLPrimitiveType::Int64 | MLPrimitiveType::UInt64 => self.context.i64_type(),
                        MLPrimitiveType::Int128 | MLPrimitiveType::UInt128 => {
                            self.context.i128_type()
                        }
                        MLPrimitiveType::Size | MLPrimitiveType::USize => self
                            .context
                            .ptr_sized_int_type(self.execution_engine.get_target_data(), None),
                        _ => panic!("Invalid MLIR Literal {:?}", name),
                    },
                    p => panic!("Invalid MLIR Literal {:?}", p),
                };
                int_type.const_int(i, false).as_any_value_enum()
            }
            MLLiteral::FloatingPoint { value, type_ } => {
                let f: f64 = value.parse().unwrap();
                let float_type = match type_ {
                    MLValueType::Primitive(name) => match name {
                        MLPrimitiveType::Float => self.context.f32_type(),
                        MLPrimitiveType::Double => self.context.f64_type(),
                        _ => panic!("Invalid MLIR Literal {:?}", name),
                    },
                    p => panic!("Invalid MLIR Literal {:?}", p),
                };
                float_type.const_float(f).as_any_value_enum()
            }
            MLLiteral::String { value, type_: _ } => unsafe {
                let str = self
                    .builder
                    .build_global_string(value.as_ref(), value.as_str());
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
                let str =
                    self.builder
                        .build_bitcast(str.as_pointer_value(), i8_ptr_type, value.as_str());
                str.as_any_value_enum()
            },
            MLLiteral::Boolean { value, type_: _ } => {
                let b: bool = value.parse().unwrap();
                let bool_type = self.context.bool_type();
                bool_type
                    .const_int(if b { 1 } else { 0 }, false)
                    .as_any_value_enum()
            }
            MLLiteral::Null { type_ } => {
                let ty = self.ml_type_to_type(type_);
                let ty = BasicTypeEnum::try_from(ty).unwrap();
                let ptr_ty = ty.ptr_type(AddressSpace::Generic);
                ptr_ty.const_null().as_any_value_enum()
            }
            MLLiteral::Struct { type_ } => {
                let struct_type = self.module.get_struct_type(&*match type_ {
                    MLValueType::Struct(name) => name,
                    p => panic!("Invalid Struct Literal {:?}", p),
                });
                let struct_type = struct_type.unwrap();
                struct_type.const_zero().as_any_value_enum()
            }
        }
    }

    pub fn call(&mut self, c: MLCall) -> AnyValueEnum<'ctx> {
        let target = self.name_expr(c.target);
        let args = c.args.into_iter().map(|arg| {
            if let MLValueType::Primitive(name) = arg.arg.type_().into_value_type() {
                if name != MLPrimitiveType::String {
                    let t = MLValueType::Primitive(name);
                    let e = self.expr(arg.arg);
                    self.load_if_pointer_value(e, &t)
                } else {
                    self.expr(arg.arg)
                }
            } else if let MLValueType::Pointer(p) = arg.arg.type_().into_value_type() {
                let t = MLValueType::Pointer(p);
                let e = self.expr(arg.arg);
                self.load_if_pointer_value(e, &t)
            } else {
                self.expr(arg.arg)
            }
        });
        let args: Vec<_> = args
            .filter_map(|arg| BasicValueEnum::try_from(arg).ok())
            .collect();
        let args: Vec<BasicMetadataValueEnum> = args.into_iter().map(|i| i.into()).collect();
        let function = target.into_function_value();
        let bv = self
            .builder
            .build_call(function, &args, "f_call")
            .try_as_basic_value();
        match bv {
            Either::Left(vb) => AnyValueEnum::from(vb),
            Either::Right(iv) => AnyValueEnum::from(iv),
        }
    }

    pub fn binop(&mut self, b: MLBinOp) -> AnyValueEnum<'ctx> {
        let l_type = b.left.type_().into_value_type();
        let r_type = b.right.type_().into_value_type();
        let lft = self.expr(*b.left);
        let rit = self.expr(*b.right);
        let lft = self.load_if_pointer_value(lft, &l_type);
        let rit = self.load_if_pointer_value(rit, &r_type);

        match (lft, rit) {
            (AnyValueEnum::IntValue(left), AnyValueEnum::IntValue(right)) => match b.kind {
                MLBinOpKind::Plus => {
                    let v = self.builder.build_int_add(left, right, "sum");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Minus => {
                    let v = self.builder.build_int_sub(left, right, "sub");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Mul => {
                    let v = self.builder.build_int_mul(left, right, "mul");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Div => {
                    let v = self.builder.build_int_signed_div(left, right, "sdiv");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Mod => {
                    let v = self.builder.build_int_signed_rem(left, right, "srem");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Equal => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::EQ, left, right, "eq");
                    v.as_any_value_enum()
                }
                MLBinOpKind::GrateThanEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SGE, left, right, "gte");
                    v.as_any_value_enum()
                }
                MLBinOpKind::GrateThan => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SGT, left, right, "gt");
                    v.as_any_value_enum()
                }
                MLBinOpKind::LessThanEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SLE, left, right, "lte");
                    v.as_any_value_enum()
                }
                MLBinOpKind::LessThan => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::SLT, left, right, "lt");
                    v.as_any_value_enum()
                }
                MLBinOpKind::NotEqual => {
                    let v = self
                        .builder
                        .build_int_compare(IntPredicate::NE, left, right, "neq");
                    v.as_any_value_enum()
                }
            },
            (AnyValueEnum::FloatValue(left), AnyValueEnum::FloatValue(right)) => match b.kind {
                MLBinOpKind::Plus => {
                    let v = self.builder.build_float_add(left, right, "sum");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Minus => {
                    let v = self.builder.build_float_sub(left, right, "sub");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Mul => {
                    let v = self.builder.build_float_mul(left, right, "mul");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Div => {
                    let v = self.builder.build_float_div(left, right, "div");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Mod => {
                    let v = self.builder.build_float_rem(left, right, "rem");
                    v.as_any_value_enum()
                }
                MLBinOpKind::Equal => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OEQ, left, right, "eq");
                    v.as_any_value_enum()
                }
                MLBinOpKind::GrateThanEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OGE, left, right, "gte");
                    v.as_any_value_enum()
                }
                MLBinOpKind::GrateThan => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OGT, left, right, "gt");
                    v.as_any_value_enum()
                }
                MLBinOpKind::LessThanEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OLE, left, right, "lte");
                    v.as_any_value_enum()
                }
                MLBinOpKind::LessThan => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::OLT, left, right, "lt");
                    v.as_any_value_enum()
                }
                MLBinOpKind::NotEqual => {
                    let v =
                        self.builder
                            .build_float_compare(FloatPredicate::ONE, left, right, "neq");
                    v.as_any_value_enum()
                }
            },
            (AnyValueEnum::PointerValue(p), AnyValueEnum::IntValue(i)) => match b.kind {
                MLBinOpKind::Plus => {
                    let p = unsafe { self.builder.build_in_bounds_gep(p, &[i], "padd") };
                    p.as_any_value_enum()
                }
                MLBinOpKind::Minus => {
                    let i = self.builder.build_int_neg(i, "psub_ineg");
                    let p = unsafe { self.builder.build_in_bounds_gep(p, &[i], "psub") };
                    p.as_any_value_enum()
                }
                MLBinOpKind::Mul => {
                    todo!()
                }
                MLBinOpKind::Div => {
                    todo!()
                }
                MLBinOpKind::Mod => {
                    todo!()
                }
                MLBinOpKind::Equal => {
                    todo!()
                }
                MLBinOpKind::GrateThanEqual => {
                    todo!()
                }
                MLBinOpKind::GrateThan => {
                    todo!()
                }
                MLBinOpKind::LessThanEqual => {
                    todo!()
                }
                MLBinOpKind::LessThan => {
                    todo!()
                }
                MLBinOpKind::NotEqual => {
                    todo!()
                }
            },
            (r, l) => {
                panic!(
                    "Unsupported binary operation.\n{:?},{:?},{:?}",
                    r, b.kind, l
                )
            }
        }
    }

    pub fn unary_op(&mut self, u: MLUnaryOp) -> AnyValueEnum<'ctx> {
        let target = self.expr(*u.target);
        match target {
            AnyValueEnum::ArrayValue(_) => {
                todo!()
            }
            AnyValueEnum::IntValue(target) => match u.kind {
                MLUnaryOpKind::Negative => self.builder.build_int_neg(target, "negative"),
                MLUnaryOpKind::Positive => target,
                MLUnaryOpKind::Not => self.builder.build_not(target, "not"),
                MLUnaryOpKind::Ref => {
                    todo!()
                }
                MLUnaryOpKind::DeRef => {
                    todo!()
                }
            }
            .as_any_value_enum(),
            AnyValueEnum::FloatValue(target) => match u.kind {
                MLUnaryOpKind::Negative => self.builder.build_float_neg(target, "negative"),
                MLUnaryOpKind::Positive => target,
                MLUnaryOpKind::Not => {
                    todo!()
                }
                MLUnaryOpKind::Ref => {
                    todo!()
                }
                MLUnaryOpKind::DeRef => {
                    todo!()
                }
            }
            .as_any_value_enum(),
            AnyValueEnum::PhiValue(_) => {
                todo!()
            }
            AnyValueEnum::FunctionValue(_) => {
                todo!()
            }
            AnyValueEnum::PointerValue(target) => match u.kind {
                MLUnaryOpKind::Negative => {
                    todo!()
                }
                MLUnaryOpKind::Positive => {
                    todo!()
                }
                MLUnaryOpKind::Not => {
                    todo!()
                }
                MLUnaryOpKind::Ref => BasicValueEnum::PointerValue(target),
                MLUnaryOpKind::DeRef => self.builder.build_load(target, "p_deref"),
            }
            .as_any_value_enum(),
            AnyValueEnum::StructValue(_) => {
                todo!()
            }
            AnyValueEnum::VectorValue(_) => {
                todo!()
            }
            AnyValueEnum::InstructionValue(_) => {
                todo!()
            }
        }
    }

    pub fn subscript(&mut self, s: MLSubscript) -> AnyValueEnum<'ctx> {
        let i_type = s.index.type_().into_value_type();
        let t_type = s.target.type_().into_value_type();
        let target = self.expr(*s.target);
        let target = self.load_if_pointer_value(target, &t_type);
        let index = self.expr(*s.index);
        let index = self.load_if_pointer_value(index, &i_type);
        match target {
            AnyValueEnum::ArrayValue(a) => unsafe {
                let a_type = a
                    .get_type()
                    .get_element_type()
                    .ptr_type(AddressSpace::Generic);
                let p = self.builder.build_bitcast(a, a_type, "aptr");
                let i = self.builder.build_in_bounds_gep(
                    p.into_pointer_value(),
                    &[index.into_int_value()],
                    "idx",
                );
                i.as_any_value_enum()
            },
            // AnyValueEnum::IntValue(_) => {}
            // AnyValueEnum::FloatValue(_) => {}
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            AnyValueEnum::PointerValue(p) => unsafe {
                let et = p.get_type().get_element_type();
                let p = if let AnyTypeEnum::ArrayType(a) = et {
                    let a_type = a.get_element_type().ptr_type(AddressSpace::Generic);
                    self.builder
                        .build_bitcast(p, a_type, "aptr")
                        .into_pointer_value()
                } else {
                    p
                };
                let i = self
                    .builder
                    .build_in_bounds_gep(p, &[index.into_int_value()], "idx");
                i.as_any_value_enum()
            },
            // AnyValueEnum::StructValue(_) => {}
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            t => panic!("unsupported subscript {:?}", t),
        }
    }

    pub fn member(&mut self, m: MLMember) -> AnyValueEnum<'ctx> {
        let field_index = self
            .get_struct_field_index_by_name(m.target.type_(), m.name)
            .unwrap();
        let target = match self.expr(*m.target) {
            AnyValueEnum::PointerValue(p) => p,
            AnyValueEnum::StructValue(_) => {
                eprintln!("never execution branch executed.");
                panic!("struct member can not access directly.");
            }
            _ => panic!("never execution branch executed."),
        };

        let ep = self
            .builder
            .build_struct_gep(target, field_index, "struct_gep")
            .unwrap();
        ep.as_any_value_enum()
    }

    fn array(&mut self, a: MLArray) -> AnyValueEnum<'ctx> {
        let array_type = self.ml_type_to_type(a.type_).into_array_type();
        let ptr = self.builder.build_alloca(array_type, "");
        for (i, element) in a.elements.into_iter().enumerate() {
            let i64_type = self.context.i64_type();
            let idx = i64_type.const_int(i as u64, false);
            let eidx = unsafe { self.builder.build_in_bounds_gep(ptr, &[idx], "") };
            let ptr_type = eidx
                .get_type()
                .get_element_type()
                .into_array_type()
                .get_element_type()
                .ptr_type(AddressSpace::Generic);
            let eidx = self
                .builder
                .build_bitcast(eidx, ptr_type, "")
                .into_pointer_value();
            let v = BasicValueEnum::try_from(self.expr(element)).unwrap();
            self.builder.build_store(eidx, v);
        }
        self.builder.build_load(ptr, "").as_any_value_enum()
    }

    pub fn if_expr(&mut self, i: MLIf) -> AnyValueEnum<'ctx> {
        let condition = i.condition;
        let body = i.body;
        let else_body = i.else_body;
        match else_body {
            None => {
                let if_block = self
                    .context
                    .append_basic_block(self.ml_context.current_function.unwrap(), "if");
                let after_if_block = self
                    .context
                    .append_basic_block(self.ml_context.current_function.unwrap(), "after_if");
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
                self.builder.build_unconditional_branch(after_if_block);
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
                    .append_basic_block(self.ml_context.current_function.unwrap(), "if");
                let else_block = self
                    .context
                    .append_basic_block(self.ml_context.current_function.unwrap(), "else");
                let after_if_block = self
                    .context
                    .append_basic_block(self.ml_context.current_function.unwrap(), "after_if");
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
            Some(e) => match *e {
                MLExpr::Name(_) => {
                    let n = self.expr(*e);
                    Some(self.builder.build_load(n.into_pointer_value(), "v"))
                }
                MLExpr::PrimitiveSubscript(_) | MLExpr::Member(_) => {
                    let s_type = e.type_().into_value_type();
                    let s = self.expr(*e);
                    let s = self.load_if_pointer_value(s, &s_type);
                    Some(BasicValueEnum::try_from(s).unwrap())
                }
                _ => Some(BasicValueEnum::try_from(self.expr(*e)).unwrap()),
            },
            None => None,
        };

        AnyValueEnum::from(self.builder.build_return(match &v {
            None => None,
            Some(b) => Some(b),
        }))
    }

    pub fn type_cast(&mut self, t: MLTypeCast) -> AnyValueEnum<'ctx> {
        let target_type = t.target.type_().into_value_type();
        let target = self.expr(*t.target);
        let target = self.load_if_pointer_value(target, &target_type);
        match target {
            // AnyValueEnum::ArrayValue(_) => {}
            AnyValueEnum::IntValue(i) => match self.ml_type_to_type(t.type_) {
                AnyTypeEnum::ArrayType(_) => {
                    todo!()
                }
                AnyTypeEnum::FloatType(ty) => if target_type.is_signed_integer() {
                    self.builder
                        .build_signed_int_to_float(i, ty, "sint_to_float")
                } else {
                    self.builder
                        .build_unsigned_int_to_float(i, ty, "uint_to_float")
                }
                .as_any_value_enum(),
                AnyTypeEnum::FunctionType(_) => {
                    todo!()
                }
                AnyTypeEnum::IntType(ty) => {
                    let t = self.builder.build_int_cast(i, ty, "int_cast");
                    t.as_any_value_enum()
                }
                AnyTypeEnum::PointerType(ty) => {
                    let t = self.builder.build_int_to_ptr(i, ty, "int_to_ptr");
                    t.as_any_value_enum()
                }
                AnyTypeEnum::StructType(_) => {
                    todo!()
                }
                AnyTypeEnum::VectorType(_) => {
                    todo!()
                }
                AnyTypeEnum::VoidType(_) => {
                    todo!()
                }
            },
            AnyValueEnum::FloatValue(f) => {
                let is_signed = t.type_.is_signed_integer();
                match self.ml_type_to_type(t.type_) {
                    AnyTypeEnum::ArrayType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::FloatType(ty) => {
                        let t = self.builder.build_float_cast(f, ty, "float_cast");
                        t.as_any_value_enum()
                    }
                    AnyTypeEnum::FunctionType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::IntType(ty) => if is_signed {
                        self.builder
                            .build_float_to_signed_int(f, ty, "float_to_int")
                    } else {
                        self.builder
                            .build_float_to_unsigned_int(f, ty, "float_to_uint")
                    }
                    .as_any_value_enum(),
                    AnyTypeEnum::PointerType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::StructType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::VectorType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::VoidType(_) => {
                        todo!()
                    }
                }
            }
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            AnyValueEnum::PointerValue(ptr) => {
                let ty = self.ml_type_to_type(t.type_);
                match ty {
                    AnyTypeEnum::ArrayType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::FloatType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::FunctionType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::IntType(ty) => {
                        let t = self.builder.build_ptr_to_int(ptr, ty, "ptr_to_int");
                        t.as_any_value_enum()
                    }
                    AnyTypeEnum::PointerType(ty) => {
                        let t = self.builder.build_pointer_cast(ptr, ty, "ptr_cast");
                        t.as_any_value_enum()
                    }
                    AnyTypeEnum::StructType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::VectorType(_) => {
                        todo!()
                    }
                    AnyTypeEnum::VoidType(_) => {
                        todo!()
                    }
                }
            }
            // AnyValueEnum::StructValue(_) => {}
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            a => {
                panic!("never execution branch executed!! {:?}", a)
            }
        }
    }

    pub fn block(&mut self, b: MLBlock) -> AnyValueEnum<'ctx> {
        let i8_type = self.context.i8_type(); // Void
        let len = b.body.len();
        for (i, stmt) in b.body.into_iter().enumerate() {
            let last_index = len - 1;
            if i == last_index {
                return self.stmt(stmt);
            } else {
                self.stmt(stmt)
            };
        }
        AnyValueEnum::from(i8_type.const_int(0, false))
    }

    fn load_if_pointer_value(
        &self,
        v: AnyValueEnum<'ctx>,
        typ: &MLValueType,
    ) -> AnyValueEnum<'ctx> {
        if Self::need_load(v.get_type(), typ) {
            let p = v.into_pointer_value();
            self.builder.build_load(p, "v").as_any_value_enum()
        } else {
            v
        }
    }

    fn need_load(may_be_pointer: AnyTypeEnum<'ctx>, request_type: &MLValueType) -> bool {
        match may_be_pointer {
            AnyTypeEnum::PointerType(p) => match request_type {
                MLValueType::Primitive(pv) => match pv {
                    MLPrimitiveType::String | MLPrimitiveType::Noting | MLPrimitiveType::Unit => {
                        false
                    }
                    _ => true,
                },
                MLValueType::Struct(_) => true,
                MLValueType::Pointer(r) | MLValueType::Reference(r) => match &**r {
                    MLType::Value(r) => Self::need_load(p.get_element_type(), r),
                    MLType::Function(_) => {
                        todo!()
                    }
                },
                MLValueType::Array(_, _) => false,
            },
            _ => false,
        }
    }

    pub fn decl(&mut self, d: MLDecl) -> AnyValueEnum<'ctx> {
        match d {
            MLDecl::Var(v) => self.global_var(v),
            MLDecl::Fun(f) => self.fun(f),
            MLDecl::Struct(s) => self.struct_(s),
        }
    }

    fn global_var(&mut self, v: MLVar) -> AnyValueEnum<'ctx> {
        let MLVar {
            is_mute,
            name,
            type_,
            value,
        } = v;
        let ty = self.ml_type_to_type(type_.into_value_type());
        let ty = BasicTypeEnum::try_from(ty).unwrap();
        let v = self.module.add_global(ty, None, &*name);
        let value = self.expr(value);
        let value = BasicValueEnum::try_from(value).unwrap();
        v.set_initializer(&value);
        if is_mute {
            v.set_constant(false)
        } else {
            v.set_constant(true)
        };
        self.set_to_environment(name, v.as_any_value_enum());
        v.as_any_value_enum()
    }

    pub fn local_var(&mut self, v: MLVar) -> AnyValueEnum<'ctx> {
        let MLVar {
            is_mute,
            name,
            type_,
            value,
        } = v;
        let v_type = type_.into_value_type();
        let value = self.expr(value);
        let value = self.load_if_pointer_value(value, &v_type);
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
            AnyValueEnum::PointerValue(p) => {
                let ptr_type = p.get_type();
                let ptr = self.builder.build_alloca(ptr_type, &*name);
                self.set_to_environment(name, ptr.as_any_value_enum());
                self.builder.build_store(ptr, p).as_any_value_enum()
            }
            AnyValueEnum::ArrayValue(a) => {
                let array_type = a.get_type();
                let ptr = self.builder.build_alloca(array_type, &*name);
                self.set_to_environment(name, ptr.as_any_value_enum());
                self.builder.build_store(ptr, a).as_any_value_enum()
            }
            t => panic!("undefined root executed {:?}", t),
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
        let return_type = self.ml_type_to_type(return_type);
        let args: Vec<_> = arg_defs
            .iter()
            .map(|a| {
                let a = a.type_.clone();
                let a = self.ml_type_to_type(a);
                if a.is_struct_type() {
                    let a = a.into_struct_type().ptr_type(AddressSpace::Generic);
                    a.as_any_type_enum()
                } else {
                    a
                }
            })
            .map(|a| BasicTypeEnum::try_from(a).unwrap())
            .collect();
        let args: Vec<BasicMetadataTypeEnum> = args.into_iter().map(|i| i.into()).collect();
        let result = if let Some(body) = body {
            self.ml_context.push_environment();
            let is_void_type = return_type.is_void_type();
            let fn_type = match return_type {
                // AnyTypeEnum::ArrayType(_) => {}
                AnyTypeEnum::FloatType(float_type) => float_type.fn_type(&args, false),
                // AnyTypeEnum::FunctionType(_) => {}
                AnyTypeEnum::IntType(int_type) => int_type.fn_type(&args, false),
                AnyTypeEnum::PointerType(pointer_type) => pointer_type.fn_type(&args, false),
                AnyTypeEnum::StructType(struct_type) => struct_type.fn_type(&args, false),
                // AnyTypeEnum::VectorType(_) => {}
                AnyTypeEnum::VoidType(void_type) => void_type.fn_type(&args, false),
                a => {
                    panic!("Return Type Error. {:?}", a);
                }
            };
            let function = if let Some(function) = self.module.get_function(&*name) {
                if !function.get_basic_blocks().is_empty() {
                    panic!("function `{}` is already defined", name);
                };
                function
            } else {
                self.module.add_function(&*name, fn_type, None)
            };
            for (v, a) in function.get_params().iter().zip(arg_defs) {
                self.set_to_environment(a.name, v.as_any_value_enum());
            }
            self.ml_context.current_function = Some(function);
            let basic_block = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(basic_block);
            for stmt in body.body {
                self.stmt(stmt);
            }
            if is_void_type {
                self.builder.build_return(None);
            };
            self.ml_context.pop_environment();
            AnyValueEnum::from(function)
        } else {
            let fn_type = match return_type {
                // AnyTypeEnum::ArrayType(_) => {}
                AnyTypeEnum::FloatType(float_type) => float_type.fn_type(&args, false),
                // AnyTypeEnum::FunctionType(_) => {}
                AnyTypeEnum::IntType(int_type) => int_type.fn_type(&args, false),
                AnyTypeEnum::PointerType(ptr_type) => ptr_type.fn_type(&args, false),
                AnyTypeEnum::StructType(struct_type) => struct_type.fn_type(&args, false),
                // AnyTypeEnum::VectorType(_) => {}
                AnyTypeEnum::VoidType(void_type) => void_type.fn_type(&args, false),
                a => panic!("Return type Error. {:?}", a),
            };
            let f = if let Some(f) = self.module.get_function(&*name) {
                f
            } else {
                self.module.add_function(&*name, fn_type, None)
            };
            self.ml_context.current_function = Some(f);
            AnyValueEnum::from(f)
        };
        result
    }

    pub fn struct_(&mut self, s: MLStruct) -> AnyValueEnum<'ctx> {
        self.ml_context.put_struct(s.clone());
        let struct_ = self.module.get_struct_type(&*s.name).unwrap();
        let struct_fields: Vec<_> = s
            .fields
            .into_iter()
            .map(|f| {
                let any_type = self.ml_type_to_type(f.type_);
                BasicTypeEnum::try_from(any_type).unwrap()
            })
            .collect();
        struct_.set_body(&struct_fields, false);
        struct_.const_zero().as_any_value_enum()
    }

    pub fn stmt(&mut self, s: MLStmt) -> AnyValueEnum<'ctx> {
        match s {
            MLStmt::Expr(expr) => self.expr(expr),
            MLStmt::Var(decl) => self.local_var(decl),
            MLStmt::Assignment(a) => self.assignment_stmt(a),
            MLStmt::Loop(l) => self.loop_stmt(l),
            MLStmt::Return(r) => self.return_expr(r),
        }
    }

    pub fn assignment_stmt(&mut self, assignment: MLAssignmentStmt) -> AnyValueEnum<'ctx> {
        let a_type = assignment.value.type_().into_value_type();
        let value = self.expr(assignment.value);
        let value = self.load_if_pointer_value(value, &a_type);
        match value {
            AnyValueEnum::IntValue(i) => {
                let p = self.expr(assignment.target).into_pointer_value();
                AnyValueEnum::from(self.builder.build_store(p, i))
            }
            AnyValueEnum::FloatValue(f) => {
                let p = self.expr(assignment.target).into_pointer_value();
                AnyValueEnum::from(self.builder.build_store(p, f))
            }
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::ArrayValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            AnyValueEnum::PointerValue(s) => {
                let p = self.expr(assignment.target).into_pointer_value();
                AnyValueEnum::from(self.builder.build_store(p, s))
            }
            AnyValueEnum::StructValue(s) => {
                let p = self.expr(assignment.target).into_pointer_value();
                AnyValueEnum::from(self.builder.build_store(p, s))
            }
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            v => panic!("Can not assinable {:?}", v),
        }
    }

    pub fn loop_stmt(&mut self, lop: MLLoopStmt) -> AnyValueEnum<'ctx> {
        let condition = lop.condition;
        let block = lop.block;
        let loop_body_block = self
            .context
            .append_basic_block(self.ml_context.current_function.unwrap(), "loop");
        let after_loop_block = self
            .context
            .append_basic_block(self.ml_context.current_function.unwrap(), "after_loop");
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
        // detect type
        for d in f.body.iter() {
            if let MLDecl::Struct(s) = d {
                self.context.opaque_struct_type(&*s.name);
            }
        }
        for d in f.body {
            self.decl(d);
        }
    }

    /// Set Target Triple
    pub fn set_target_triple(&mut self, triple: &str) {
        let target_triple = TargetTriple::create(triple);
        self.module.set_triple(&target_triple)
    }

    /// Write LLVM IR to file to the given path.
    pub fn print_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), LLVMString> {
        self.module.print_to_file(path)
    }

    fn ml_type_to_type(&self, ml_type: MLValueType) -> AnyTypeEnum<'ctx> {
        match ml_type {
            MLValueType::Primitive(name) => match name {
                MLPrimitiveType::Unit => AnyTypeEnum::from(self.context.void_type()),
                MLPrimitiveType::Int8 | MLPrimitiveType::UInt8 => {
                    AnyTypeEnum::from(self.context.i8_type())
                }
                MLPrimitiveType::Int16 | MLPrimitiveType::UInt16 => {
                    AnyTypeEnum::from(self.context.i16_type())
                }
                MLPrimitiveType::Int32 | MLPrimitiveType::UInt32 => {
                    AnyTypeEnum::from(self.context.i32_type())
                }
                MLPrimitiveType::Int64 | MLPrimitiveType::UInt64 => {
                    AnyTypeEnum::from(self.context.i64_type())
                }
                MLPrimitiveType::Bool => AnyTypeEnum::from(self.context.bool_type()),
                MLPrimitiveType::Float => AnyTypeEnum::from(self.context.f32_type()),
                MLPrimitiveType::Double => AnyTypeEnum::from(self.context.f64_type()),
                MLPrimitiveType::String => AnyTypeEnum::from(self.context.i8_type()),
                t => panic!("Invalid Primitive Type {:?}", t),
            },
            MLValueType::Struct(t) => AnyTypeEnum::from(self.module.get_struct_type(&*t).unwrap()),
            MLValueType::Pointer(p) | MLValueType::Reference(p) => match *p {
                MLType::Value(p) => BasicTypeEnum::try_from(self.ml_type_to_type(p))
                    .unwrap()
                    .ptr_type(AddressSpace::Generic)
                    .as_any_type_enum(),
                MLType::Function(_) => {
                    todo!()
                }
            },
            MLValueType::Array(a, size) => {
                let size = size as u32;
                match self.ml_type_to_type(*a) {
                    AnyTypeEnum::ArrayType(a) => a.array_type(size),
                    AnyTypeEnum::FloatType(a) => a.array_type(size),
                    AnyTypeEnum::FunctionType(_) => {
                        panic!("never execution branch executed!!")
                    }
                    AnyTypeEnum::IntType(a) => a.array_type(size),
                    AnyTypeEnum::PointerType(a) => a.array_type(size),
                    AnyTypeEnum::StructType(a) => a.array_type(size),
                    AnyTypeEnum::VectorType(a) => a.array_type(size),
                    AnyTypeEnum::VoidType(_) => {
                        panic!("never execution branch executed!!")
                    }
                }
                .as_any_type_enum()
            }
        }
    }
}
