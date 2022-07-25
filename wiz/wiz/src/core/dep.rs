use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use crate::core::manifest;
use crate::core::manifest::{Dependency, Manifest};
use crate::core::Result;
use std::env;
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
) -> Result<ResolvedDependencyTree> {
    let home_dir = PathBuf::from(env!("HOME"));
    let builtin_package_dir = home_dir.join(".wiz/lib/src/");
    let package_index_cache_dir = home_dir.join(".wiz/repository/");
    let package_dirs = vec![builtin_package_dir, package_index_cache_dir];
    let mut result = Vec::with_capacity(manifest.dependencies.0.len());
    for (name, version) in manifest.dependencies.0.iter() {
        if let Some(std) = another_std {
            let manifest_path = PathBuf::from(std).join(name).join(MANIFEST_FILE_NAME);
            if manifest_path.exists() {
                let manifest = manifest::read(&manifest_path)?;
                let dependency =
                    resolve_manifest_dependencies(&manifest_path, &manifest, another_std)?;
                result.push(dependency);
                continue;
            }
        }

        let manifest_path = manifest_find_in(&package_dirs, (name, version))?;

        let manifest = manifest::read(&manifest_path)?;
        let dependency = resolve_manifest_dependencies(&manifest_path, &manifest, another_std)?;
        result.push(dependency);
    }
    Ok(ResolvedDependencyTree {
        name: manifest.package.name.clone(),
        version: manifest.package.version.clone(),
        src_path: manifest_path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap(),
        dependencies: result,
    })
}

fn manifest_find_in(
    find_dirs: &[PathBuf],
    (name, version): (&String, &Dependency),
) -> Result<PathBuf> {
    let manifest_path = find_dirs
        .iter()
        .map(|dir| match version {
            Dependency::Simple(version) => dir.join(name).join(version).join(MANIFEST_FILE_NAME),
            Dependency::Detailed(detail) => dir
                .join(detail.path.as_ref().unwrap())
                .join(MANIFEST_FILE_NAME),
        })
        .find(|manifest_path| manifest_path.exists());
    match manifest_path {
        Some(manifest_path) => Ok(manifest_path),
        None => Err(Box::new(CliError::from(format!(
            "Could not find dependency {} {:?}",
            name, version
        )))),
    }
}
