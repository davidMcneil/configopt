use clap::FromArgMatches;
use clap::IntoApp;
use clap::{ArgSettings, Clap};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Clap, Debug, Deserialize, Serialize)]
struct Config {
    /// A Foo argument
    #[clap(long = "foo", short = "f")]
    foo: u64,
    /// A Bar argument
    #[clap(long = "bar", short = "b")]
    bar: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct PartialConfig {
    foo: Option<u64>,
    bar: Option<String>,
}

impl PartialConfig {
    #[allow(dead_code)]
    fn patch(self, full: &mut Config) {
        if let Some(foo) = self.foo {
            full.foo = foo;
        }
        if let Some(bar) = self.bar {
            full.bar = bar;
        }
    }
}

// This is a bit of a hack. Clap's default values[1] are stringly typed. Not only that but they take
// a `&str` and therefore we have to make sure the string has a sufficient lifetime. To achieve this
// we create a "stringy" version of the PartialConfig that is the `ToString` representation of each
// field.
//
// [1] https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.default_value
#[derive(Debug, Default)]
struct StringyPartialConfig {
    foo: Option<String>,
    bar: Option<String>,
}

impl StringyPartialConfig {
    // Create a new stringy version of the partial config by calling `to_string` on each of its
    // fields.
    fn new(partial: &PartialConfig) -> Self {
        Self {
            foo: partial.foo.as_ref().map(|v| v.to_string()),
            bar: partial.bar.as_ref().map(|v| v.to_string()),
        }
    }
}

fn main() {
    // Create a partial Config from a json string. This could have been read from a config file.
    let partial_config = serde_json::from_str::<PartialConfig>(
        "{\"foo\": 5, \"bar\": \"default value from config\"}",
    )
    .unwrap();

    // Convert the Config into a clap app
    let mut app = Config::into_app();

    // Make a clone of the app and set all of its arguments to be optional.
    let mut optional_app = app.clone();
    for arg in &app.args.args {
        optional_app =
            optional_app.mut_arg(arg.name, |arg| arg.unset_setting(ArgSettings::Required));
    }

    // Get the matches from the command line for the totally optional app. We do not
    // want to display the help message here so we remove the `-h` or `--h` options. We defer
    // showing the help message here so the help message will include any overridden default values.
    //
    // Because all arguments are optional and we removed the help options, `get_matches_from`
    // should always succeed unless the user supplies invalid arguments or the --version argument
    // is used.
    let args = env::args()
        .filter(|a| a != "-h" && a != "--h")
        .collect::<Vec<_>>();
    let matches = optional_app.get_matches_from(args);

    // Create a stringy version of the partial config. This struct has a lifetime sufficient
    // for updating the app's default values.
    let stringy_partial_config = StringyPartialConfig::new(&partial_config);

    // Modify the default values of the app to account for the values in stringy_partial_config.
    // For every argument, if the argument is set in stringy_partial_config and it did not
    // appear on the command line set its default value to be the value from stringy_partial_config.
    if let Some(foo) = &stringy_partial_config.foo {
        if !matches.is_present("foo") {
            app = app.mut_arg("foo", |arg| arg.default_value(foo));
        }
    }
    if let Some(bar) = &stringy_partial_config.bar {
        if !matches.is_present("bar") {
            app = app.mut_arg("bar", |arg| arg.default_value(bar));
        }
    }

    // Use the updated app to get matches and convert that into a Config
    let matches = app.get_matches();
    let config = Config::from_argmatches(&matches);

    println!("{:?}", config);
}
