use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use crate::core::manifest;
use crate::core::manifest::Manifest;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ResolvedDependencyTree {
    pub name: String,
    pub version: String,
    pub src_path: String,
    pub dependencies: Vec<ResolvedDependencyTree>,
}

pub fn resolve_manifest_dependencies(
    manifest_path: &Path,
    manifest: &Manifest,
    another_std: Option<&str>,
) -> Result<ResolvedDependencyTree, Box<dyn Error>> {
    let home_dir = PathBuf::from(env!("HOME"));
    let builtin_package_dir = home_dir.join(".wiz/lib/src/");
    let package_index_cache_dir = home_dir.join(".wiz/repository/");
    let package_dirs = vec![builtin_package_dir, package_index_cache_dir];
    let mut result = Vec::new();
    for (name, version) in manifest.dependencies.iter() {
        let mut resolved = false;

        if let Some(std) = another_std {
            let manifest_path = PathBuf::from(std).join(name).join(MANIFEST_FILE_NAME);
            if manifest_path.exists() {
                let manifest = manifest::read(&manifest_path)?;
                let dependency = resolve_manifest_dependencies(&manifest_path, &manifest, another_std)?;
                result.push(dependency);
                resolved = true;
            }
        }

        for package_dir in package_dirs.iter() {
            let manifest_path = package_dir
                .join(name)
                .join(version)
                .join(MANIFEST_FILE_NAME);
            if !resolved && manifest_path.exists() {
                let manifest = manifest::read(&manifest_path)?;
                let dependency =
                    resolve_manifest_dependencies(&manifest_path, &manifest, another_std)?;
                result.push(dependency);
                resolved = true;
                break;
            }
        }
        if !resolved {
            return Err(Box::new(CliError::from(format!(
                "Could not find dependency {} {}",
                name, version
            ))));
        }
    }
    Ok(ResolvedDependencyTree {
        name: manifest.package.name.clone(),
        version: manifest.package.version.clone(),
        src_path: manifest_path
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        dependencies: result,
    })
}
