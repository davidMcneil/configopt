use configopt::{configopt_fields, ConfigOpt, ConfigOptArgToOsString, ConfigOptType};
use serde::Deserialize;
use std::{ffi::OsString, path::PathBuf};
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
    #[allow(clippy::option_option)]
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

const MY_STRUCT_ARGS: &[&str] = &[
    "--maybe",
    "--numbers",
    "1",
    "2",
    "3",
    "--optional=from_cli1",
    "--notOptional=from_cli2",
    "--doubleOptional=1.5",
    "--optionalVec",
    "4",
    "5",
    "--path=/some/path",
];
const MY_ENUM_CMD2_ARGS: &[&str] = &["cmd2", "--field-1=from_cli3", "--field-2=from_cli4"];
const MY_ENUM_CMD3_ARGS: &[&str] = &[
    "cmd3",
    "--field-a=from_cli4",
    "--field-b=from_cli5",
    "--flat-optional=6",
    "--flat-maybe",
    "--flat-numbers=7",
];

#[test]
fn test_configopt_generate_config() {
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: None,
            config_files: Vec::new(),
            generate_config: Some(true),
        },
        ConfigOptMyStruct::from_iter_safe(&["app", "--generate-config"]).unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd3(ConfigOptAnotherStruct {
                field_a: None,
                field_b: None,
                flat_struct: ConfigOptFlatStruct {
                    flat_optional: None,
                    flat_maybe: None,
                    flat_numbers: Vec::new(),
                },
                config_files: Vec::new(),
                generate_config: Some(true)
            })),
            config_files: Vec::new(),
            generate_config: None,
        },
        ConfigOptMyStruct::from_iter_safe(&["app", "cmd3", "--generate-config"]).unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd3(ConfigOptAnotherStruct {
                field_a: None,
                field_b: None,
                flat_struct: ConfigOptFlatStruct {
                    flat_optional: None,
                    flat_maybe: None,
                    flat_numbers: Vec::new(),
                },
                config_files: Vec::new(),
                generate_config: Some(true)
            })),
            config_files: Vec::new(),
            generate_config: Some(true),
        },
        ConfigOptMyStruct::from_iter_safe(&[
            "app",
            "--generate-config",
            "cmd3",
            "--generate-config"
        ])
        .unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd3(ConfigOptAnotherStruct {
                field_a: None,
                field_b: None,
                flat_struct: ConfigOptFlatStruct {
                    flat_optional: None,
                    flat_maybe: None,
                    flat_numbers: Vec::new(),
                },
                config_files: Vec::new(),
                generate_config: Some(false)
            })),
            config_files: Vec::new(),
            generate_config: Some(true),
        },
        ConfigOptMyStruct::from_iter_safe(&[
            "app",
            "--generate-config=true",
            "cmd3",
            "--generate-config=false"
        ])
        .unwrap()
    );
}

#[test]
fn test_configopt_from_cli_no_args() {
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: None,
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(&["app"]).unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd1),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(&["app", "cmd1"]).unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd2 {
                field_1: None,
                field_2: None,
            }),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(&["app", "cmd2"]).unwrap()
    );
    assert_eq!(
        ConfigOptMyStruct {
            maybe: None,
            numbers: Vec::new(),
            optional: None,
            not_optional: None,
            double_optional: None,
            optional_vec: None,
            path: None,
            cmd: Some(ConfigOptMyEnum::Cmd3(ConfigOptAnotherStruct {
                field_a: None,
                field_b: None,
                flat_struct: ConfigOptFlatStruct {
                    flat_optional: None,
                    flat_maybe: None,
                    flat_numbers: Vec::new(),
                },
                config_files: Vec::new(),
                generate_config: None
            })),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(&["app", "cmd3"]).unwrap()
    );
}

#[test]
fn test_configopt_from_cli_all_args() {
    let mut cli = Vec::new();
    cli.push("app");
    cli.extend(MY_STRUCT_ARGS);
    cli.push("cmd1");
    assert_eq!(
        ConfigOptMyStruct {
            maybe: Some(true),
            numbers: vec![1, 2, 3],
            optional: Some(String::from("from_cli1")),
            not_optional: Some(String::from("from_cli2")),
            double_optional: Some(Some(1.5)),
            optional_vec: Some(vec![4, 5]),
            path: Some(PathBuf::from("/some/path")),
            cmd: Some(ConfigOptMyEnum::Cmd1),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(cli).unwrap()
    );
    let mut cli = Vec::new();
    cli.push("app");
    cli.extend(MY_STRUCT_ARGS);
    cli.extend(MY_ENUM_CMD2_ARGS);
    assert_eq!(
        ConfigOptMyStruct {
            maybe: Some(true),
            numbers: vec![1, 2, 3],
            optional: Some(String::from("from_cli1")),
            not_optional: Some(String::from("from_cli2")),
            double_optional: Some(Some(1.5)),
            optional_vec: Some(vec![4, 5]),
            path: Some(PathBuf::from("/some/path")),
            cmd: Some(ConfigOptMyEnum::Cmd2 {
                field_1: Some(String::from("from_cli3")),
                field_2: Some(String::from("from_cli4"))
            }),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(cli).unwrap()
    );
    let mut cli = Vec::new();
    cli.push("app");
    cli.extend(MY_STRUCT_ARGS);
    cli.extend(MY_ENUM_CMD3_ARGS);
    assert_eq!(
        ConfigOptMyStruct {
            maybe: Some(true),
            numbers: vec![1, 2, 3],
            optional: Some(String::from("from_cli1")),
            not_optional: Some(String::from("from_cli2")),
            double_optional: Some(Some(1.5)),
            optional_vec: Some(vec![4, 5]),
            path: Some(PathBuf::from("/some/path")),
            cmd: Some(ConfigOptMyEnum::Cmd3(ConfigOptAnotherStruct {
                field_a: Some(String::from("from_cli4")),
                field_b: Some(String::from("from_cli5")),
                flat_struct: ConfigOptFlatStruct {
                    flat_optional: Some(6),
                    flat_maybe: Some(true),
                    flat_numbers: vec![7],
                },
                config_files: Vec::new(),
                generate_config: None
            })),
            config_files: Vec::new(),
            generate_config: None
        },
        ConfigOptMyStruct::from_iter_safe(cli).unwrap()
    );
}

#[test]
fn test_configopt_from_empty_file() {
    use tempfile::NamedTempFile;
    let config_file = NamedTempFile::new().unwrap();
    let mut cli = Vec::new();
    cli.push("app");
    let config_arg = format!("--config-files={}", config_file.path().to_string_lossy());
    cli.push(&config_arg);
    cli.push("cmd3");
    let config_arg = format!("--config-files={}", config_file.path().to_string_lossy());
    cli.push(&config_arg);

    let mut s = ConfigOptMyStruct::from_iter_safe(cli).unwrap();
    assert_eq!(1, s.config_files.len());
    match s.cmd.as_ref().unwrap() {
        ConfigOptMyEnum::Cmd3(s) => {
            assert_eq!(1, s.config_files.len());
        }
        _ => unreachable!(),
    }
    s.patch_with_config_files().unwrap();
}

#[test]
fn test_configopt_from_file_and_defaults() {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let config_contents_1 = r###"
        maybe = false
        numbers = [1, 2, 3]
        optional = "from_config1"
        not_optional = "from_config2"
        double_optional = 5.1
        optional_vec = [4, 5]
        path = "/this/is/a/path"
"###;
    let config_contents_2 = r###"
        field_a = "from_config3"
        field_b = "from_config4"
        flat_optional = 9
        flat_maybe = true
        flat_numbers = [8, 9, 10]
"###;
    let mut config_file_1 = NamedTempFile::new().unwrap();
    write!(config_file_1, "{}", config_contents_1).unwrap();
    let mut config_file_2 = NamedTempFile::new().unwrap();
    write!(config_file_2, "{}", config_contents_2).unwrap();

    let mut cli = Vec::new();
    cli.push("app");
    cli.push("--notOptional=from_cli_again2");
    let config_arg_1 = format!("--config-files={}", config_file_1.path().to_string_lossy());
    cli.push(&config_arg_1);
    cli.push("cmd3");
    cli.push("--field-a=from_cli_again3");
    let config_arg_2 = format!("--config-files={}", config_file_2.path().to_string_lossy());
    cli.push(&config_arg_2);

    let mut s = ConfigOptMyStruct::from_iter_safe(cli).unwrap();
    s.patch_with_config_files().unwrap();

    assert_eq!(
        Some(OsString::from("false")),
        s.arg_to_os_string(&[String::from("maybe")])
    );
    // TODO (DM): Defaults for Vecs was removed. How exactly should it work?
    // assert_eq!(
    //     Some(OsString::from("1 2 3")),
    //     s.arg_to_os_string(&[String::from("numbers")])
    // );
    assert_eq!(
        Some(OsString::from("from_config1")),
        s.arg_to_os_string(&[String::from("optional")])
    );
    assert_eq!(
        Some(OsString::from("from_cli_again2")),
        s.arg_to_os_string(&[String::from("notOptional")])
    );
    assert_eq!(
        Some(OsString::from("5.1")),
        s.arg_to_os_string(&[String::from("doubleOptional")])
    );
    // TODO (DM): Defaults for Vecs was removed. How exactly should it work?
    // assert_eq!(
    //     Some(OsString::from("4 5")),
    //     s.arg_to_os_string(&[String::from("optionalVec")])
    // );
    assert_eq!(
        Some(OsString::from("/this/is/a/path")),
        s.arg_to_os_string(&[String::from("path")])
    );

    assert_eq!(
        Some(OsString::from("from_cli_again3")),
        s.arg_to_os_string(&[String::from("cmd3"), String::from("field-a")])
    );
    assert_eq!(
        Some(OsString::from("from_config4")),
        s.arg_to_os_string(&[String::from("cmd3"), String::from("field-b")])
    );
    assert_eq!(
        Some(OsString::from("9")),
        s.arg_to_os_string(&[String::from("cmd3"), String::from("flat-optional")])
    );
    assert_eq!(
        Some(OsString::from("true")),
        s.arg_to_os_string(&[String::from("cmd3"), String::from("flat-maybe")])
    );
    // TODO (DM): Defaults for Vecs was removed. How exactly should it work?
    // assert_eq!(
    //     Some(OsString::from("8 9 10")),
    //     s.arg_to_os_string(&[String::from("cmd3"), String::from("flat-numbers")])
    // );
}

#[test]
fn test_take_for() {
    use configopt::ConfigOpt;

    let mut c = ConfigOptMyStruct {
        maybe: None,
        numbers: Vec::new(),
        optional: Some(String::from("configopt_optional")),
        not_optional: Some(String::from("configopt_not_optional")),
        double_optional: None,
        optional_vec: None,
        path: Some(PathBuf::from("/some/path")),
        cmd: None,
        config_files: Vec::new(),
        generate_config: None,
    };
    let mut s =
        MyStruct::try_from_iter_with_defaults(&["app", "cmd3", "--field-a=from_cli"], &c).unwrap();
    assert!(s.optional.is_none());
    c.take_for(&mut s);
    assert_eq!(Some(String::from("configopt_optional")), s.optional);
}

#[test]
fn test_patch_for() {
    use configopt::ConfigOpt;

    let mut c = ConfigOptMyStruct {
        maybe: None,
        numbers: Vec::new(),
        optional: Some(String::from("optional_from_configopt")),
        not_optional: Some(String::from("not_optional_from_configopt")),
        double_optional: None,
        optional_vec: None,
        path: Some(PathBuf::from("/some/path")),
        cmd: None,
        config_files: Vec::new(),
        generate_config: None,
    };

    let mut s = MyStruct::try_from_iter_with_defaults(
        &[
            "app",
            "--optional=optional_from_cli",
            "--notOptional=not_optional_from_cli",
            "cmd3",
            "--field-a=from_cli",
        ],
        &c,
    )
    .unwrap();
    assert!(s.optional.is_some());
    c.patch_for(&mut s);
    assert_eq!(Some(String::from("optional_from_cli")), s.optional);

    let mut s = MyStruct::try_from_iter_with_defaults(
        &[
            "app",
            "--notOptional=not_optional_from_cli",
            "cmd3",
            "--field-a=from_cli",
        ],
        &c,
    )
    .unwrap();
    assert!(s.optional.is_none());
    c.patch_for(&mut s);
    assert_eq!(Some(String::from("optional_from_configopt")), s.optional);
}

#[test]
fn test_from() {
    let m = MyStruct {
        maybe: true,
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: String::from("testing123"),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: PathBuf::from("/test/path"),
        cmd: MyEnum::Cmd1,
        config_files: Vec::new(),
        generate_config: false,
    };
    let c1 = ConfigOptMyStruct::from(m);
    let c2 = ConfigOptMyStruct {
        maybe: Some(true),
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: Some(String::from("testing123")),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: Some(PathBuf::from("/test/path")),
        cmd: Some(ConfigOptMyEnum::Cmd1),
        config_files: Vec::new(),
        generate_config: Some(false),
    };
    assert_eq!(c1, c2);

    let f = FlatStruct {
        flat_optional: None,
        flat_maybe: true,
        flat_numbers: vec![4, 5, 6],
    };
    let a = AnotherStruct {
        field_a: String::from("another_struct_test"),
        field_b: None,
        flat_struct: f,
        config_files: Vec::new(),
        generate_config: false,
    };
    let f = MyStruct {
        maybe: true,
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: String::from("testing123"),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: PathBuf::from("/test/path"),
        cmd: MyEnum::Cmd3(a),
        config_files: Vec::new(),
        generate_config: false,
    };
    let c1 = ConfigOptMyStruct::from(f);
    let f = ConfigOptFlatStruct {
        flat_optional: None,
        flat_maybe: Some(true),
        flat_numbers: vec![4, 5, 6],
    };
    let a = ConfigOptAnotherStruct {
        field_a: Some(String::from("another_struct_test")),
        field_b: None,
        flat_struct: f,
        config_files: Vec::new(),
        generate_config: Some(false),
    };
    let c2 = ConfigOptMyStruct {
        maybe: Some(true),
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: Some(String::from("testing123")),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: Some(PathBuf::from("/test/path")),
        cmd: Some(ConfigOptMyEnum::Cmd3(a)),
        config_files: Vec::new(),
        generate_config: Some(false),
    };
    assert_eq!(c1, c2);
}

#[test]
fn test_try_from() {
    use std::convert::TryFrom;

    let mut c = ConfigOptMyStruct {
        maybe: Some(true),
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: None,
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: Some(PathBuf::from("/test/path")),
        cmd: Some(ConfigOptMyEnum::Cmd1),
        config_files: Vec::new(),
        generate_config: None,
    };
    assert!(!c.is_convertible());
    c.not_optional = Some(String::from("testing123"));
    c.cmd = None;
    assert!(!c.is_convertible());
    c.cmd = Some(ConfigOptMyEnum::Cmd1);
    assert!(c.is_convertible());
    let m1 = MyStruct::try_from(c).unwrap();
    let m2 = MyStruct {
        maybe: true,
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: String::from("testing123"),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: PathBuf::from("/test/path"),
        cmd: MyEnum::Cmd1,
        config_files: Vec::new(),
        generate_config: false,
    };
    assert_eq!(m1, m2);

    let f = ConfigOptFlatStruct {
        flat_optional: None,
        flat_maybe: None,
        flat_numbers: vec![4, 5, 6],
    };
    let a = ConfigOptAnotherStruct {
        field_a: None,
        field_b: None,
        flat_struct: f,
        config_files: Vec::new(),
        generate_config: Some(false),
    };
    let mut c = ConfigOptMyStruct {
        maybe: Some(true),
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: Some(String::from("testing123")),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: Some(PathBuf::from("/test/path")),
        cmd: Some(ConfigOptMyEnum::Cmd3(a)),
        config_files: Vec::new(),
        generate_config: None,
    };
    assert!(!c.is_convertible());
    let f = ConfigOptFlatStruct {
        flat_optional: None,
        flat_maybe: None,
        flat_numbers: vec![4, 5, 6],
    };
    let a = ConfigOptAnotherStruct {
        field_a: Some(String::from("another_struct_test")),
        field_b: None,
        flat_struct: f,
        config_files: Vec::new(),
        generate_config: Some(false),
    };
    c.cmd = Some(ConfigOptMyEnum::Cmd3(a));
    assert!(c.is_convertible());
    let m1 = MyStruct::try_from(c).unwrap();
    let f = FlatStruct {
        flat_optional: None,
        flat_maybe: false,
        flat_numbers: vec![4, 5, 6],
    };
    let a = AnotherStruct {
        field_a: String::from("another_struct_test"),
        field_b: None,
        flat_struct: f,
        config_files: Vec::new(),
        generate_config: false,
    };
    let m2 = MyStruct {
        maybe: true,
        numbers: vec![1, 2, 3],
        optional: None,
        not_optional: String::from("testing123"),
        double_optional: Some(Some(5.6)),
        optional_vec: None,
        path: PathBuf::from("/test/path"),
        cmd: MyEnum::Cmd3(a),
        config_files: Vec::new(),
        generate_config: false,
    };
    assert_eq!(m1, m2);
}
