use crate::build;
use crate::core::Result;
use clap::ArgMatches;

pub(crate) const COMMAND_NAME: &str = "run";

pub(crate) fn command(_: &str, options: &ArgMatches) -> Result<()> {
    let build_options = build::Options::new(
        options.get_one::<String>("manifest-path").map(|i|i.as_str()),
        options.get_one::<String>("std").map(|i|i.as_str()),
        options.get_one::<String>("target-dir").map(|i|i.as_str()),
        None,
        false,
    );
    build::command("", build_options)?;
    Ok(())
}
