use crate::{build, core::{Cmd, Result}};
use clap::ArgMatches;

pub(crate) struct RunCommand;

impl Cmd for RunCommand {
    const NAME: &'static str = "run";

    fn execute(args: &ArgMatches) -> Result<()> {
        let build_options = build::Options::new(
            args
                .get_one::<String>("manifest-path")
                .map(|i| i.as_str()),
            args.get_one::<String>("std").map(|i| i.as_str()),
            args.get_one::<String>("target-dir").map(|i| i.as_str()),
            None,
            false,
        );
        let output = build::command("", build_options)?;
        super::subcommand::execute(output, args.get_many::<String>("").unwrap_or_default())
    }
}
