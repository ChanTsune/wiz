use crate::constant::MANIFEST_FILE_NAME;
use crate::core::error::CliError;
use std::error::Error;
use std::path::PathBuf;
use crate::manifest::{self, Manifest};

pub(crate) struct Workspace {
    pub(crate) cws: PathBuf,
    pub(crate) manifest_path: PathBuf,
}

impl Workspace {
    pub(crate) fn get_manifest(&self) -> Result<Manifest, Box<dyn Error>> {
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

pub(crate) fn construct_workspace_from(cws: PathBuf) -> Result<Workspace, Box<dyn Error>> {
    if !cws.is_dir() {
        return Err(Box::new(CliError::from(format!(
            "{} is not directory",
            cws.display()
        ))));
    }
    let mut manifest = cws.clone();
    manifest.push(MANIFEST_FILE_NAME);
    if !manifest.exists() {
        return Err(Box::new(CliError::from(format!(
            "could not find `Package.wiz` in `{}`",
            cws.display()
        ))));
    }
    Ok(Workspace {
        cws,
        manifest_path: manifest,
    })
}
