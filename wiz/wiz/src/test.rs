use crate::build;
use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "test";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let build_options = build::Options::new(
        options.get_one::<&str>("manifest-path").copied(),
        options.get_one::<&str>("std").copied(),
        options.get_one::<&str>("target-dir").copied(),
        None,
        true,
    );
    build::command("", build_options)?;
    Ok(())
}
