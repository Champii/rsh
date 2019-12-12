use clap::{App, Arg};

use super::config::Config;

pub fn parse_config() -> Config {
    let matches = App::new("RSH")
        .version("0.0.2")
        .author("Champii <contact@champii.io>")
        .about("Rust shell")
        .arg(
            Arg::with_name("script")
                .help("Script to run")
                .index(1)
                .required(false),
        )
        .get_matches();

    let script_path = matches
        .value_of("script")
        .map(std::string::ToString::to_string);

    Config { script_path }
}
