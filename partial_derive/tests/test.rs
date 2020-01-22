use configopt;
use partial_derive::Partial;
use structopt::StructOpt;
use std::{net::SocketAddr, path::PathBuf};
use serde::{Serialize, Deserialize};

#[test]
fn this_is_a_test() {
    // #[derive(Partial)]
    // #[partial(derive(Debug, Default))]
    // struct Unnamed(&'static str, Option<()>);

    // /// A comment
    // #[derive(Default, Partial)]
    // #[partial(derive(Debug, Default))]
    // struct Partially {
    //     f: f64,
    // };

    #[derive(Debug, Partial, StructOpt)]
    #[partial(derive(Debug, Default), attrs(serde))]
    struct MyStruct {
        // #[partial]
        // u: Unnamed,
        // #[partial]
        // p: Partially,
        #[structopt(name = "X_NAME", long = "test", default_value = "127.0.0.1:1234")]
        x: SocketAddr,
        // y: String,
        // z: Option<String>,
        // v: Vec<String>,
        // p: PathBuf,
    };
    let mut partial = PartialMyStruct::default();
    partial.x = Some("5.6.7.8:1111".parse().unwrap());
    // let mut u = PartialUnnamed::default();
    // u.0 = Some("test");
    // partial.u = Some(u);
    // partial.p = Some(PartialPartially::default());
    println!("{:#?}", partial);
    let s: MyStruct = configopt::from_args_with_defaults(&partial);
    println!("{:#?}", s);
}
