use crate::core::error::{CliError, ProcessError};
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Output};

fn get_executable_path(executable: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push(executable);
    if !path.exists() {
        return Err(Box::new(CliError::from(format!(
            "command `{}` could not find",
            executable
        ))));
    }
    Ok(path)
}

pub(crate) fn execute(executable: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let executable_path = get_executable_path(executable)?;
    let mut command = Command::new(executable_path);
    command.args(args);
    let err = command.exec();
    let error = anyhow::Error::from(err).context(ProcessError::new(None));
    if let Some(perr) = error.downcast_ref::<ProcessError>() {
        if let Some(code) = perr.code {
            return Err(Box::new(ProcessError::code(code)));
        }
    }
    Ok(())
}

pub(crate) fn output(executable: &str, args: &[&str]) -> Result<Output, Box<dyn Error>> {
    let executable_path = get_executable_path(executable)?;
    let mut command = Command::new(executable_path);
    command.args(args);
    Ok(command.output()?)
}
