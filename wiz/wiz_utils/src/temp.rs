use crate::random;
use std::env;
use std::path::PathBuf;

pub fn temp_dir() -> PathBuf {
    env::temp_dir().join(random::ascii_string(12))
}
