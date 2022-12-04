use std::env;
use std::path::PathBuf;
use crate::random;

pub fn temp_dir() -> PathBuf {
    env::temp_dir().join(random::ascii_string(12))
}
