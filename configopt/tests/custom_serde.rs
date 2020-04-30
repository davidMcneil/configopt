// use configopt::{configopt_fields, ConfigOpt, ConfigOptDefaults, ConfigOptType};
// use serde::Deserialize;
// use std::{ffi::OsString, io, path::PathBuf, str::FromStr};
// use structopt::StructOpt;

// mod serde_string {
//     use serde;
//     use std::{error, fmt, marker::PhantomData, str::FromStr};

//     pub fn deserialize<'de, T, E, D>(d: D) -> Result<T, D::Error>
//     where
//         T: FromStr<Err = E>,
//         E: error::Error,
//         D: serde::Deserializer<'de>,
//     {
//         struct FromStringable<T, E>(PhantomData<T>, PhantomData<E>);

//         impl<'de, T, E> serde::de::Visitor<'de> for FromStringable<T, E>
//         where
//             T: FromStr<Err = E>,
//             E: error::Error,
//         {
//             type Value = T;

//             fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 formatter.write_str("a string")
//             }

//             fn visit_str<R>(self, value: &str) -> Result<T, R>
//             where
//                 R: serde::de::Error,
//             {
//                 match FromStr::from_str(value) {
//                     Ok(t) => Ok(t),
//                     Err(err) => Err(R::custom(format!(
//                         "string cannot be parsed: \"{}\" ({})",
//                         value, err
//                     ))),
//                 }
//             }
//         }

//         d.deserialize_any(FromStringable(PhantomData, PhantomData))
//     }

//     pub fn serialize<T, S>(t: &T, s: S) -> Result<S::Ok, S::Error>
//     where
//         T: ToString,
//         S: serde::Serializer,
//     {
//         s.serialize_str(&t.to_string())
//     }
// }

// #[derive(Debug, PartialEq, Deserialize)]
// struct CustomString(String);

// impl FromStr for CustomString {
//     type Err = io::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(CustomString(String::from(s)))
//     }
// }

// #[configopt_fields]
// #[derive(StructOpt, Debug, Deserialize, PartialEq)]
// // #[configopt(derive(Debug, PartialEq), attrs(serde))]
// #[serde(deny_unknown_fields)]
// struct MyStruct {
//     #[structopt(long)]
//     // #[serde(with = "serde_string")]
//     custom: CustomString,
// }

// #[configopt_fields]
// #[derive(StructOpt, Debug, Deserialize, PartialEq)]
// // #[configopt(derive(Debug, PartialEq), attrs(serde))]
// #[serde(deny_unknown_fields)]
// struct ConfigOptMyStruct {
//     #[structopt(long)]
//     custom: Option<CustomString>,
// }

// #[test]
// fn test_custom_serde() {
//     let mut s = ConfigOptMyStruct::clap();
//     for arg in &mut s.p.opts {
//         println!("{:?}", arg.b.name);
//     }
// }
