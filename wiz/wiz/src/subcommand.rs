use crate::core::error::{CliError, ProcessError};
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub(crate) fn execute(executable: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut current_exe_path = env::current_exe()?;
    let _ = current_exe_path.pop();
    current_exe_path.push(executable);
    if !current_exe_path.exists() {
        return Err(Box::new(CliError::from(format!(
            "command `{}` can not find",
            executable
        ))));
    }
    let mut command = Command::new(current_exe_path.as_os_str());
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
