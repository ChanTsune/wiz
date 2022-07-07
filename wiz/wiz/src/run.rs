use crate::core::Result;
use clap::ArgMatches;
use crate::{BuildCommand, Cmd};

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    crate::build::command(BuildCommand::NAME, options)?;
    Ok(())
}
