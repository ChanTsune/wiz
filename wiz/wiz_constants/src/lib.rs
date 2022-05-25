// primitive types
pub const INT8: &str = "Int8"; // "i8";
pub const INT16: &str = "Int16"; // "i16";
pub const INT32: &str = "Int32"; // "i32";
pub const INT64: &str = "Int64"; // "i64";
pub const INT128: &str = "Int128"; // "i128";
pub const SIZE: &str = "Size"; // "size";
pub const UINT8: &str = "UInt8"; // "u8";
pub const UINT16: &str = "UInt16"; // "u16";
pub const UINT32: &str = "UInt32"; // "u32";
pub const UINT64: &str = "UInt64"; // "u64";
pub const UINT128: &str = "UInt128"; // "u128";
pub const USIZE: &str = "USize"; // "usize";

pub const BOOL: &str = "Bool"; // "bool";

pub const F32: &str = "Float"; // "f32";
pub const F64: &str = "Double"; // "f64";
pub const F128: &str = "Double128"; //"f128";

pub const UNIT: &str = "Unit"; // "unit";
pub const NOTING: &str = "Noting"; // "noting";

pub const STRING: &str = "str";

pub mod annotation {
    pub const BUILTIN: &str = "builtin";
    pub const NO_MANGLE: &str = "no_mangle";
}
