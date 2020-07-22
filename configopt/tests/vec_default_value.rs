use configopt::ConfigOpt;
use serde::Serialize;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Debug, Serialize))]
struct MyStruct {
    #[structopt(long)]
    vec: Vec<u32>,
    #[structopt(long)]
    opt_vec: Option<Vec<u32>>,
}

#[test]
fn vec_default_value() {
    let s = MyStruct::try_from_iter_with_configopt(&["app", "--vec", "1", "2", "3"]).unwrap();
    assert_eq!(vec![1, 2, 3], s.vec);
    assert_eq!(None, s.opt_vec);
    let s = MyStruct::try_from_iter_with_configopt(&["app", "--vec", "1", "2", "3", "--opt-vec"])
        .unwrap();
    assert_eq!(vec![1, 2, 3], s.vec);
    assert_eq!(Some(vec![]), s.opt_vec);
    let s = MyStruct::try_from_iter_with_configopt(&[
        "app",
        "--vec",
        "1",
        "2",
        "3",
        "--opt-vec",
        "4",
        "5",
    ])
    .unwrap();
    assert_eq!(vec![1, 2, 3], s.vec);
    assert_eq!(Some(vec![4, 5]), s.opt_vec);

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
