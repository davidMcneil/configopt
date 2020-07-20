use configopt::{configopt_fields, ConfigOpt, ConfigOptBool};
use serde::Deserialize;
use structopt::StructOpt;

#[test]
fn test_configopt_with_optional_bool() {
    #[configopt_fields]
    #[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
    #[configopt(derive(Debug, PartialEq), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct MyStruct {
        #[structopt(long, default_value = "true")]
        maybe: Option<bool>,
    }

    let c = ConfigOptMyStruct::from_iter_safe(&["app"]).unwrap();
    assert_eq!(None, c.maybe);
    let c = ConfigOptMyStruct::from_iter_safe(&["app", "--maybe"]).unwrap();
    assert_eq!(Some(true), c.maybe);
    let c = ConfigOptMyStruct::from_iter_safe(&["app", "--maybe=false"]).unwrap();
    assert_eq!(Some(false), c.maybe);
    let c = ConfigOptMyStruct::from_iter_safe(&["app", "--maybe=true"]).unwrap();
    assert_eq!(Some(true), c.maybe);

    let c = ConfigOptMyStruct {
        maybe: Some(true),
        config_files: None,
        generate_config: None.into(),
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    assert_eq!(None, s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe"], &c).unwrap();
    assert_eq!(Some(true), s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe=true"], &c).unwrap();
    assert_eq!(Some(true), s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe=false"], &c).unwrap();
    assert_eq!(Some(false), s.maybe);

    let c = ConfigOptMyStruct {
        maybe: Some(false),
        config_files: None,
        generate_config: None.into(),
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    assert_eq!(None, s.maybe);
    // TODO: This is kinda odd. Because we set the default to `false`, passing the flag does not
    // make the value `true`. We could probably use a `from_occurrences` parser to get around this.
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe"], &c).unwrap();
    assert_eq!(Some(false), s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe=true"], &c).unwrap();
    assert_eq!(Some(true), s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe=false"], &c).unwrap();
    assert_eq!(Some(false), s.maybe);
}

#[test]
fn test_configopt_with_bool() {
    #[configopt_fields]
    #[derive(ConfigOpt, StructOpt, Debug, Deserialize, PartialEq)]
    #[configopt(derive(Debug, PartialEq), attrs(serde))]
    #[serde(deny_unknown_fields)]
    struct MyStruct {
        #[structopt(long)]
        maybe: bool,
    }

    let c = ConfigOptMyStruct::from_iter_safe(&["app"]).unwrap();
    assert_eq!(ConfigOptBool::from(None), c.maybe);
    let c = ConfigOptMyStruct::from_iter_safe(&["app", "--maybe"]).unwrap();
    assert_eq!(ConfigOptBool::from(Some(true)), c.maybe);

    let c = ConfigOptMyStruct {
        maybe: Some(true).into(),
        config_files: None,
        generate_config: None.into(),
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    // We want this to be true, but setting a default value for a boolean is impossible.
    assert_eq!(false, s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe"], &c).unwrap();
    assert_eq!(true, s.maybe);

    let c = ConfigOptMyStruct {
        maybe: Some(false).into(),
        config_files: None,
        generate_config: None.into(),
    };
    let s = MyStruct::try_from_iter_with_defaults(&["app"], &c).unwrap();
    assert_eq!(false, s.maybe);
    let s = MyStruct::try_from_iter_with_defaults(&["app", "--maybe"], &c).unwrap();
    assert_eq!(true, s.maybe);
}
