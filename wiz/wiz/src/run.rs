use crate::core::Result;
use crate::{BuildCommand, Cmd};
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    BuildCommand::execute(options)?;
    Ok(())
}
