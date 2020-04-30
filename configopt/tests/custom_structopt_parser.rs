use configopt::{configopt_fields, ConfigOpt};
use serde::{Deserialize, Serialize};
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct CustomString(String);

fn custom_parser(_s: &str) -> Result<CustomString, String> {
    Ok(CustomString(String::from("test")))
}

impl fmt::Display for CustomString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long, parse(try_from_str = custom_parser))]
    custom: CustomString,
}

#[test]
fn test_custom_structopt_parser() {}
