use crate::core::Result;
use crate::build;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "test";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let build_options = build::Options::new(
        options.value_of("manifest-path"),
        options.value_of("std"),
        options.value_of("target-dir"),
        None,
        true,
    );
    build::command("", build_options)?;
    Ok(())
}
