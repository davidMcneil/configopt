use clap::{App, ArgMatches, ArgSettings, Clap, FromArgMatches, IntoApp};
use parse_with_defaults_derive::ParseWithDefaults;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Clap, Debug, Deserialize, ParseWithDefaults, Serialize)]
struct Config {
    /// A Foo argument
    #[clap(long = "foo", short = "f")]
    foo: u64,
    /// A Bar argument
    #[clap(long = "bar", short = "b")]
    bar: String,
}

fn main() {
    // Create a partial Config from a json string. This could have been read from a config file.
    let partial_config = serde_json::from_str::<PartialConfig>(
        "{\"foo\": 5, \"bar\": \"default value from config\"}",
    )
    .unwrap();

    // Create a Config with defaults from the partial config
    let config = Config::parse_with_defaults(&partial_config);
    println!("{:?}", config);
}
