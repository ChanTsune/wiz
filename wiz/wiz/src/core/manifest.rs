use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}

impl Dependency {
    pub fn simple<T: ToString>(version: T) -> Dependency {
        Dependency::Simple(version.to_string())
    }

    pub fn path<T: ToString>(path: T) -> Dependency {
        Dependency::Detailed(DetailedDependency {
            version: None,
            path: Some(path.to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub path: Option<String>,
}

pub fn read(path: &Path) -> Result<Manifest> {
    let file = std::fs::read_to_string(path)?;
    read_from_string(&file)
}

pub fn read_from_string(str: &str) -> Result<Manifest> {
    let manifest = toml::from_str(&str)?;
    Ok(manifest)
}

pub fn write(path: &Path, manifest: &Manifest) -> Result<()> {
    let file = toml::to_string(manifest)?;
    std::fs::write(path, file)?;
    Ok(())
}
