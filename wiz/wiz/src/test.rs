use crate::build;
use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "test";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    build::command(build::COMMAND_NAME, options)?;
    Ok(())
}
