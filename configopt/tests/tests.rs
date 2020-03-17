use configopt::{ConfigOpt, ConfigOptDefaults};
use serde::Deserialize;
use serde_json;
use std::{convert::TryFrom, path::PathBuf};
use structopt::StructOpt;

const DEFAULT_VALUE: &str = "5";

#[test]
fn test_simple_configopt_defaults() {
    #[derive(ConfigOpt, StructOpt)]
    #[configopt(derive(Default))]
    #[structopt(rename_all = "pascal")]
    struct MyStruct {
        #[structopt(name = "a_new_name", default_value = DEFAULT_VALUE)]
        this_is_an_arg: u64,
        another_arg: String,
        list_arg: Vec<String>,
        optional_arg: Option<String>,
    }

    let mut partial = PartialMyStruct::default();
    partial.this_is_an_arg = Some(12);
    partial.another_arg = Some(String::from("test"));
    assert_eq!(
        partial.arg_default(&[String::from("a_new_name")]).unwrap(),
        "12"
    );
    assert_eq!(
        partial.arg_default(&[String::from("AnotherArg")]).unwrap(),
        "test"
    );
    assert!(partial
        .arg_default(&[String::from("another_arg")])
        .is_none(),);
}

#[test]
fn test_nested_configopt_defaults() {
    #[derive(ConfigOpt, StructOpt, Debug)]
    #[configopt(derive(Default, Debug))]
    #[structopt(rename_all = "screamingsnake")]
    struct NestedStruct {
        #[structopt(default_value = DEFAULT_VALUE)]
        field1: u64,
        #[configopt(to_default = configopt::path_buf_to_default)]
        field2: PathBuf,
        #[configopt(to_default = configopt::path_buf_to_default)]
        field3: Option<PathBuf>,
        #[configopt(to_default = configopt::path_buf_to_default)]
        field4: Vec<PathBuf>,
    }

    #[derive(ConfigOpt, StructOpt)]
    #[configopt(derive(Default))]
    #[structopt(rename_all = "pascal")]
    struct MyStruct {
        #[structopt(name = "a_new_name", default_value = DEFAULT_VALUE)]
        this_is_an_arg: u64,
        another_arg: String,
        #[configopt(nested)]
        #[structopt(flatten)]
        nested: NestedStruct,
    }

    let mut partial = PartialMyStruct::default();
    partial.this_is_an_arg = Some(12);
    partial.another_arg = Some(String::from("test"));
    partial.nested = PartialNestedStruct {
        field1: Some(4),
        field2: None,
        field3: None,
        field4: Some(vec![
            PathBuf::from("/test/path1"),
            PathBuf::from("/test/path2"),
        ]),
    };
    assert_eq!(
        partial.arg_default(&[String::from("a_new_name")]).unwrap(),
        "12"
    );
    assert_eq!(
        partial.arg_default(&[String::from("AnotherArg")]).unwrap(),
        "test"
    );
    assert_eq!(
        partial
            .arg_default(&[String::from("Nested"), String::from("FIELD_1")])
            .unwrap(),
        "4"
    );
    assert_eq!(
        partial
            .arg_default(&[String::from("Nested"), String::from("FIELD_4")])
            .unwrap(),
        "/test/path1,/test/path2"
    );
    assert!(partial
        .arg_default(&[String::from("another_arg")])
        .is_none(),);
}

#[test]
fn test_simple() {
    #[derive(ConfigOpt, Debug, PartialEq)]
    #[configopt(partial_only, derive(Default, PartialEq, Debug))]
    struct MyStruct {
        s: String,
        u: u64,
    };

    let mut partial = PartialMyStruct::default();
    assert_eq!(partial, PartialMyStruct { s: None, u: None });
    assert!(partial.is_empty());
    assert!(!partial.is_complete());

    partial.s = Some(String::from("test"));
    let mut partial2 = PartialMyStruct::default();
    partial2.s = Some(String::from("another"));
    partial2.u = Some(16);
    partial.patch(&mut partial2);
    assert!(!partial2.is_empty());
    assert!(!partial.is_empty());
    assert!(partial.is_complete());
    assert_eq!(partial.s.as_ref().unwrap(), "test");

    let mut partial2 = PartialMyStruct::default();
    partial2.s = Some(String::from("test2"));
    partial2.u = Some(162);
    partial.take(&mut partial2);
    assert!(partial2.is_empty());

    let partial2 = PartialMyStruct::from(MyStruct {
        s: String::from("test2"),
        u: 162,
    });
    assert_eq!(partial2, partial);

    let full = MyStruct::try_from(partial).expect("to convert");
    assert_eq!(
        full,
        MyStruct {
            s: String::from("test2"),
            u: 162,
        }
    );
}

#[test]
fn test_nested() {
    #[derive(ConfigOpt, Debug, PartialEq, Default)]
    #[configopt(partial_only, derive(Default, PartialEq, Debug))]
    struct AStruct {
        a: String,
        b: u64,
    };

    #[derive(ConfigOpt, Debug, PartialEq, Default)]
    #[configopt(partial_only, derive(Default, PartialEq, Debug))]
    struct AnotherStruct {
        #[configopt(nested)]
        s: AStruct,
    };

    #[derive(Debug, PartialEq, Default)]
    struct NotPartialStruct {
        u: u64,
    };

    #[derive(ConfigOpt, Debug, PartialEq, Default)]
    #[configopt(partial_only, derive(Default, PartialEq, Debug))]
    struct YetAnotherStruct {
        not_partial: NotPartialStruct,
        #[configopt(nested)]
        another: AnotherStruct,
    };

    #[derive(ConfigOpt, Debug, PartialEq, Default)]
    #[configopt(partial_only, derive(Default, PartialEq, Debug))]
    struct MyStruct {
        field: String,
        #[configopt(nested)]
        another: AnotherStruct,
        #[configopt(nested)]
        yet_another: YetAnotherStruct,
    };

    let partial = PartialMyStruct::default();
    let full = MyStruct::try_from(partial);
    assert!(full.is_err());
    let mut partial = full.unwrap_err();
    let mut partial2 = PartialMyStruct::default();
    partial2.another = PartialAnotherStruct {
        s: PartialAStruct {
            a: Some(String::from("test")),
            b: None,
        },
    };
    partial.take(&mut partial2);
    assert_eq!(partial.another.s.a.as_ref().unwrap(), "test");
    assert!(partial2.is_empty());

    let mut partial2 = PartialMyStruct::default();
    partial2.another = PartialAnotherStruct {
        s: PartialAStruct {
            a: Some(String::from("test2")),
            b: None,
        },
    };
    partial.take(&mut partial2);
    assert_eq!(partial.another.s.a.as_ref().unwrap(), "test2");
    assert!(partial2.is_empty());

    let mut partial3 = PartialMyStruct::default();
    partial3.another = PartialAnotherStruct {
        s: PartialAStruct {
            a: Some(String::from("test3")),
            b: Some(15),
        },
    };
    partial.patch(&mut partial3);
    assert_eq!(partial.another.s.a.as_ref().unwrap(), "test2");
    assert_eq!(*partial.another.s.b.as_ref().unwrap(), 15);

    let mut full = MyStruct::default();
    partial.merge(&mut full);
    assert_eq!(full.another.s.a, "test2");
    assert_eq!(full.another.s.b, 15);
}

#[test]
fn test_serde_structopt() {
    #[derive(ConfigOpt, Debug, PartialEq, StructOpt, Deserialize)]
    #[configopt(derive(Default, PartialEq, Debug, Deserialize), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct MyStruct {
        /// This is arg1
        #[structopt(long = "arg1", default_value = "arg1")]
        #[serde(rename = "arg1")]
        s: String,
        /// This is arg2
        #[structopt(long = "arg2", default_value = "2")]
        #[serde(rename = "arg2")]
        u: u64,
    };

    let data = r#"
    {
        "arg1": "Test",
        "arg2": 42
    }"#;
    let partial: PartialMyStruct = serde_json::from_str(data).expect("to parse");
    let full = MyStruct::try_from(partial).expect("to convert");
    assert_eq!(
        MyStruct {
            s: String::from("Test"),
            u: 42
        },
        full
    );

    let data = r#"
    {
        "arg1": "Test"
    }"#;
    let partial: PartialMyStruct = serde_json::from_str(data).expect("to parse");
    assert_eq!(
        PartialMyStruct {
            s: Some(String::from("Test")),
            u: None
        },
        partial
    );

    let data = r#"
    {
        "unknown": "field",
        "arg1": "Test"
    }"#;
    assert!(serde_json::from_str::<PartialMyStruct>(data).is_err());
}
