use clap::{App, ArgMatches, ArgSettings, Clap, FromArgMatches, IntoApp};
use parse_with_defaults_derive::ParseWithDefaults;
use serde::{Deserialize, Serialize};
use std::env;

#[test]
fn parse_with_defaults() {
    #[derive(Clap, ParseWithDefaults)]
    struct MyStruct {
        x: u8,
        y: String,
    };
    let mut s = MyStruct {
        x: 5,
        y: String::from("Test"),
    };
    let mut o = PartialMyStruct::default();
    assert_eq!(o.x, None);
    assert_eq!(o.y, None);

    o.x = Some(9);
    o.merge(&mut s);
    let o = PartialMyStruct::from(s);
    assert_eq!(o.x, Some(9));
    assert_eq!(o.y, Some(String::from("Test")));

    let s = StringyMyStruct::from(&o);
    assert_eq!(s.x, Some(String::from("9")));
    assert_eq!(s.y, Some(String::from("Test")));
}
