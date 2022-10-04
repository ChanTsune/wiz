use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: Dependencies,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Dependencies(pub(crate) BTreeMap<String, Dependency>);

impl<const N: usize> From<[(String, Dependency); N]> for Dependencies {
    fn from(attr: [(String, Dependency); N]) -> Self {
        Self(BTreeMap::from(attr))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

impl PackageInfo {
    pub fn new<N: ToString, V: ToString>(name: N, version: V) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
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

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dependency::Simple(v) => write!(f, "v{}", v),
            Dependency::Detailed(d) => Display::fmt(d, f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub path: Option<String>,
}

impl Display for DetailedDependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.version {
            write!(f, "v{}", v)?;
        }
        if let Some(p) = &self.path {
            write!(f, "({})", p)?;
        }
        Ok(())
    }
}

pub fn read(path: &Path) -> Result<Manifest> {
    let file = std::fs::read_to_string(path)?;
    read_from_string(&file)
}

pub fn read_from_string(str: &str) -> Result<Manifest> {
    Ok(toml::from_str(str)?)
}

pub fn write(path: &Path, manifest: &Manifest) -> Result<()> {
    let file = toml::to_string(manifest)?;
    Ok(std::fs::write(path, file)?)
}

#[cfg(test)]
mod tests {
    use super::PackageInfo;
    use crate::core::manifest::{Dependencies, Dependency, Manifest};
    use wiz_dev_utils::StringExt;

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

        assert_eq!(
            manifest,
            Manifest {
                package: PackageInfo::new("test", "0.0.0"),
                dependencies: Dependencies::from([
                    ("std".to_string(), Dependency::simple("0.0.0")),
                    ("local".to_string(), Dependency::path("../local"))
                ])
            }
        );
    }

    #[test]
    fn to_string() {
        let manifest = Manifest {
            package: PackageInfo::new("test", "0.0.0"),
            dependencies: Dependencies::from([("std".to_string(), Dependency::simple("0.0.0"))]),
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
