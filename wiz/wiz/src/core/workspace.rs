use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use crate::core::error::CliError;

pub(crate) struct Workspace {
    cws: PathBuf,
    manifest: PathBuf,
}

pub(crate) fn construct_workspace_from(cws: PathBuf) -> Result<Workspace, CliError> {
    if !cws.is_dir() {
       return Err(CliError::from(format!("{} is not directory", cws.display())))
    }
    let mut manifest = cws.clone();
    manifest.push("Package.wiz");
    if !manifest.exists() {
        return Err(CliError::from(format!("Can not find `Package.wiz` in {}", cws.display())))
    }
    Ok(    Workspace {
        cws,
        manifest
    }
    )
}
