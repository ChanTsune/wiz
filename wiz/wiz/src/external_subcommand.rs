use crate::error::{ProcessError, WizError};
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub(crate) fn try_execute(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut args = vec![cmd];
    args.extend(options.values_of("").unwrap_or_default());
    let executable = format!("wiz-{}{}", cmd, env::consts::EXE_SUFFIX);
    println!("external command {} {:?}", executable, args);
    let mut current_exe_path = env::current_exe()?;
    let _ = current_exe_path.pop();
    current_exe_path.push(executable);
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
