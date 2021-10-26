use clap::ArgMatches;
use std::env;
use std::error::Error;

pub(crate) fn try_execute(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let executable = format!("wiz-{}{}", cmd, env::consts::EXE_SUFFIX);
    println!("external command {} {:?}", executable, options);
    let current_exe_path = env::current_exe()?;
    println!("{}", current_exe_path.display());
    Ok(())
}
