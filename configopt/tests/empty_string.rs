use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct MyStruct {
    #[structopt(long, empty_values = false)]
    custom: Option<String>,
}

#[test]
fn test_empty_argument() {
    assert!(MyStruct::from_iter_safe(&["app", "--custom"]).is_err());
    assert!(MyStruct::from_iter_safe(&["app", "--custom", ""]).is_err());
}
