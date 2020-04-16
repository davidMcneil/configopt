use configopt::{configopt_fields, ConfigOpt, ConfigOptDefaults};
use serde::Deserialize;
use std::{ffi::OsString, path::PathBuf};
use structopt::StructOpt;

const DEFAULT_VALUE: &str = "5";
const MY_ENVAR: &str = "MY_ENVAR";

#[test]
fn test_basic() {
    #[configopt_fields]
    #[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
    #[structopt(rename_all = "camelcase")]
    #[configopt(derive(Debug), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct MyStruct {
        #[structopt(long)]
        maybe: bool,
        #[structopt(long)]
        numbers: Vec<u32>,
        #[structopt(long)]
        optional: Option<String>,
        #[structopt(long, env = MY_ENVAR, default_value = DEFAULT_VALUE)]
        not_optional: String,
        #[structopt(long)]
        double_optional: Option<Option<f32>>,
        #[structopt(long)]
        optional_vec: Option<Vec<u32>>,
        #[structopt(long)]
        path: PathBuf,
        #[structopt(long, default_value = "Some Default")]
        name: String,
        #[structopt(subcommand)]
        #[serde(skip)]
        cmd: MyEnum,
    }

    #[derive(ConfigOpt, StructOpt, Debug)]
    #[configopt(derive(Debug), attrs(serde))]
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

    // TODO: Remove this
    impl Default for MyEnum {
        fn default() -> Self {
            Self::Cmd1
        }
    }

    #[configopt_fields]
    #[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
    #[configopt(derive(Debug), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct AnotherStruct {
        #[structopt(long)]
        field_a: String,
        #[structopt(long)]
        #[serde(skip)]
        field_b: Option<String>,
        #[structopt(flatten)]
        #[serde(flatten)]
        flat_struct: FlatStruct,
    }

    #[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
    #[configopt(derive(Debug), attrs(serde))]
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

    assert!(ConfigOptMyStruct::from_iter_safe(&["test"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "--numbers", "1", "2", "5"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "cmd3"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "cmd3", "--field-a=test"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "--generateConfig"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "cmd3", "--generate-config"]).is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&[
        "test",
        "--generateConfig",
        "cmd3",
        "--generate-config"
    ])
    .is_ok());
    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "--configFiles=test1.txt",]).is_ok());

    let mut defaults =
        ConfigOptMyEnum::from_iter_safe(&["test", "cmd3", "--field-a=test"]).unwrap();
    let mut from_config =
        toml::from_str::<ConfigOptAnotherStruct>("flat_numbers = [7]\nflat_optional = 6").unwrap();
    assert_eq!(vec![7], from_config.flat_struct.flat_numbers);
    match &mut defaults {
        ConfigOptMyEnum::Cmd3(s) => {
            s.patch(&mut from_config);
            assert_eq!(vec![7], s.flat_struct.flat_numbers);
        }
        _ => {}
    }
    assert_eq!(
        Some(OsString::from("test")),
        defaults.arg_default(&[String::from("cmd3"), String::from("field-a")])
    );
    assert_eq!(
        Some(OsString::from("7")),
        defaults.arg_default(&[String::from("cmd3"), String::from("flat-numbers")])
    );

    assert!(MyEnum::from_iter_safe_with_defaults(
        &["test", "cmd3", "--field-b=another"],
        &defaults
    )
    .is_ok());

    assert!(ConfigOptMyStruct::from_iter_safe(&["test", "--maybe", "cmd3"]).is_ok());
}
