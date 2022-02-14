mod config;

use clap::{App, Arg};
pub use config::build_type::BuildType;
pub use config::Config;

pub fn app(name: &str) -> App {
    App::new(name)
        .arg(position("input").required(true))
        .arg(long("name").takes_value(true))
        .arg(
            long("type")
                .takes_value(true)
                .possible_values(BuildType::all_str()),
        )
        .arg(short("output", 'o').takes_value(true))
        .arg(long("out-dir").takes_value(true))
        .arg(long("target-triple").takes_value(true))
        .arg(
            short("path", 'p')
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(short("L", 'L').takes_value(true).multiple_occurrences(true))
        .arg(long("library").takes_value(true).multiple_occurrences(true))
}

fn position(name: &str) -> Arg {
    Arg::new(name)
}

fn long(name: &str) -> Arg {
    Arg::new(name).long(name)
}

fn short(name: &str, s: char) -> Arg {
    Arg::new(name).short(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
