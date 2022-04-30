mod build;
mod check;
mod constant;
mod core;
mod external_subcommand;
mod init;
mod new;
mod subcommand;

use crate::build::build_command;
use crate::check::check_command;
use crate::init::init_command;
use crate::new::new_command;
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
            Command::new("new")
                .about("Create a new wiz package at <path>")
                .arg(Arg::new("path").required(true)),
        )
        .subcommand(
            Command::new("init")
                .about("Create a new wiz package in an current directory")
                .arg(
                    Arg::new("overwrite")
                        .long("overwrite")
                        .help("Overwrite files for target Directory"),
                ),
        )
        .subcommand(
            Command::new("build")
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
                ),
        )
        .subcommand(
            Command::new("check")
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
        Some((cmd, option)) if cmd == "new" => {
            new_command(cmd, option)?;
        }
        Some((cmd, option)) if cmd == "init" => {
            init_command(cmd, option)?;
        }
        Some((cmd, option)) if cmd == "build" => {
            build_command(cmd, option)?;
        }
        Some((cmd, option)) if cmd == "check" => {
            check_command(cmd, option)?;
        }
        Some((cmd, option)) => {
            external_subcommand::try_execute(cmd, option)?;
        }
        _ => panic!(),
    }
    Ok(())
}

fn main() {
    if let Err(e) = cli() {
        eprintln!("{} {}", Color::Red.bold().paint("Error"), e);
        exit(-1)
    }
}
