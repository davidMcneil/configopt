use configopt::{ConfigOptDefaults, Partial};
use structopt::StructOpt;

const DEFAULT_VALUE: &str = "5";

#[test]
fn test_simple() {
    #[derive(ConfigOptDefaults, Partial, StructOpt)]
    #[configopt_defaults(type = "PartialMyStruct")]
    #[partial(derive(Default))]
    #[structopt(rename_all = "pascal")]
    struct MyStruct {
        #[structopt(name = "a_new_name", default_value = DEFAULT_VALUE)]
        this_is_an_arg: u64,
        another_arg: String,
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
