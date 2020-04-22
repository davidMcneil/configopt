use configopt::{configopt_fields, ConfigOpt};
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[structopt(rename_all = "camelcase")]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long)]
    maybe: bool,
    #[structopt(long)]
    #[serde(default)]
    numbers: Vec<u32>,
    #[structopt(long)]
    optional: Option<String>,
    #[structopt(long)]
    not_optional: String,
    #[structopt(long)]
    double_optional: Option<Option<f32>>,
    #[structopt(long)]
    optional_vec: Option<Vec<u32>>,
    #[structopt(long)]
    path: PathBuf,
    #[structopt(subcommand)]
    #[serde(skip)]
    cmd: MyEnum,
}

#[derive(ConfigOpt, StructOpt, Debug, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
enum MyEnum {
    Cmd1,
    Cmd2 {
        #[structopt(long)]
        field_1: String,
        #[structopt(long)]
        field_2: Option<String>,
    },
    Cmd3(AnotherStruct),
}

// TODO: Remove the need for implementing `Default`
impl Default for MyEnum {
    fn default() -> Self {
        Self::Cmd1
    }
}

#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct AnotherStruct {
    #[structopt(long)]
    field_a: String,
    #[structopt(long)]
    field_b: Option<String>,
    #[structopt(flatten)]
    #[serde(flatten)]
    flat_struct: FlatStruct,
}

#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct FlatStruct {
    #[structopt(long)]
    flat_optional: Option<u32>,
    #[structopt(long)]
    #[serde(default)]
    flat_maybe: bool,
    #[structopt(long)]
    #[serde(default)]
    flat_numbers: Vec<u32>,
}

fn main() {}
