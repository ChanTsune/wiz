use crate::middle_level_ir::format::Formatter;
use crate::middle_level_ir::ml_node::MLNode;
use crate::middle_level_ir::ml_stmt::MLBlock;
use crate::middle_level_ir::ml_type::MLType;
use std::fmt;
use std::fmt::Write;
use std::process::exit;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLExpr {
    Name(MLName),
    Literal(MLLiteral),
    Call(MLCall),
    PrimitiveBinOp(MLBinOp),
    PrimitiveUnaryOp(MLUnaryOp),
    Member(MLMember),
    If(MLIf),
    When,
    Return(MLReturn),
    TypeCast,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLName {
    pub(crate) name: String,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLLiteral {
    Integer { value: String, type_: MLType },
    FloatingPoint { value: String, type_: MLType },
    String { value: String, type_: MLType },
    Boolean { value: String, type_: MLType },
    Null { type_: MLType },
    Struct { type_: MLType },
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLCall {
    pub(crate) target: Box<MLExpr>,
    pub(crate) args: Vec<MLCallArg>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLCallArg {
    pub(crate) arg: MLExpr,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLIf {
    pub(crate) condition: Box<MLExpr>,
    pub(crate) body: MLBlock,
    pub(crate) else_body: Option<MLBlock>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLBinOp {
    pub(crate) left: Box<MLExpr>,
    pub(crate) kind: MLBinopKind,
    pub(crate) right: Box<MLExpr>,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLBinopKind {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Equal,
    GrateThanEqual,
    GrateThan,
    LessThanEqual,
    LessThan,
    NotEqual,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLUnaryOp {
    pub(crate) target: Box<MLExpr>,
    pub(crate) kind: MLUnaryOpKind,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub enum MLUnaryOpKind {
    Negative,
    Positive,
    Not,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLMember {
    pub(crate) target: Box<MLExpr>,
    pub(crate) name: String,
    pub(crate) type_: MLType,
}

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLReturn {
    pub(crate) value: Option<Box<MLExpr>>,
    pub(crate) type_: MLType,
}

impl MLExpr {
    pub fn type_(&self) -> MLType {
        match self {
            MLExpr::Name(n) => n.type_.clone(),
            MLExpr::Literal(l) => l.type_(),
            MLExpr::Call(c) => c.type_.clone(),
            MLExpr::PrimitiveBinOp(b) => b.type_.clone(),
            MLExpr::PrimitiveUnaryOp(b) => b.type_.clone(),
            MLExpr::Member(f) => f.type_.clone(),
            MLExpr::If(i) => i.type_.clone(),
            MLExpr::When => exit(-9),
            MLExpr::Return(r) => r.type_.clone(),
            MLExpr::TypeCast => exit(-9),
        }
    }
}

impl MLLiteral {
    pub fn type_(&self) -> MLType {
        match self {
            MLLiteral::Integer { value, type_ } => type_.clone(),
            MLLiteral::FloatingPoint { value, type_ } => type_.clone(),
            MLLiteral::String { value, type_ } => type_.clone(),
            MLLiteral::Boolean { value, type_ } => type_.clone(),
            MLLiteral::Null { type_ } => type_.clone(),
            MLLiteral::Struct { type_ } => type_.clone(),
        }
    }
}

impl MLCallArg {
    pub fn type_(&self) -> MLType {
        self.arg.type_()
    }
}

impl MLReturn {
    pub fn new(expr: MLExpr) -> Self {
        let type_ = expr.type_();
        MLReturn {
            value: Some(Box::new(expr)),
            type_: type_,
        }
    }
}

impl MLNode for MLExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLExpr::Name(n) => n.fmt(f),
            MLExpr::Literal(l) => l.fmt(f),
            MLExpr::Call(c) => c.fmt(f),
            MLExpr::PrimitiveBinOp(b) => b.fmt(f),
            MLExpr::PrimitiveUnaryOp(u) => u.fmt(f),
            MLExpr::Member(m) => m.fmt(f),
            MLExpr::If(i) => i.fmt(f),
            MLExpr::When => fmt::Result::Err(Default::default()),
            MLExpr::Return(r) => r.fmt(f),
            MLExpr::TypeCast => fmt::Result::Err(Default::default()),
        }
    }
}

impl MLNode for MLName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name)
    }
}

impl MLNode for MLLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MLLiteral::Integer { value, type_ } => f.write_str(value),
            MLLiteral::FloatingPoint { value, type_ } => f.write_str(value),
            MLLiteral::String { value, type_ } => {
                f.write_char('"')?;
                f.write_str(value)?;
                f.write_char('"')
            }
            MLLiteral::Boolean { value, type_ } => f.write_str(value),
            MLLiteral::Null { type_ } => fmt::Result::Err(Default::default()),
            MLLiteral::Struct { type_ } => {
                type_.fmt(f)?;
                f.write_str(" { }")
            }
        }
    }
}

impl MLNode for MLCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('(')?;
        for (c, arg) in self.args.iter().enumerate() {
            arg.fmt(f)?;
            let s = self.args.len() - 1;
            if s != c {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

impl MLNode for MLCallArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.arg.fmt(f)
    }
}

impl MLNode for MLBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.left.fmt(f)?;
        f.write_char(' ')?;
        f.write_str(match self.kind {
            MLBinopKind::Plus => "+",
            MLBinopKind::Minus => "-",
            MLBinopKind::Mul => "*",
            MLBinopKind::Div => "/",
            MLBinopKind::Mod => "%",
            MLBinopKind::Equal => "==",
            MLBinopKind::GrateThanEqual => "<=",
            MLBinopKind::GrateThan => "<",
            MLBinopKind::LessThanEqual => ">=",
            MLBinopKind::LessThan => ">",
            MLBinopKind::NotEqual => "!=",
        })?;
        f.write_char(' ')?;
        self.right.fmt(f)
    }
}

impl MLNode for MLUnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self.kind {
            MLUnaryOpKind::Negative => "-",
            MLUnaryOpKind::Positive => "+",
            MLUnaryOpKind::Not => "!",
        })?;
        self.target.fmt(f)
    }
}

impl MLNode for MLMember {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.target.fmt(f)?;
        f.write_char('.')?;
        f.write_str(&*self.name)
    }
}

impl MLNode for MLIf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("if (")?;
        self.condition.fmt(f)?;
        f.write_str(") ")?;
        self.body.fmt(f)?;
        match &self.else_body {
            Some(b) => {
                f.write_str(" else ")?;
                b.fmt(f)?;
            }
            None => {}
        };
        fmt::Result::Ok(())
    }
}

impl MLNode for MLReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("return ")?;
        match &self.value {
            Some(v) => v.fmt(f),
            None => fmt::Result::Ok(()),
        }
    }
}
