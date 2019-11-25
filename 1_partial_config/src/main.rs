use clap::Clap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clap, Debug, Deserialize, Serialize)]
struct Config {
    /// A Foo argument
    #[clap(long = "foo", short = "f", default_value = "5")]
    foo: u64,
    /// A Bar argument
    #[clap(long = "bar", short = "b", default_value = "test")]
    bar: String,
}

// PartialConfig simply wraps every field from Config in an Option. This lets us patch a full
// config with values from a partial config. This is screaming to be converted into a macro. We
// will get to that.
#[derive(Debug, Default, Deserialize, Serialize)]
struct PartialConfig {
    foo: Option<u64>,
    bar: Option<String>,
}

impl PartialConfig {
    // For each field in the partial config, check if it exists. If it does, override it in the full
    // config.
    fn patch(self, full: &mut Config) {
        if let Some(foo) = self.foo {
            full.foo = foo;
        }
        if let Some(bar) = self.bar {
            full.bar = bar;
        }
    }
}

fn main() {
    // Create a full Config from command line options
    let mut config = Config::parse();
    println!("{:?}", config);

    // Create a partial Config from a json string. This could have been read from a config file.
    let partial_config = serde_json::from_str::<PartialConfig>("{\"foo\": 7}").unwrap();
    println!("{:?}", partial_config);

    // Merge the full config and the partial config dynamically updating our apps configuration. Wrapping the
    // config in a singleton like [state::Storage](https://sergio.bz/rustdocs/state/struct.Storage.html)
    // would give us a single source of truth for our app's configuration that is safe to update on the fly.
    partial_config.patch(&mut config);
    println!("{:?}", config);
}
