use configopt::{configopt_fields, ConfigOpt};
use serde::Deserialize;
use structopt::StructOpt;

#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long, default_value = "true")]
    maybe: Option<bool>,
}
fn main() {}
