use crate::core::Result;
use clap::ArgMatches;
use std::env;
use std::iter::FromIterator;

pub(crate) fn try_execute(cmd: &str, options: &ArgMatches) -> Result<()> {
    let args = Vec::from_iter(options.get_many::<String>("").unwrap().map(AsRef::as_ref));
    let executable = format!("wiz-{}{}", cmd, env::consts::EXE_SUFFIX);
    super::subcommand::execute(&executable, &args)
}
