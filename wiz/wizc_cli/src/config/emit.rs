#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emit {
    LlvmIr,
    Assembly,
    Object,
    Binary,
}

impl Emit {

    pub fn all_str() -> &'static [&'static str] {
        &["llvm-ir", "object", "asm", "bin"]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Emit::LlvmIr => "llvm-ir",
            Emit::Assembly => "asm",
            Emit::Object => "object",
            Emit::Binary => "bin",
        }
    }
}

impl From<&str> for Emit {
    fn from(value: &str) -> Self {
        match value {
            "llvm-ir" => Self::LlvmIr,
            "asm" => Self::Assembly,
            "object" => Self::Object,
            _ => Self::Binary,
        }
    }
}
