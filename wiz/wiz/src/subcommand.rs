use crate::core::error::{CliError, ProcessError};
use crate::core::Result;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
pub(crate) use clean::CleanCommand;

mod clean;

fn get_executable_path<P: AsRef<Path>>(executable: P) -> Result<PathBuf> {
    let path = env::current_exe()?;
    let path = path.with_file_name(executable.as_ref());
    if !path.exists() {
        return Err(Box::new(CliError::from(format!(
            "command `{}` could not find",
            executable.as_ref().display()
        ))));
    }
    Ok(path)
}

#[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
pub(crate) fn execute<I, S, P>(executable: P, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    use std::os::unix::process::CommandExt;
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

#[cfg(any(windows, doc))]
pub(crate) fn execute<I, S, P>(executable: P, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    output(executable, args)?;
    Ok(())
}

pub(crate) fn output<P, S, I>(executable: P, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let executable_path = get_executable_path(executable)?;
    let mut command = Command::new(executable_path);
    command.args(args);
    Ok(command.output()?)
}
