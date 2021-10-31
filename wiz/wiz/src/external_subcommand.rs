use clap::ArgMatches;
use std::env;
use std::error::Error;

pub(crate) fn try_execute(cmd: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut args = vec![cmd];
    args.extend(options.values_of("").unwrap_or_default());
    let executable = format!("wiz-{}{}", cmd, env::consts::EXE_SUFFIX);
    super::subcommand::execute(&executable, &args)
}
