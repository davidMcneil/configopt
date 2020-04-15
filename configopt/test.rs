#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use configopt::{configopt_fields, ConfigOpt};
use serde::Deserialize;
use structopt::StructOpt;
struct MyStruct {
    #[structopt(long, default_value = "1")]
    first: Option<i32>,
    #[structopt(subcommand)]
    cmd: MyEnum,
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOpt for MyStruct {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt");
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        MyStruct {
            first: if matches.occurrences_of("first") > 0 || matches.env_set("first") {
                matches
                    .value_of("first")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
            } else {
                None
            },
            cmd: <MyEnum as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand())
                .unwrap(),
        }
    }
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOptInternal for MyStruct {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        {
            let app = app;
            let app = app.arg(
                ::structopt::clap::Arg::with_name("first")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: i32| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("first")
                    .default_value("1"),
            );
            let app = <MyEnum as ::structopt::StructOptInternal>::augment_clap(app);
            let app = app.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
            app.version("0.1.0")
        }
    }
    fn is_subcommand() -> bool {
        false
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MyStruct {
                first: ref __self_0_0,
                cmd: ref __self_0_1,
            } => {
                let mut debug_trait_builder = f.debug_struct("MyStruct");
                let _ = debug_trait_builder.field("first", &&(*__self_0_0));
                let _ = debug_trait_builder.field("cmd", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
enum MyEnum {
    Cmd1,
    Cmd3(AnotherStruct),
}
#[allow(unknown_lints)]
#[allow(unused_variables, dead_code, unreachable_code)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
impl ::structopt::StructOpt for MyEnum {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt")
            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        <MyEnum as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand()).unwrap()
    }
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOptInternal for MyEnum {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        let app = app;
        let app = app.subcommand({
            let subcommand = ::structopt::clap::SubCommand::with_name("cmd1");
            let subcommand = subcommand;
            subcommand.version("0.1.0")
        });
        let app = app.subcommand({
            let subcommand = ::structopt::clap::SubCommand::with_name("cmd3");
            let subcommand = {
                let subcommand =
                    <AnotherStruct as ::structopt::StructOptInternal>::augment_clap(subcommand);
                if <AnotherStruct as ::structopt::StructOptInternal>::is_subcommand() {
                    subcommand.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                } else {
                    subcommand
                }
            };
            subcommand.version("0.1.0")
        });
        app.version("0.1.0")
    }
    fn from_subcommand<'a, 'b>(
        sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>),
    ) -> Option<Self> {
        match sub {
            ("cmd1", Some(matches)) => Some(MyEnum::Cmd1),
            ("cmd3", Some(matches)) => Some(MyEnum::Cmd3(
                <AnotherStruct as ::structopt::StructOpt>::from_clap(matches),
            )),
            other => {
                None
            }
        }
    }
    fn is_subcommand() -> bool {
        true
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&MyEnum::Cmd1,) => {
                let mut debug_trait_builder = f.debug_tuple("Cmd1");
                debug_trait_builder.finish()
            }
            (&MyEnum::Cmd3(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Cmd3");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
struct AnotherStruct {
    #[structopt(long, default_value = "2")]
    second: Option<i32>,
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOpt for AnotherStruct {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt");
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        AnotherStruct {
            second: if matches.occurrences_of("second") > 0 || matches.env_set("second") {
                matches
                    .value_of("second")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
            } else {
                None
            },
        }
    }
}
#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo
)]
#[deny(clippy::correctness)]
#[allow(dead_code, unreachable_code)]
impl ::structopt::StructOptInternal for AnotherStruct {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        {
            let app = app;
            let app = app.arg(
                ::structopt::clap::Arg::with_name("second")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: i32| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("second")
                    .default_value("2"),
            );
            app.version("0.1.0")
        }
    }
    fn is_subcommand() -> bool {
        false
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for AnotherStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            AnotherStruct {
                second: ref __self_0_0,
            } => {
                let mut debug_trait_builder = f.debug_struct("AnotherStruct");
                let _ = debug_trait_builder.field("second", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
fn main() {
    let app = MyStruct::clap();
    let m = app.get_matches_from_safe(&["", "cmd3"]).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&m.value_of("first"),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    if let Some(m) = m.subcommand_matches("cmd3") {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&m.value_of("second"),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                },
            ));
        };
    }
}
