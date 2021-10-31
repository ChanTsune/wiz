mod build;
mod core;
mod error;
mod external_subcommand;
mod init;
mod new;
mod subcommand;

use crate::build::build_command;
use crate::init::init_command;
use crate::new::new_command;
use ansi_term::Color;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use std::error::Error;
use std::process::exit;

fn _main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .version(crate_version!())
        .about("Wiz's package manager")
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::AllowExternalSubcommands,
        ])
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new wiz package at <path>")
                .arg(Arg::with_name("path").required(true)),
        )
        .subcommand(
            SubCommand::with_name("init").about("Create a new wiz package in an current directory"),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Compile the current package")
                .arg(Arg::with_name("target-dir").help("Directory for all generated artifacts")),
        )
        .arg(
            Arg::with_name("quite")
                .short("q")
                .long("quite")
                .help("No output printed to stdout")
                .global(true),
        );
    let matches = app.get_matches();
    match matches.subcommand() {
        ("new", Some(option)) => {
            new_command("new", option)?;
        }
        ("init", Some(option)) => {
            init_command("init", option)?;
        }
        ("build", Some(option)) => {
            build_command("build", option)?;
        }
        (cmd, Some(option)) => {
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
