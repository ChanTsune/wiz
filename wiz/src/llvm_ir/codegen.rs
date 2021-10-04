use crate::middle_level_ir::ml_decl::{MLDecl, MLFun, MLStruct, MLVar};
use crate::middle_level_ir::ml_expr::{
    MLBinOp, MLBinopKind, MLCall, MLExpr, MLIf, MLLiteral, MLMember, MLReturn, MLSubscript,
    MLTypeCast, MLUnaryOp,
};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_stmt::{MLAssignmentStmt, MLBlock, MLLoopStmt, MLStmt};
use crate::middle_level_ir::ml_type::{MLType, MLValueType, MLPrimitiveType};
use crate::utils::stacked_hash_map::StackedHashMap;
use either::Either;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::types::{AnyType, AnyTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use nom::lib::std::convert::TryFrom;
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;

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
        self.struct_environment.get(name).map(|s| s.clone())
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
    fn get_from_environment(&self, name: String) -> Option<AnyValueEnum<'ctx>> {
        match self.ml_context.local_environments.get(&name) {
            Some(v) => Some(*v),
            None => match self.module.get_function(&*name) {
                Some(f) => Some(AnyValueEnum::FunctionValue(f)),
                None => None,
            },
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
            MLExpr::Name(n) => self.get_from_environment(n.name).unwrap(),
            MLExpr::Literal(literal) => self.literal(literal),
            MLExpr::PrimitiveBinOp(b) => self.binop(b),
            MLExpr::PrimitiveUnaryOp(u) => self.unaryop(u),
            MLExpr::PrimitiveSubscript(s) => self.subscript(s),
            MLExpr::Call(c) => self.call(c),
            MLExpr::Member(m) => self.member(m),
            MLExpr::If(i) => self.if_expr(i),
            MLExpr::When => exit(-1),
            MLExpr::Return(r) => self.return_expr(r),
            MLExpr::PrimitiveTypeCast(t) => self.type_cast(t),
        }
    }

    pub fn literal(&self, l: MLLiteral) -> AnyValueEnum<'ctx> {
        match l {
            MLLiteral::Integer { value, type_ } => {
                let i: u64 = value.parse().unwrap();
                let int_type = match type_ {
                    MLValueType::Primitive(name) => match name {
                        MLPrimitiveType::Int8 |
                        MLPrimitiveType::UInt8 => self.context.i8_type(),
                        MLPrimitiveType::Int16 |
                        MLPrimitiveType::UInt16 => self.context.i16_type(),
                        MLPrimitiveType::Int32 |
                        MLPrimitiveType::UInt32 => self.context.i32_type(),
                        MLPrimitiveType::Int64 |
                        MLPrimitiveType::UInt64 => self.context.i64_type(),
                        MLPrimitiveType::Size |
                        MLPrimitiveType::USize => {todo!()}
                        _ => {
                            eprintln!("Invalid MLIR Literal {:?}", name);
                            exit(-1)
                        }
                    },
                    p => {
                        eprintln!("Invalid MLIR Literal {:?}", p);
                        exit(-1)
                    }
                };
                int_type.const_int(i, false).as_any_value_enum()
            }
            MLLiteral::FloatingPoint { value, type_ } => {
                let f: f64 = value.parse().unwrap();
                let float_type = match type_ {
                    MLValueType::Primitive(name) => match name {
                        MLPrimitiveType::Float => self.context.f32_type(),
                        MLPrimitiveType::Double => self.context.f64_type(),
                        _ => {
                            eprintln!("Invalid MLIR Literal {:?}", name);
                            exit(-1)
                        }
                    },
                    p => {
                        eprintln!("Invalid MLIR Literal {:?}", p);
                        exit(-1)
                    }
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
            MLLiteral::Null { .. } => {
                println!("Literall::Null");
                exit(-2)
            }
            MLLiteral::Struct { type_ } => {
                let struct_type = self.module.get_struct_type(&*match type_ {
                    MLValueType::Struct(name) => name,
                    p => {
                        eprintln!("Invalid Struct Literal {:?}", p);
                        exit(-1)
                    }
                });
                let struct_type = struct_type.unwrap();
                struct_type.const_zero().as_any_value_enum()
            }
        }
    }

    pub fn call(&mut self, c: MLCall) -> AnyValueEnum<'ctx> {
        let target = self.expr(*c.target);
        let args = c.args.into_iter().map(|arg| {
            if let MLValueType::Primitive(name) = arg.arg.type_().into_value_type() {
                if name != MLPrimitiveType::String {
                    let t = arg.type_().into_value_type();
                    let e = self.expr(arg.arg);
                    self.load_if_pointer_value(e, &t)
                } else {
                    self.expr(arg.arg)
                }
            } else if let MLValueType::Pointer(p) = arg.arg.type_().into_value_type() {
                let t = arg.type_().into_value_type();
                let e = self.expr(arg.arg);
                self.load_if_pointer_value(e, &t)
            } else {
                self.expr(arg.arg)
            }
        });
        let args: Vec<BasicValueEnum> = args
            .filter_map(|arg| BasicValueEnum::try_from(arg).ok())
            .collect();
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
        todo!()
    }

    pub fn subscript(&mut self, s: MLSubscript) -> AnyValueEnum<'ctx> {
        let i_type = s.index.type_().into_value_type();
        let t_type = s.target.type_().into_value_type();
        let target = self.expr(*s.target);
        let target = self.load_if_pointer_value(target, &t_type);
        let index = self.expr(*s.index);
        let index = self.load_if_pointer_value(index, &i_type);
        match target {
            // AnyValueEnum::ArrayValue(_) => {}
            // AnyValueEnum::IntValue(_) => {}
            // AnyValueEnum::FloatValue(_) => {}
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            AnyValueEnum::PointerValue(p) => unsafe {
                let i = self
                    .builder
                    .build_in_bounds_gep(p, &[index.into_int_value()], "idx");
                i.as_any_value_enum()
            },
            // AnyValueEnum::StructValue(_) => {}
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            t => {
                eprintln!("unsupported subscript {:?}", t);
                exit(-1)
            }
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
                eprintln!("struct member can not access directly.");
                exit(-2)
            }
            _ => {
                eprintln!("never execution branch executed.");
                exit(-2)
            }
        };

        let ep = self
            .builder
            .build_struct_gep(target, field_index, "struct_gep")
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
                MLExpr::PrimitiveSubscript(_) => {
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
        let target = self.expr(*t.target);
        match target {
            // AnyValueEnum::ArrayValue(_) => {}
            AnyValueEnum::IntValue(i) => {
                let ty = match t.type_ {
                    MLValueType::Primitive(p) => match p {
                        MLPrimitiveType::Int8 |
                        MLPrimitiveType::UInt8 => self.context.i8_type(),
                        MLPrimitiveType::Int16 |
                        MLPrimitiveType::UInt16 => self.context.i16_type(),
                        MLPrimitiveType::Int32 |
                        MLPrimitiveType::UInt32 => self.context.i32_type(),
                        MLPrimitiveType::Int64 |
                        MLPrimitiveType::UInt64 => self.context.i64_type(),
                        MLPrimitiveType::Size |
                        MLPrimitiveType::USize => {todo!()}
                        _ => panic!(),
                    },
                    MLValueType::Struct(_) => {
                        todo!()
                    }
                    MLValueType::Pointer(_) => {
                        todo!()
                    }
                    MLValueType::Reference(_) => {
                        todo!()
                    }
                };
                let t = self.builder.build_int_cast(i, ty, "int_cast");
                t.as_any_value_enum()
            }
            AnyValueEnum::FloatValue(f) => {
                let ty = match t.type_ {
                    MLValueType::Primitive(p) => match p {
                        MLPrimitiveType::Float => self.context.f32_type(),
                        MLPrimitiveType::Double => self.context.f64_type(),
                        _ => panic!(),
                    },
                    MLValueType::Struct(_) => {
                        todo!()
                    }
                    MLValueType::Pointer(_) => {
                        todo!()
                    }
                    MLValueType::Reference(_) => {
                        todo!()
                    }
                };
                let t = self.builder.build_float_cast(f, ty, "float_cast");
                t.as_any_value_enum()
            }
            // AnyValueEnum::PhiValue(_) => {}
            // AnyValueEnum::FunctionValue(_) => {}
            // AnyValueEnum::PointerValue(_) => {}
            // AnyValueEnum::StructValue(_) => {}
            // AnyValueEnum::VectorValue(_) => {}
            // AnyValueEnum::InstructionValue(_) => {}
            a => {
                panic!("never execution branch executed!!")
            }
        }
    }

    pub fn block(&mut self, b: MLBlock) -> AnyValueEnum<'ctx> {
        let i64_type = self.context.i64_type(); // Void
        let len = b.body.len();
        for (i, stmt) in b.body.into_iter().enumerate() {
            let last_index = len - 1;
            if i == last_index {
                return self.stmt(stmt);
            } else {
                self.stmt(stmt)
            };
        }
        AnyValueEnum::from(i64_type.const_int(0, false))
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
                MLValueType::Primitive(_) | MLValueType::Struct(_) => true,
                MLValueType::Pointer(r) | MLValueType::Reference(r) => {
                    Self::need_load(p.get_element_type(), r)
                }
            },
            _ => false,
        }
    }

    pub fn decl(&mut self, d: MLDecl) -> AnyValueEnum<'ctx> {
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
        let v_type = type_.clone().into_value_type();
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
            t => {
                eprintln!("undefined root executed {:?}", t);
                exit(-14)
            }
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
        let args: Vec<BasicTypeEnum<'ctx>> = arg_defs
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
                _ => {
                    println!("Return Type Error.");
                    exit(-1)
                }
            };
            let function = self.module.add_function(&*name, fn_type, None);
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
                _ => {
                    println!("Return type Error");
                    exit(-1)
                }
            };
            let f = self.module.add_function(&*name, fn_type, None);
            self.ml_context.current_function = Some(f);
            AnyValueEnum::from(f)
        };
        result
    }

    pub fn struct_(&mut self, s: MLStruct) -> AnyValueEnum<'ctx> {
        self.ml_context.put_struct(s.clone());
        let struct_ = self.context.opaque_struct_type(&*s.name);
        let struct_fields: Vec<BasicTypeEnum<'ctx>> = s
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
            MLStmt::Var(decl) => self.var(decl),
            MLStmt::Assignment(a) => self.assignment_stmt(a),
            MLStmt::Loop(l) => self.loop_stmt(l),
        }
    }

    pub fn assignment_stmt(&mut self, assignment: MLAssignmentStmt) -> AnyValueEnum<'ctx> {
        // TODO: replace to ↓
        // let a_type = assignment.value.type_().into_value_type();
        let a_type = assignment.target.type_().into_value_type();
        let value = self.expr(assignment.value);
        let value = self.load_if_pointer_value(value, &a_type);
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
            AnyValueEnum::PointerValue(s) => {
                let target = self.expr(assignment.target);
                if let AnyValueEnum::PointerValue(p) = target {
                    return AnyValueEnum::from(self.builder.build_store(p, s));
                }
                exit(-3)
            }
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
        for d in f.body {
            self.decl(d);
        }
    }

    /**
     * Write the LLVM IR to a file in the path.
     */
    pub fn print_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), LLVMString> {
        self.module.print_to_file(path)
    }

    fn ml_type_to_type(&self, ml_type: MLValueType) -> AnyTypeEnum<'ctx> {
        match ml_type {
            MLValueType::Primitive(name) => match name {
                MLPrimitiveType::Unit => AnyTypeEnum::from(self.context.void_type()),
                MLPrimitiveType::Int8 | MLPrimitiveType::UInt8 => AnyTypeEnum::from(self.context.i8_type()),
                MLPrimitiveType::Int16 | MLPrimitiveType::UInt16 => AnyTypeEnum::from(self.context.i16_type()),
                MLPrimitiveType::Int32 | MLPrimitiveType::UInt32 => AnyTypeEnum::from(self.context.i32_type()),
                MLPrimitiveType::Int64 | MLPrimitiveType::UInt64 => AnyTypeEnum::from(self.context.i64_type()),
                MLPrimitiveType::Bool => AnyTypeEnum::from(self.context.bool_type()),
                MLPrimitiveType::Float => AnyTypeEnum::from(self.context.f32_type()),
                MLPrimitiveType::Double => AnyTypeEnum::from(self.context.f64_type()),
                MLPrimitiveType::String => {
                    AnyTypeEnum::from(self.context.i8_type().ptr_type(AddressSpace::Generic))
                }
                t => {
                    eprintln!("Invalid Primitive Type {:?}", t);
                    exit(-1)
                }
            },
            MLValueType::Struct(t) => AnyTypeEnum::from(self.module.get_struct_type(&*t).unwrap()),
            MLValueType::Pointer(p) | MLValueType::Reference(p) => {
                BasicTypeEnum::try_from(self.ml_type_to_type(*p))
                    .unwrap()
                    .ptr_type(AddressSpace::Generic)
                    .as_any_type_enum()
            }
        }
    }
}
