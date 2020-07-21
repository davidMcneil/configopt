use configopt::ConfigOpt;
use serde::Serialize;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug, Serialize))]
struct MyStruct {
    #[structopt(long)]
    vec: Vec<u32>,
}

#[test]
fn vec_default_value() {
    assert_eq!(
        vec![1, 2, 3],
        MyStruct::from_iter_safe(&["app", "--vec", "1", "2", "3"])
            .unwrap()
            .vec
    );
    // TODO: This currently does not work. How should `Vec` default values work?
    // let c = ConfigOptMyStruct {
    //     vec: Some(vec![1, 2, 3]),
    // };
    // assert_eq!(
    //     vec![1, 2, 3],
    //     MyStruct::try_from_iter_with_defaults(&["app"], &c)
    //         .unwrap()
    //         .vec
    // );
}
