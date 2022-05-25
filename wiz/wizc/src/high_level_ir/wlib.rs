use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::Path;
use wiz_hir::typed_file::TypedSourceSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WLib {
    pub typed_ir: TypedSourceSet,
}

impl WLib {
    pub fn new(typed_ir: TypedSourceSet) -> WLib {
        WLib { typed_ir }
    }

    pub fn read_from(path: &Path) -> WLib {
        let file = std::fs::read_to_string(path).unwrap();
        let lib: WLib = serde_json::from_str(&file).unwrap();
        lib
    }

    pub fn write_to(&self, path: &Path) {
        let file = serde_json::to_string(self).unwrap();
        std::fs::write(path, file).unwrap();
    }
}
