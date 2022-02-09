mod build;
mod constant;
mod core;
mod external_subcommand;
mod init;
mod new;
mod subcommand;

use crate::build::build_command;
use crate::init::init_command;
use crate::new::new_command;
use ansi_term::Color;
use clap::{crate_version, App, AppSettings, Arg};
use std::error::Error;
use std::process::exit;

fn _main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .version(crate_version!())
        .about("Wiz's package manager")
        .setting(AppSettings::ArgRequiredElseHelp | AppSettings::AllowExternalSubcommands)
        .subcommand(
            App::new("new")
                .about("Create a new wiz package at <path>")
                .arg(Arg::new("path").required(true)),
        )
        .subcommand(
            App::new("init")
                .about("Create a new wiz package in an current directory")
                .arg(
                    Arg::new("overwrite")
                        .long("overwrite")
                        .help("Overwrite files for target Directory"),
                ),
        )
        .subcommand(
            App::new("build")
                .about("Compile the current package")
                .arg(Arg::new("target-dir").help("Directory for all generated artifacts"))
                .arg(
                    Arg::new("target-triple")
                        .long("target-triple")
                        .takes_value(true)
                        .help("Build target platform"),
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
        Some((cmd, option)) => {
            external_subcommand::try_execute(cmd, option)?;
        }
        _ => {
            panic!()
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        eprintln!("{} {}", Color::Red.bold().paint("Error"), e);
        exit(-1)
    }
}
