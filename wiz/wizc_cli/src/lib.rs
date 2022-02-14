mod config;

use clap::{App, Arg};
pub use config::build_type::BuildType;
pub use config::Config;

pub fn app(name: &str) -> App {
    let app = App::new(name)
        .arg(Arg::new("input").required(true))
        .arg(Arg::new("name").long("name").takes_value(true))
        .arg(
            Arg::new("type")
                .long("type")
                .takes_value(true)
                .possible_values(BuildType::all_str()),
        )
        .arg(Arg::new("output").short('o').takes_value(true))
        .arg(Arg::new("out-dir").long("out-dir").takes_value(true))
        .arg(
            Arg::new("target-triple")
                .long("target-triple")
                .takes_value(true),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("L")
                .short('L')
                .takes_value(true)
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("library")
                .long("library")
                .takes_value(true)
                .multiple_occurrences(true),
        );
    app
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
