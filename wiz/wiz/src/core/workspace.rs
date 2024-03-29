use crate::core::error::CliError;
use crate::core::manifest::{self, Manifest};
use crate::core::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Workspace {
    pub(crate) cws: PathBuf,
    pub(crate) manifest_path: PathBuf,
}

impl Workspace {
    pub(crate) fn get_manifest(&self) -> Result<Manifest> {
        if self.manifest_path.exists() {
            Ok(manifest::read(&self.manifest_path)?)
        } else {
            Err(Box::new(CliError::from(format!(
                "Manifest file not found at {}",
                self.manifest_path.display()
            ))))
        }
    }
}

pub(crate) fn construct_workspace_from(manifest_path: &Path) -> Result<Workspace> {
    let manifest_path = manifest_path.to_owned();
    if !manifest_path.exists() {
        return Err(Box::new(CliError::from(format!(
            "could not find `{}`",
            manifest_path.display()
        ))));
    }
    let cws = manifest_path.parent().expect("").to_owned();
    if !cws.is_dir() {
        return Err(Box::new(CliError::from(format!(
            "{} is not directory",
            cws.display()
        ))));
    }
    Ok(Workspace { cws, manifest_path })
}
