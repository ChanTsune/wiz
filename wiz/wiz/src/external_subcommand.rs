use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::iter::FromIterator;
use crate::core::Result;

pub(crate) fn try_execute(cmd: &str, options: &ArgMatches) -> Result<()> {
    let args = Vec::from_iter(options.values_of("").unwrap_or_default());
    let executable = format!("wiz-{}{}", cmd, env::consts::EXE_SUFFIX);
    super::subcommand::execute(&executable, &args)
}
