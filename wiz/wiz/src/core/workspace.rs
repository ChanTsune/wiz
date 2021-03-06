use crate::constant::MANIFEST_FILE_NAME;
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

pub(crate) fn construct_workspace_from(cws: &Path) -> Result<Workspace> {
    let cws = cws.to_path_buf();
    if !cws.is_dir() {
        return Err(Box::new(CliError::from(format!(
            "{} is not directory",
            cws.display()
        ))));
    }
    let manifest_path = cws.join(MANIFEST_FILE_NAME);
    if !manifest_path.exists() {
        return Err(Box::new(CliError::from(format!(
            "could not find `{}` in `{}`",
            MANIFEST_FILE_NAME,
            cws.display()
        ))));
    }
    Ok(Workspace { cws, manifest_path })
}
