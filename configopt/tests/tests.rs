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
        yet_another: YetAnotherStruct,
    }

    #[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
    #[configopt(derive(Debug), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct YetAnotherStruct {
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

    let mut defaults = ConfigOptMyEnum::from_iter_safe(&["test", "cmd3", "--field-a=test"]).unwrap();
    let mut from_config = toml::from_str::<ConfigOptAnotherStruct>("flat_numbers = [7]\nflat_optional = 6").unwrap();
    println!("{:?}", from_config);
    assert_eq!(vec![7], from_config.yet_another.flat_numbers);
    match &mut defaults {
        ConfigOptMyEnum::Cmd3(s) => {
            s.patch(&mut from_config);
            assert_eq!(vec![7], s.yet_another.flat_numbers);
        },
        _ => {}
    }
    assert_eq!(Some(OsString::from("test")), defaults.arg_default(&[String::from("cmd3"), String::from("field-a")]));
    assert_eq!(Some(OsString::from("7")), defaults.arg_default(&[String::from("cmd3"), String::from("flat-numbers")]));
    println!("{:?}", defaults);
    

    let app = MyEnum::from_iter_safe_with_defaults(
        &["test", "cmd3", "--field-b=another"],
        &defaults
    ).unwrap();
    println!("{:?}", app);

}

// #[test]
// fn test_simple_configopt_defaults() {
// #[derive(StructOpt, Deserialize, Debug, Serialize)]
// // #[serde(deny_unknown_fields)]
// struct MakeCookie {
//     #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
//     supervising_faerie: String,
//     /// The faerie tree this cookie is being made in.
//     tree: Option<String>,
//     #[structopt(subcommand)] // Note that we mark a field as a subcommand
//     cmd: Command,
// }

// #[derive(StructOpt, Deserialize, Debug, Serialize)]
// enum Command {
//     /// Pound acorns into flour for cookie dough.
//     Pound {
//         acorns: u32,
//     },
//     /// Add magical sparkles -- the secret ingredient!
//     Sparkle {
//         #[structopt(short)]
//         magicality: u64,
//         #[structopt(short)]
//         color: String,
//     },
//     Finish(Finish),
// }

// // Subcommand can also be externalized by using a 1-uple enum variant
// #[derive(StructOpt, ConfigOpt, Deserialize, Debug, Serialize)]
// // #[serde(deny_unknown_fields)]
// struct Finish {
//     #[structopt(short)]
//     time: u32,
//     #[structopt(subcommand)] // Note that we mark a field as a subcommand
//     finish_type: FinishType,
// }

// // subsubcommand!
// #[derive(StructOpt, Deserialize, Debug, Serialize)]
// enum FinishType {
//     Glaze { applications: u32 },
//     Powder { flavor: String, dips: u32 },
// }

//     #[derive(StructOpt, Deserialize, Debug, Serialize)]
//     // #[serde(deny_unknown_fields)]
//     struct ConfigOptMakeCookie {
//         #[structopt(name = "supervisor", long = "supervisor")]
//         supervising_faerie: Option<String>,
//         /// The faerie tree this cookie is being made in.
//         tree: Option<String>,
//         #[structopt(subcommand)] // Note that we mark a field as a subcommand
//         cmd: Option<ConfigOptCommand>,
//     }

//     #[derive(StructOpt, Deserialize, Debug, Serialize)]
//     enum ConfigOptCommand {
//         /// Pound acorns into flour for cookie dough.
//         Pound {
//             acorns: Option<u32>,
//         },
//         /// Add magical sparkles -- the secret ingredient!
//         Sparkle {
//             #[structopt(short)]
//             magicality: Option<u64>,
//             #[structopt(short)]
//             color: Option<String>,
//         },
//         Finish(ConfigOptFinish),
//     }

//     // Subcommand can also be externalized by using a 1-uple enum variant
//     #[derive(StructOpt, Deserialize, Debug, Serialize)]
//     // #[serde(deny_unknown_fields)]
//     struct ConfigOptFinish {
//         #[structopt(short)]
//         time: Option<u32>,
//         #[structopt(subcommand)] // Note that we mark a field as a subcommand
//         finish_type: Option<ConfigOptFinishType>,
//     }

//     // subsubcommand!
//     #[derive(StructOpt, Deserialize, Debug, Serialize)]
//     enum ConfigOptFinishType {
//         Glaze {
//             applications: Option<u32>,
//         },
//         Powder {
//             flavor: Option<String>,
//             dips: Option<u32>,
//         },
//     }

//     let app = ConfigOptMakeCookie::from_iter_safe(&[""]).unwrap();
//     println!("{:?}", app);
//     let app = ConfigOptMakeCookie::from_iter_safe(&["", "pound"]).unwrap();
//     println!("{:?}", app);
//     let app = ConfigOptMakeCookie::from_iter_safe(&["", "finish", "glaze"]).unwrap();
//     println!("{:?}", app);

//     let s = r###""###;
//     let app: ConfigOptMakeCookie = toml::from_str(s).unwrap();
//     println!("{:?}", app);
//     let s = r###"
// supervising_faerie = "Henry"
// cmd.Pound.acorns = 42
//     "###;
//     let app: ConfigOptMakeCookie = toml::from_str(s).unwrap();
//     println!("{:?}", app);

//     let app = ConfigOptMakeCookie::from_iter_safe(&["", "pound", "53"]).unwrap();
//     println!("{:?}", app);
//     println!("{}", serde_json::to_string_pretty(&app).unwrap());
// }
