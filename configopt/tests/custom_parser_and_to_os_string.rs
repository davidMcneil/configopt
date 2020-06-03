use configopt::{configopt_fields, ConfigOpt, ConfigOptArgToOsString, ConfigOptType};
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct CustomString(String);

fn custom_parser(_s: &str) -> Result<CustomString, String> {
    Ok(CustomString(String::from("custom_parser")))
}

fn custom_to_os_string(_c: &CustomString) -> OsString {
    OsString::from("custom_to_os_string")
}

#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long, parse(try_from_str = custom_parser))]
    #[configopt(to_os_string = custom_to_os_string)]
    custom: CustomString,
}

#[test]
fn test_custom_parser_and_to_os_string() {
    let c = ConfigOptMyStruct {
        custom: Some(CustomString(String::from("serde"))),
        config_files: Vec::new(),
        generate_config: None,
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    assert_eq!(s.custom.0, "custom_parser");
    assert_eq!(c.toml_config(), "custom = \"serde\"\n\n");
    assert_eq!(
        c.arg_to_os_string(&[String::from("custom")]).unwrap(),
        "custom_to_os_string"
    );
}
