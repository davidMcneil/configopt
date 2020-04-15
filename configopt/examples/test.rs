use configopt::{configopt_fields, ConfigOpt};
use serde::Deserialize;
use structopt::clap::{App, Arg, SubCommand};
use structopt::StructOpt;

// #[configopt_fields]
// #[derive(ConfigOpt, StructOpt, Debug, Deserialize)]
// #[configopt(derive(Debug), attrs(serde))]
// #[serde(deny_unknown_fields)]
// struct MyStruct {
//     #[structopt(long)]
//     maybe: bool,
// }

// #[derive(StructOpt, Debug)]
// struct MyStruct {
//     #[structopt(long, default_value = "1", takes_value = false, multiple = false, require_delimiter = true, value_delimiter = "\0")]
//     first: Option<i32>,
//     #[structopt(subcommand)]
//     cmd: MyEnum,
// }

// #[derive(StructOpt, Debug)]
// enum MyEnum {
//     Cmd1,
//     Cmd3(AnotherStruct),
// }

// #[derive(StructOpt, Debug)]
// struct AnotherStruct {
//     #[structopt(long, default_value = "2", takes_value = false, multiple = false, require_delimiter = true, value_delimiter = "\0")]
//     second: Option<i32>,
// }

fn main() {
    // let app = MyStruct::clap();
    // let m = app.get_matches_from_safe(&["", "--first", "cmd3"]).unwrap();
    // println!("{:?}", m.value_of("first"));
    // if let Some(m) = m.subcommand_matches("cmd3") {
    //     println!("{:?}", m.value_of("second"));
    // }
    // println!("{:?}", MyStruct::from_args());

    let app = App::new("app")
        .arg(
            Arg::with_name("arg")
                .takes_value(true)
                .multiple(false)
                .long("arg"),
        )
        .subcommand(SubCommand::with_name("sub"));

    let matches = app
        .clone()
        .get_matches_from_safe(&["app", "--arg", "sub"])
        .unwrap();
    println!("{:?}", matches);
    println!("value_of       {:?}", matches.value_of("arg"));
    println!("values_of      {:?}", matches.values_of("arg"));
    println!("is_present     {:?}", matches.is_present("arg"));
    println!("occurrences_of {:?}", matches.occurrences_of("arg"));
    println!("index_of       {:?}", matches.index_of("arg"));
    println!("indices_of     {:?}", matches.indices_of("arg"));
    println!("env_set        {:?}", matches.env_set("arg"));
}
