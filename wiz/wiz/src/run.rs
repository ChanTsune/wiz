use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    crate::build::command(crate::build::COMMAND_NAME, options)?;
    Ok(())
}
