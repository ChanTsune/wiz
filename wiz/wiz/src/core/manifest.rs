use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

pub fn read(path: &Path) -> Result<Manifest> {
    let file = std::fs::read_to_string(path)?;
    let manifest = toml::from_str(&file)?;
    Ok(manifest)
}

pub fn write(path: &Path, manifest: &Manifest) -> Result<()> {
    let file = toml::to_string(manifest)?;
    std::fs::write(path, file)?;
    Ok(())
}
