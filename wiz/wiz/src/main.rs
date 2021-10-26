mod common;
mod init;
mod new;

use crate::init::init_command;
use crate::new::new_command;
use ansi_term::Color;
use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process::exit;

fn _main() -> Result<(), Box<dyn Error>> {
    let app = App::new("wiz")
        .subcommand(
            SubCommand::with_name("new")
                .arg(Arg::with_name("path").required(true))
                .arg(
                    Arg::with_name("quite")
                        .short("q")
                        .long("quite")
                        .help("No output printed to stdout"),
                )
                .help("Create a new wiz package at <path>"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("quite")
                        .short("q")
                        .long("quite")
                        .help("No output printed to stdout"),
                )
                .help("Create a new wiz package in an current directory."),
        );
    let matches = app.get_matches();
    match matches.subcommand() {
        ("new", Some(option)) => {
            new_command("new", option)?;
        }
        ("init", Some(option)) => {
            init_command("init", option)?;
        }
        _ => {}
    }
    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        eprintln!("{} {}", Color::Red.bold().paint("Error"), e);
        exit(-1)
    }
}
