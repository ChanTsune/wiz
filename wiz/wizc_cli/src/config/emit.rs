#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emit {
    LlvmIr,
    Assembly,
    Object,
    Binary,
}

impl Emit {
    pub fn as_str(&self) -> &str {
        match self {
            Emit::LlvmIr => "llvm-ir",
            Emit::Assembly => "asm",
            Emit::Object => "obj",
            Emit::Binary => "bin",
        }
    }
}

impl From<&str> for Emit {
    fn from(value: &str) -> Self {
        match value {
            "llvm-ir" => Self::LlvmIr,
            "asm" => Self::Assembly,
            "obj" => Self::Object,
            _ => Self::Binary,
        }
    }
}
