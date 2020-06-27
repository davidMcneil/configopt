use configopt::ConfigOpt;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug))]
struct MyStruct {
    #[structopt(long)]
    value: String,
    #[structopt(subcommand)]
    command: MyEnum,
}

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug))]
enum MyEnum {
    Cmd1,
    Cmd2,
}

#[test]
fn test_minimal_requirements() {
    let mut m = ConfigOptMyStruct::default();
    assert!(!m.is_convertible());
    m.value = Some(String::from("test"));
    assert!(!m.is_convertible());
    m.command = Some(ConfigOptMyEnum::Cmd1);
    assert!(m.is_convertible());
    let f = MyStruct::try_from(m).ok().unwrap();
    assert_eq!(f.value, "test");
}
