use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use serde::ser::{SerializeSeq, SerializeStruct};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: Dependencies,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Dependencies(pub(crate) BTreeMap<String, Dependency>);

impl <const N: usize> From<[(String, Dependency); N]> for Dependencies {
    fn from(attr: [(String, Dependency); N]) -> Self {
        Self(BTreeMap::from(attr))
    }
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

#[cfg(test)]
mod tests {
    use super::PackageInfo;
    use wiz_dev_utils::StringExt;
    use crate::core::manifest::{Dependencies, Dependency, Manifest};

    #[test]
    fn read_from_string() {
        let manifest = super::read_from_string(
            &r#"
            [package]
            name = "test"
            version = "0.0.0"

            [dependencies]
            std = "0.0.0"
            local = { path = "../local" }
            "#
            .trim_indent(),
        )
        .unwrap();

        assert_eq!(manifest, Manifest {
            package: PackageInfo { name: "test".to_string(), version: "0.0.0".to_string() },
            dependencies: Dependencies::from([
                ("std".to_string(), Dependency::simple("0.0.0")),
                ("local".to_string(), Dependency::path("../local"))
            ])
        });
    }

    #[test]
    fn to_string() {
        let manifest = Manifest {
            package: PackageInfo { name: "test".to_string(), version: "0.0.0".to_string() },
            dependencies: Dependencies::from([
                ("std".to_string(), Dependency::simple("0.0.0")),
            ])
        };
        assert_eq!(
            toml::to_string(&manifest).unwrap().trim_indent(),
            r#"
            [package]
            name = "test"
            version = "0.0.0"

            [dependencies]
            std = "0.0.0"
            "#
            .trim_indent()
        );
    }
}
