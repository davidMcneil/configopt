use partial_derive::Partial;
use serde::Deserialize;
use serde_json;
use std::convert::TryFrom;
use structopt::StructOpt;

#[test]
fn test_simple() {
    #[derive(Partial, Debug, PartialEq)]
    #[partial(derive(Default, PartialEq, Debug))]
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
    #[derive(Partial, Debug, PartialEq, Default)]
    #[partial(derive(Default, PartialEq, Debug))]
    struct UnnamedStruct(String, u64);

    #[derive(Partial, Debug, PartialEq, Default)]
    #[partial(derive(Default, PartialEq, Debug))]
    struct AnotherStruct {
        #[partial]
        s: UnnamedStruct,
    };

    #[derive(Debug, PartialEq, Default)]
    struct NotPartialStruct {
        u: u64,
    };

    #[derive(Partial, Debug, PartialEq, Default)]
    #[partial(derive(Default, PartialEq, Debug))]
    struct YetAnotherStruct {
        not_partial: NotPartialStruct,
        #[partial]
        another: AnotherStruct,
    };

    #[derive(Partial, Debug, PartialEq, Default)]
    #[partial(derive(Default, PartialEq, Debug))]
    struct MyStruct {
        field: String,
        #[partial]
        another: AnotherStruct,
        #[partial]
        yet_another: YetAnotherStruct,
    };

    let partial = PartialMyStruct::default();
    let full = MyStruct::try_from(partial);
    assert!(full.is_err());
    let mut partial = full.unwrap_err();
    let mut partial2 = PartialMyStruct::default();
    partial2.another = Some(PartialAnotherStruct {
        s: Some(PartialUnnamedStruct(Some(String::from("test")), None)),
    });
    partial.take(&mut partial2);
    assert_eq!(
        partial
            .another
            .as_ref()
            .unwrap()
            .s
            .as_ref()
            .unwrap()
            .0
            .as_ref()
            .unwrap(),
        "test"
    );
    assert!(partial2.is_empty());

    let mut partial2 = PartialMyStruct::default();
    partial2.another = Some(PartialAnotherStruct {
        s: Some(PartialUnnamedStruct(Some(String::from("test2")), None)),
    });
    partial.take(&mut partial2);
    assert_eq!(
        partial
            .another
            .as_ref()
            .unwrap()
            .s
            .as_ref()
            .unwrap()
            .0
            .as_ref()
            .unwrap(),
        "test2"
    );
    assert!(partial2.is_empty());

    let mut partial3 = PartialMyStruct::default();
    partial3.another = Some(PartialAnotherStruct {
        s: Some(PartialUnnamedStruct(Some(String::from("test3")), Some(15))),
    });
    partial.patch(&mut partial3);
    assert_eq!(
        partial
            .another
            .as_ref()
            .unwrap()
            .s
            .as_ref()
            .unwrap()
            .0
            .as_ref()
            .unwrap(),
        "test2"
    );
    assert_eq!(
        *partial
            .another
            .as_ref()
            .unwrap()
            .s
            .as_ref()
            .unwrap()
            .1
            .as_ref()
            .unwrap(),
        15
    );

    let mut full = MyStruct::default();
    partial.merge(&mut full);
    assert_eq!(full.another.s.0, "test2");
    assert_eq!(full.another.s.1, 15);
}

#[test]
fn test_serde_structopt() {
    #[derive(Partial, Debug, PartialEq, StructOpt, Deserialize)]
    #[partial(derive(Default, PartialEq, Debug, Deserialize), attrs(serde))]
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
