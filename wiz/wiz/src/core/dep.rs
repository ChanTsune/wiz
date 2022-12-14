use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use crate::core::manifest::{Dependency, Manifest};
use crate::core::Result;
use crate::core::{manifest, WizContext};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ResolvedDependencyTree {
    pub name: String,
    pub version: String,
    pub src_path: PathBuf,
    pub dependencies: Vec<ResolvedDependencyTree>,
}

impl Display for ResolvedDependencyTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} v{} ({})",
            self.name,
            self.version,
            self.src_path.display()
        )?;
        for dependency in &self.dependencies {
            Display::fmt(dependency, f)?;
        }
        Ok(())
    }
}

pub fn resolve_manifest_dependencies(
    manifest_path: &Path,
    manifest: &Manifest,
    another_std: Option<&str>,
) -> Result<ResolvedDependencyTree> {
    let home_dir = WizContext::home();
    let builtin_package_dir = home_dir.join("lib/src/");
    let package_index_cache_dir = WizContext::git_dir();
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

        let manifest_path = manifest_find_in(manifest_path, &package_dirs, (name, version))?;

        let manifest = manifest::read(&manifest_path)?;
        let dependency = resolve_manifest_dependencies(&manifest_path, &manifest, another_std)?;
        result.push(dependency);
    }
    Ok(ResolvedDependencyTree {
        name: manifest.package.name.clone(),
        version: manifest.package.version.clone(),
        src_path: manifest_path
            .parent()
            .map(|p| p.join("src").join("lib.wiz"))
            .unwrap(),
        dependencies: result,
    })
}

fn manifest_find_in(
    parent_manifest_path: &Path,
    find_dirs: &[PathBuf],
    (name, version): (&String, &Dependency),
) -> Result<PathBuf> {
    let manifest_path = find_dirs
        .iter()
        .map(|dir| match version {
            Dependency::Simple(version) => dir.join(name).join(version).join(MANIFEST_FILE_NAME),
            Dependency::Detailed(detail) => {
                if detail.is_path() && detail.is_git() {
                    panic!("`path` and `git` both are specified.");
                } else if let Some(path) = &detail.path {
                    let p = PathBuf::from(path);
                    let p = if p.is_absolute() {
                        p.join(MANIFEST_FILE_NAME)
                    } else {
                        parent_manifest_path
                            .join(detail.path.as_ref().unwrap())
                            .join(MANIFEST_FILE_NAME)
                    }
                    .canonicalize()
                    .unwrap();
                    p
                } else if let Some(git) = &detail.git {
                    let clone_dir = WizContext::git_dir().join(name);
                    let repo = if clone_dir.exists() {
                        git2::Repository::open(clone_dir).unwrap()
                    } else {
                        git2::Repository::clone(git, clone_dir).unwrap()
                    };
                    let mut remote = repo.find_remote("origin").unwrap();
                    remote.fetch(&["master"], None, None).unwrap();
                    repo.path().parent().unwrap().join(MANIFEST_FILE_NAME)
                } else {
                    unreachable!()
                }
            }
        })
        .find(|manifest_path| manifest_path.exists());
    match manifest_path {
        Some(manifest_path) => Ok(manifest_path),
        None => Err(Box::new(CliError::from(format!(
            "Could not find dependency {} {}",
            name, version
        )))),
    }
}
