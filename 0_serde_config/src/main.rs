use clap::Clap;
use serde::{Deserialize, Serialize};
use serde_json;

// Adding serde's Deserialize and Serialize lets us read/write our Config from/to anywhere in any
// format serde supports.
#[derive(Clap, Debug, Deserialize, Serialize)]
struct Config {
    /// A Foo argument
    #[clap(long = "foo", short = "f", default_value = "5")]
    foo: u64,
    /// A Bar argument
    #[clap(long = "bar", short = "b", default_value = "test")]
    bar: String,
}

fn main() {
    // Create a Config from command line options
    let config = Config::parse();
    println!("{:?}", config);

    // Create a Config from a json string. This could have been read from a config file.
    let config =
        serde_json::from_str::<Config>("{\"foo\": 7, \"bar\": \"a test message\"}").unwrap();
    println!("{:?}", config);
}
