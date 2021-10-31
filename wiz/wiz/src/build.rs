use clap::ArgMatches;
use std::env;
use std::error::Error;

pub(crate) fn build_command(_: &str, options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let wizc = "wizc";
    let mut args = vec![wizc];
    let target_dir = if let Some(target_dir) = options.value_of("target-dir") {
        String::from(target_dir)
    } else {
        String::from(env::current_dir().unwrap().to_str().unwrap())
    };
    args.extend(["--target-dir", &target_dir]);
    super::subcommand::execute(wizc, &args)
}
