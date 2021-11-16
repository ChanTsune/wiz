use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use std::path::PathBuf;

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
    manifest.push(MANIFEST_FILE_NAME);
    if !manifest.exists() {
        return Err(CliError::from(format!(
            "could not find `Package.wiz` in `{}`",
            cws.display()
        )));
    }
    Ok(Workspace { cws, manifest })
}
