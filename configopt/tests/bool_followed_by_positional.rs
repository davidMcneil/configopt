use configopt::ConfigOpt;
use serde::Serialize;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug, PartialEq)]
#[configopt(derive(Debug, Serialize, PartialEq))]
struct MyStruct {
    #[structopt(long)]
    flag: bool,
    #[structopt()]
    positional: String,
}

#[test]
fn bool_followed_by_positional() {
    let s = ConfigOptMyStruct::from_iter_safe(&["app", "test"]).unwrap();
    assert_eq!(
        ConfigOptMyStruct {
            flag: None.into(),
            positional: Some(String::from("test"))
        },
        s
    );
    let s = ConfigOptMyStruct::from_iter_safe(&["app", "--flag", "test"]).unwrap();
    assert_eq!(
        ConfigOptMyStruct {
            flag: true.into(),
            positional: Some(String::from("test"))
        },
        s
    );
    assert!(ConfigOptMyStruct::from_iter_safe(&["app", "--flag", "bad", "test"]).is_err());

    let s = MyStruct::from_iter_safe(&["app", "test"]).unwrap();
    assert_eq!(
        MyStruct {
            flag: false,
            positional: String::from("test")
        },
        s
    );
    let s = MyStruct::from_iter_safe(&["app", "--flag", "test"]).unwrap();
    assert_eq!(
        MyStruct {
            flag: true,
            positional: String::from("test")
        },
        s
    );
    assert!(MyStruct::from_iter_safe(&["app", "--flag", "bad", "test"]).is_err());
}
