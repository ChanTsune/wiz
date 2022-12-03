mod build;
mod check;
mod constant;
mod core;
mod external_subcommand;
mod init;
mod new;
mod run;
mod subcommand;
mod test;

use crate::build::BuildCommand;
use crate::core::{Cmd, Result};
use crate::run::RunCommand;
use crate::test::TestCommand;
use ansi_term::Color;
use clap::{Arg, ArgAction, Command, crate_version, value_parser};
use std::process::exit;

fn arg_target_triple() -> Arg {
    Arg::new("target-triple")
        .long("target-triple")
        .num_args(1)
        .help("Build target platform")
}

fn arg_manifest_path() -> Arg {
    Arg::new("manifest-path")
        .long("manifest-path")
        .num_args(1)
        .help("Path to the manifest file")
}

fn arg_std() -> Arg {
    Arg::new("std")
        .long("std")
        .num_args(1)
        .help("Use another std library")
}

fn cli() -> Result<()> {
    let app = Command::new("wiz")
        .version(crate_version!())
        .about("Wiz's package manager")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .external_subcommand_value_parser(value_parser!(String))
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
                        .action(ArgAction::SetTrue)
                        .long("overwrite")
                        .help("Overwrite files for target Directory"),
                ),
        )
        .subcommand(
            Command::new(BuildCommand::NAME)
                .about("Compile the current package")
                .arg(Arg::new("target-dir").help("Directory for all generated artifacts"))
                .arg(arg_target_triple())
                .arg(arg_manifest_path())
                .arg(arg_std())
                .arg(Arg::new("tests").action(ArgAction::SetTrue).long("tests")),
        )
        .subcommand(
            Command::new(check::COMMAND_NAME)
                .about("Check the current package")
                .arg(
                    Arg::new("manifest")
                        .action(ArgAction::SetTrue)
                        .long("manifest")
                        .help("Check manifest.toml"),
                )
                .arg(arg_manifest_path())
                .arg(arg_std()),
        )
        .subcommand(
            Command::new(RunCommand::NAME)
                .about("Run a binary or example of the local package")
                .arg(Arg::new("target-dir").help("Directory for all generated artifacts"))
                .arg(arg_target_triple())
                .arg(arg_manifest_path())
                .arg(arg_std()),
        )
        .subcommand(
            Command::new(TestCommand::NAME)
                .about("Run the tests")
                .arg(Arg::new("target-dir").help("Directory for all generated artifacts"))
                .arg(arg_manifest_path())
                .arg(arg_std()),
        )
        .arg(
            Arg::new("quite")
                .action(ArgAction::SetTrue)
                .short('q')
                .long("quite")
                .help("No output printed to stdout")
                .global(true),
        );
    let matches = app.get_matches();
    match matches.subcommand() {
        Some((new::COMMAND_NAME, option)) => new::command(new::COMMAND_NAME, option),
        Some((init::COMMAND_NAME, option)) => init::command(init::COMMAND_NAME, option),
        Some((BuildCommand::NAME, option)) => BuildCommand::execute(option),
        Some((check::COMMAND_NAME, option)) => check::command(check::COMMAND_NAME, option),
        Some((TestCommand::NAME, option)) => TestCommand::execute(option),
        Some((RunCommand::NAME, option)) => RunCommand::execute(option),
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
