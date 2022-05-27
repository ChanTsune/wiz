mod build;
mod check;
mod constant;
mod core;
mod external_subcommand;
mod init;
mod new;
mod subcommand;
mod test;

use ansi_term::Color;
use clap::{crate_version, Arg, Command};
use std::error::Error;
use std::process::exit;

fn cli() -> Result<(), Box<dyn Error>> {
    let app = Command::new("wiz")
        .version(crate_version!())
        .about("Wiz's package manager")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(new::COMMAND_NAME)
                .about("Create a new wiz package at <path>")
                .arg(Arg::new("path").required(true)),
        )
        .subcommand(
            Command::new(init::COMMAND_NAME)
                .about("Create a new wiz package in an current directory")
                .arg(
                    Arg::new("overwrite")
                        .long("overwrite")
                        .help("Overwrite files for target Directory"),
                ),
        )
        .subcommand(
            Command::new(build::COMMAND_NAME)
                .about("Compile the current package")
                .arg(Arg::new("target-dir").help("Directory for all generated artifacts"))
                .arg(
                    Arg::new("target-triple")
                        .long("target-triple")
                        .takes_value(true)
                        .help("Build target platform"),
                )
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .takes_value(true)
                        .help("Path to the manifest file"),
                )
                .arg(
                    Arg::new("std")
                        .long("std")
                        .takes_value(true)
                        .help("Use another std library"),
                ),
        )
        .subcommand(
            Command::new(check::COMMAND_NAME)
                .about("Check the current package")
                .arg(
                    Arg::new("manifest")
                        .long("manifest")
                        .help("Check manifest.toml"),
                )
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .takes_value(true)
                        .help("Path to the manifest file"),
                )
                .arg(
                    Arg::new("std")
                        .long("std")
                        .takes_value(true)
                        .help("Use another std library"),
                ),
        )
        .subcommand(
            Command::new(test::COMMAND_NAME)
                .about("Run the tests")
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .takes_value(true)
                        .help("Path to the manifest file"),
                )
                .arg(
                    Arg::new("std")
                        .long("std")
                        .takes_value(true)
                        .help("Use another std library"),
                ),
        )
        .arg(
            Arg::new("quite")
                .short('q')
                .long("quite")
                .help("No output printed to stdout")
                .global(true),
        );
    let matches = app.get_matches();
    match matches.subcommand() {
        Some((new::COMMAND_NAME, option)) => new::command(new::COMMAND_NAME, option),
        Some((init::COMMAND_NAME, option)) => init::command(init::COMMAND_NAME, option),
        Some((build::COMMAND_NAME, option)) => build::command(build::COMMAND_NAME, option),
        Some((check::COMMAND_NAME, option)) => check::command(check::COMMAND_NAME, option),
        Some((test::COMMAND_NAME, option)) => test::command(test::COMMAND_NAME, option),
        Some((cmd, option)) => external_subcommand::try_execute(cmd, option),
        _ => panic!(),
    }?;
    Ok(())
}

fn main() {
    if let Err(e) = cli() {
        eprintln!("{} {}", Color::Red.bold().paint("Error"), e);
        exit(-1)
    }
}
