use clap::ArgMatches;
use crate::core::Result;

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    crate::build::command(crate::build::COMMAND_NAME, options)?;
    Ok(())
}
