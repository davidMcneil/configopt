use configopt::ConfigOpt;
use serde::Serialize;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug, Serialize))]
struct MyStruct {
    #[structopt(long)]
    positional: Vec<String>,
}

#[test]
fn positional_vec() {}
