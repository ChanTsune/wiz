use crate::core::Result;
use clap::ArgMatches;
use crate::{BuildCommand, Cmd};

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    BuildCommand::execute(options)?;
    Ok(())
}
