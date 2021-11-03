use crate::core::error::CliError;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

pub(crate) struct Workspace {
    pub(crate) cws: PathBuf,
    pub(crate) manifest: PathBuf,
}

pub(crate) fn construct_workspace_from(cws: PathBuf) -> Result<Workspace, CliError> {
    if !cws.is_dir() {
        return Err(CliError::from(format!(
            "{} is not directory",
            cws.display()
        )));
    }
    let mut manifest = cws.clone();
    manifest.push("Package.wiz");
    if !manifest.exists() {
        return Err(CliError::from(format!(
            "Can not find `Package.wiz` in {}",
            cws.display()
        )));
    }
    Ok(Workspace { cws, manifest })
}
