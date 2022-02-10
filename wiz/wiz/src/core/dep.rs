use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use crate::core::manifest;
use crate::core::manifest::Manifest;
use std::env;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ResolvedDependencyTree {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<ResolvedDependencyTree>,
}

pub fn resolve_manifest_dependencies(
    manifest: &Manifest,
) -> Result<ResolvedDependencyTree, Box<dyn Error>> {
    let home_dir =
        env::home_dir().ok_or_else(|| Box::new(CliError::from("Could not find home directory")))?;
    let builtin_package_dir = home_dir.join(".wiz/lib/");
    let package_index_cache_dir = home_dir.join(".wiz/repository/");
    let package_dirs = vec![builtin_package_dir, package_index_cache_dir];
    let mut result = Vec::new();
    for (name, version) in manifest.dependencies.iter() {
        let mut resolved = false;
        for package_dir in package_dirs.iter() {
            let manifest_path = package_dir
                .join(name)
                .join(version)
                .join(MANIFEST_FILE_NAME);
            if manifest_path.exists() {
                let manifest = manifest::read(&manifest_path)?;
                let dependency = resolve_manifest_dependencies(&manifest)?;
                result.push(dependency);
                resolved = true;
                break;
            }
        }
        if !resolved {
            println!("Could not find dependency {} {}", name, version);
        }
    }
    Ok(ResolvedDependencyTree {
        name: manifest.package.name.clone(),
        version: manifest.package.version.clone(),
        dependencies: result,
    })
}
