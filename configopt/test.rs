#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use configopt::{configopt_fields, ConfigOpt};
use serde::Deserialize;
use structopt::StructOpt;
#[configopt(derive(Debug, PartialEq), attrs(serde))]
#[serde(deny_unknown_fields)]
struct MyStruct {
    #[structopt(long, default_value = "true")]
    maybe: Option<bool>,
    /// Paths to config files to read
    #[structopt(long = "config-files", hidden = false)]
    #[serde(skip)]
    config_files: Vec<::std::path::PathBuf>,
    /// Generate a TOML config
    #[structopt(long = "generate-config", hidden = false)]
    #[serde(skip)]
    generate_config: bool,
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
#[serde(deny_unknown_fields)]
struct ConfigOptMyStruct {
    #[structopt(long, default_value = "true")]
    maybe: Option<bool>,
    #[structopt(long = "config-files", hidden = false)]
    #[serde(skip)]
    config_files: Option<Vec<::std::path::PathBuf>>,
    #[structopt(long = "generate-config", hidden = false)]
    #[serde(skip)]
    #[structopt(default_value = "true")]
    generate_config: Option<bool>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
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
impl ::core::fmt::Debug for ConfigOptMyStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            ConfigOptMyStruct {
                maybe: ref __self_0_0,
                config_files: ref __self_0_1,
                generate_config: ref __self_0_2,
            } => {
                let mut debug_trait_builder = f.debug_struct("ConfigOptMyStruct");
                let _ = debug_trait_builder.field("maybe", &&(*__self_0_0));
                let _ = debug_trait_builder.field("config_files", &&(*__self_0_1));
                let _ = debug_trait_builder.field("generate_config", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
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
impl ::core::marker::StructuralPartialEq for ConfigOptMyStruct {}
#[automatically_derived]
#[allow(unused_qualifications)]
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
impl ::core::cmp::PartialEq for ConfigOptMyStruct {
    #[inline]
    fn eq(&self, other: &ConfigOptMyStruct) -> bool {
        match *other {
            ConfigOptMyStruct {
                maybe: ref __self_1_0,
                config_files: ref __self_1_1,
                generate_config: ref __self_1_2,
            } => match *self {
                ConfigOptMyStruct {
                    maybe: ref __self_0_0,
                    config_files: ref __self_0_1,
                    generate_config: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ConfigOptMyStruct) -> bool {
        match *other {
            ConfigOptMyStruct {
                maybe: ref __self_1_0,
                config_files: ref __self_1_1,
                generate_config: ref __self_1_2,
            } => match *self {
                ConfigOptMyStruct {
                    maybe: ref __self_0_0,
                    config_files: ref __self_0_1,
                    generate_config: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
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
impl ::core::default::Default for ConfigOptMyStruct {
    #[inline]
    fn default() -> ConfigOptMyStruct {
        ConfigOptMyStruct {
            maybe: ::core::default::Default::default(),
            config_files: ::core::default::Default::default(),
            generate_config: ::core::default::Default::default(),
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
impl ::structopt::StructOpt for ConfigOptMyStruct {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt");
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        ConfigOptMyStruct {
            maybe: if matches.occurrences_of("maybe") > 0 || matches.env_set("maybe") {
                matches
                    .value_of("maybe")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
            } else {
                None
            },
            config_files: if matches.is_present("config-files") {
                Some(
                    matches
                        .values_of("config-files")
                        .map_or_else(Vec::new, |v| {
                            v.map(|s| ::std::str::FromStr::from_str(s).unwrap())
                                .collect()
                        }),
                )
            } else {
                None
            },
            generate_config: if matches.occurrences_of("generate-config") > 0
                || matches.env_set("generate-config")
            {
                matches
                    .value_of("generate-config")
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
impl ::structopt::StructOptInternal for ConfigOptMyStruct {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        {
            let app = app;
            let app = app.arg(
                ::structopt::clap::Arg::with_name("maybe")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: bool| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("maybe")
                    .default_value("true"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("config-files")
                    .takes_value(true)
                    .multiple(true)
                    .min_values(0)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: ::std::path::PathBuf| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("config-files")
                    .hidden(false),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("generate-config")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: bool| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("generate-config")
                    .hidden(false)
                    .default_value("true"),
            );
            app.version("0.1.0")
        }
    }
    fn is_subcommand() -> bool {
        false
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ConfigOptMyStruct: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ConfigOptMyStruct {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 1",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "maybe" => _serde::export::Ok(__Field::__field0),
                        _ => _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS)),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"maybe" => _serde::export::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<ConfigOptMyStruct>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ConfigOptMyStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct ConfigOptMyStruct")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<Option<bool>>(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ConfigOptMyStruct with 1 element",
                                ));
                            }
                        };
                    let __field1 = _serde::export::Default::default();
                    let __field2 = _serde::export::Default::default();
                    _serde::export::Ok(ConfigOptMyStruct {
                        maybe: __field0,
                        config_files: __field1,
                        generate_config: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<bool>> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("maybe"),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<bool>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => match _serde::private::de::missing_field("maybe") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(ConfigOptMyStruct {
                        maybe: __field0,
                        config_files: _serde::export::Default::default(),
                        generate_config: _serde::export::Default::default(),
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["maybe"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ConfigOptMyStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<ConfigOptMyStruct>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
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
impl ConfigOptMyStruct {
    /// Take each field from `other` and set it in `self`
    pub fn take(&mut self, other: &mut ConfigOptMyStruct) {
        if (&mut other.maybe).is_some() {
            self.maybe = (&mut other.maybe).take();
        }
        if (&mut other.config_files).is_some() {
            self.config_files = (&mut other.config_files).take();
        }
        if (&mut other.generate_config).is_some() {
            self.generate_config = (&mut other.generate_config).take();
        }
    }
    /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
    pub fn patch(&mut self, other: &mut ConfigOptMyStruct) {
        if (&mut self.maybe).is_none() {
            self.maybe = (&mut other.maybe).take();
        }
        if (&mut self.config_files).is_none() {
            self.config_files = (&mut other.config_files).take();
        }
        if (&mut self.generate_config).is_none() {
            self.generate_config = (&mut other.generate_config).take();
        }
    }
    /// Take each field from `self` and set it in `other`
    pub fn take_for(&mut self, other: &mut MyStruct) {
        if (&mut self.maybe).is_some() {
            other.maybe = (&mut self.maybe).take();
        }
        if let Some(value) = (&mut self.config_files).take() {
            other.config_files = value;
        }
        if let Some(value) = (&mut self.generate_config).take() {
            other.generate_config = value;
        }
    }
    /// For each field in `other` if it is `None`, take the value from `self` and set it in `other`
    pub fn patch_for(&mut self, other: &mut MyStruct) {
        if (&mut other.maybe).is_none() {
            other.maybe = (&mut self.maybe).take();
        }
    }
    /// Check if all fields of `self` are `None`
    #[allow(clippy::eq_op)]
    pub fn is_empty(&self) -> bool {
        self.maybe.is_none() && self.config_files.is_none() && self.generate_config.is_none()
    }
    /// Check if all fields of `self` are `Some` applied recursively
    #[allow(clippy::eq_op)]
    pub fn is_complete(&self) -> bool {
        self.maybe.is_some() && self.config_files.is_some() && self.generate_config.is_some()
    }
    /// Check if `self` can be converted into a full version
    #[allow(clippy::eq_op)]
    pub fn is_convertible(&self) -> bool {
        true && true && true
    }
    /// Get the default config files
    pub fn default_config_files() -> Vec<::std::path::PathBuf> {
        Vec::new()
    }
    pub fn from_default_config_files() -> ::std::result::Result<Self, ::configopt::Error> {
        use std::convert::TryFrom;
        Self::try_from(Self::default_config_files().as_slice())
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
impl ::std::convert::From<MyStruct> for ConfigOptMyStruct {
    fn from(other: MyStruct) -> Self {
        Self {
            maybe: other.maybe,
            config_files: Some(other.config_files),
            generate_config: Some(other.generate_config),
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
impl ::std::convert::TryFrom<ConfigOptMyStruct> for MyStruct {
    type Error = ConfigOptMyStruct;
    fn try_from(configopt: ConfigOptMyStruct) -> ::std::result::Result<Self, Self::Error> {
        use std::convert::TryInto;
        if !configopt.is_convertible() {
            return Err(configopt);
        }
        Ok(Self {
            maybe: configopt.maybe,
            config_files: configopt.config_files.unwrap_or_default(),
            generate_config: configopt.generate_config.unwrap_or_default(),
        })
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
impl ::std::convert::TryFrom<&::std::path::Path> for ConfigOptMyStruct {
    type Error = ::configopt::Error;
    fn try_from(path: &::std::path::Path) -> ::std::result::Result<Self, Self::Error> {
        ::configopt::from_toml_file(path)
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
impl<T: ::std::convert::AsRef<::std::path::Path>> ::std::convert::TryFrom<&[T]>
    for ConfigOptMyStruct
{
    type Error = ::configopt::Error;
    fn try_from(paths: &[T]) -> ::std::result::Result<Self, Self::Error> {
        let mut result = ConfigOptMyStruct::default();
        for path in paths {
            match ConfigOptMyStruct::try_from(path.as_ref()) {
                Ok(mut from_default_config_file) => {
                    result.take(&mut from_default_config_file);
                }
                Err(e) if e.config_file_not_found() => {}
                Err(e) => return Err(e),
            }
        }
        Ok(result)
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
impl ::configopt::ConfigOptArgToOsString for ConfigOptMyStruct {
    fn arg_to_os_string(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
        let full_arg_path = arg_path;
        if let Some((arg_name, arg_path)) = full_arg_path.split_first() {
            match arg_name.as_str() {
                "maybe" => self
                    .maybe
                    .as_ref()
                    .and_then(|value| value.arg_to_os_string(arg_path)),
                "config-files" => {
                    if let Some(value) = self.config_files {
                        let vec = value
                            .iter()
                            .map(|value| value.arg_to_os_string(arg_path))
                            .flatten()
                            .collect::<Vec<_>>();
                        let mut result = ::std::ffi::OsString::new();
                        for (i, v) in vec.iter().enumerate() {
                            if i != 0 {
                                result.push(" ");
                            }
                            result.push(&v);
                        }
                        Some(result)
                    } else {
                        None
                    }
                }
                "generate-config" => self
                    .generate_config
                    .as_ref()
                    .and_then(|value| value.arg_to_os_string(arg_path)),
                _ => None,
            }
        } else {
            None
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
impl ::configopt::IgnoreHelp for ConfigOptMyStruct {}
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
impl ::configopt::ConfigOptType for ConfigOptMyStruct {
    fn maybe_config_file(&self) -> Option<String> {
        if self.generate_config.unwrap_or_default() {
            return Some(self.toml_config());
        }
        None
    }
    fn patch_with_config_files(&mut self) -> ::configopt::Result<&mut ConfigOptMyStruct> {
        use std::convert::TryFrom;
        let mut from_default_config_files = ConfigOptMyStruct::from_default_config_files()?;
        let mut from_config_files = if let Some(config_files) = &self.config_files {
            let mut from_config_files = ConfigOptMyStruct::try_from(config_files.as_slice())?;
            from_config_files.patch(&mut from_default_config_files);
            from_config_files
        } else {
            from_default_config_files
        };
        self.patch(&mut from_config_files);
        Ok(self)
    }
    fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
        let app = MyStruct::clap();
        let mut result = String::new();
        let key = if serde_prefix.is_empty() {
            String::from("maybe")
        } else {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "."],
                    &match (&serde_prefix.join("."), &"maybe") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }
        };
        let mut comment = String::new();
        let mut hidden = false;
        for arg in &app.p.flags {
            let b = &arg.b;
            if "maybe" == b.name {
                if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                    hidden = true;
                    break;
                }
                comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                break;
            }
        }
        if comment.is_empty() && !hidden {
            for arg in &app.p.opts {
                let b = &arg.b;
                if "maybe" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if comment.is_empty() && !hidden {
            for (_, arg) in &app.p.positionals {
                let b = &arg.b;
                if "maybe" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if !hidden && !&["generate-config", "config-files"].contains(&"maybe") {
            if !comment.is_empty() {
                comment = comment
                    .lines()
                    .map(|l| {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["### ", "\n"],
                            &match (&l,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })
                    .collect::<String>();
            }
            match toml::Value::try_from(&self.maybe) {
                Ok(val) => {
                    use toml::value::Value;
                    match &val {
                        Value::Array(a) if a.is_empty() => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "# ", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                        _ => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                    }
                }
                Err(toml::ser::Error::UnsupportedNone) => {
                    result = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "", "# ", " =\n\n"],
                            &match (&result, &comment, &key) {
                                (arg0, arg1, arg2) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    };
                }
                _ => {}
            }
        }
        let key = if serde_prefix.is_empty() {
            String::from("config_files")
        } else {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "."],
                    &match (&serde_prefix.join("."), &"config_files") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }
        };
        let mut comment = String::new();
        let mut hidden = false;
        for arg in &app.p.flags {
            let b = &arg.b;
            if "config-files" == b.name {
                if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                    hidden = true;
                    break;
                }
                comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                break;
            }
        }
        if comment.is_empty() && !hidden {
            for arg in &app.p.opts {
                let b = &arg.b;
                if "config-files" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if comment.is_empty() && !hidden {
            for (_, arg) in &app.p.positionals {
                let b = &arg.b;
                if "config-files" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if !hidden && !&["generate-config", "config-files"].contains(&"config-files") {
            if !comment.is_empty() {
                comment = comment
                    .lines()
                    .map(|l| {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["### ", "\n"],
                            &match (&l,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })
                    .collect::<String>();
            }
            match toml::Value::try_from(&self.config_files) {
                Ok(val) => {
                    use toml::value::Value;
                    match &val {
                        Value::Array(a) if a.is_empty() => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "# ", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                        _ => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                    }
                }
                Err(toml::ser::Error::UnsupportedNone) => {
                    result = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "", "# ", " =\n\n"],
                            &match (&result, &comment, &key) {
                                (arg0, arg1, arg2) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    };
                }
                _ => {}
            }
        }
        let key = if serde_prefix.is_empty() {
            String::from("generate_config")
        } else {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "."],
                    &match (&serde_prefix.join("."), &"generate_config") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }
        };
        let mut comment = String::new();
        let mut hidden = false;
        for arg in &app.p.flags {
            let b = &arg.b;
            if "generate-config" == b.name {
                if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                    hidden = true;
                    break;
                }
                comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                break;
            }
        }
        if comment.is_empty() && !hidden {
            for arg in &app.p.opts {
                let b = &arg.b;
                if "generate-config" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if comment.is_empty() && !hidden {
            for (_, arg) in &app.p.positionals {
                let b = &arg.b;
                if "generate-config" == b.name {
                    if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                        hidden = true;
                        break;
                    }
                    comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                    break;
                }
            }
        }
        if !hidden && !&["generate-config", "config-files"].contains(&"generate-config") {
            if !comment.is_empty() {
                comment = comment
                    .lines()
                    .map(|l| {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["### ", "\n"],
                            &match (&l,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })
                    .collect::<String>();
            }
            match toml::Value::try_from(&self.generate_config) {
                Ok(val) => {
                    use toml::value::Value;
                    match &val {
                        Value::Array(a) if a.is_empty() => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "# ", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                        _ => {
                            result = {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", "", "", " = ", "\n\n"],
                                    &match (&result, &comment, &key, &val) {
                                        (arg0, arg1, arg2, arg3) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg3,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            };
                        }
                    }
                }
                Err(toml::ser::Error::UnsupportedNone) => {
                    result = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", "", "# ", " =\n\n"],
                            &match (&result, &comment, &key) {
                                (arg0, arg1, arg2) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    };
                }
                _ => {}
            }
        }
        result
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
impl ::configopt::ConfigOpt for MyStruct {
    type ConfigOptType = ConfigOptMyStruct;
    fn patch(&mut self, other: &mut Self::ConfigOptType) {
        other.patch_for(self);
    }
    fn take(&mut self, other: &mut Self::ConfigOptType) {
        other.take_for(self);
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
impl ::structopt::StructOpt for MyStruct {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt");
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        MyStruct {
            maybe: if matches.occurrences_of("maybe") > 0 || matches.env_set("maybe") {
                matches
                    .value_of("maybe")
                    .map(|s| ::std::str::FromStr::from_str(s).unwrap())
            } else {
                None
            },
            config_files: matches
                .values_of("config-files")
                .map_or_else(Vec::new, |v| {
                    v.map(|s| ::std::str::FromStr::from_str(s).unwrap())
                        .collect()
                }),
            generate_config: matches.is_present("generate-config"),
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
                ::structopt::clap::Arg::with_name("maybe")
                    .takes_value(true)
                    .multiple(false)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: bool| ())
                            .map_err(|e| e.to_string())
                    })
                    .long("maybe")
                    .default_value("true"),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("config-files")
                    .takes_value(true)
                    .multiple(true)
                    .validator(|s| {
                        ::std::str::FromStr::from_str(s.as_str())
                            .map(|_: ::std::path::PathBuf| ())
                            .map_err(|e| e.to_string())
                    })
                    .help("Paths to config files to read")
                    .long("config-files")
                    .hidden(false),
            );
            let app = app.arg(
                ::structopt::clap::Arg::with_name("generate-config")
                    .takes_value(false)
                    .multiple(false)
                    .help("Generate a TOML config")
                    .long("generate-config")
                    .hidden(false),
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
impl ::core::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MyStruct {
                maybe: ref __self_0_0,
                config_files: ref __self_0_1,
                generate_config: ref __self_0_2,
            } => {
                let mut debug_trait_builder = f.debug_struct("MyStruct");
                let _ = debug_trait_builder.field("maybe", &&(*__self_0_0));
                let _ = debug_trait_builder.field("config_files", &&(*__self_0_1));
                let _ = debug_trait_builder.field("generate_config", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_MyStruct: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MyStruct {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 1",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "maybe" => _serde::export::Ok(__Field::__field0),
                        _ => _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS)),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"maybe" => _serde::export::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<MyStruct>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MyStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct MyStruct")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<Option<bool>>(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct MyStruct with 1 element",
                                ));
                            }
                        };
                    let __field1 = _serde::export::Default::default();
                    let __field2 = _serde::export::Default::default();
                    _serde::export::Ok(MyStruct {
                        maybe: __field0,
                        config_files: __field1,
                        generate_config: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<bool>> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("maybe"),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<bool>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => match _serde::private::de::missing_field("maybe") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(MyStruct {
                        maybe: __field0,
                        config_files: _serde::export::Default::default(),
                        generate_config: _serde::export::Default::default(),
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["maybe"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "MyStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<MyStruct>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
impl ::core::marker::StructuralPartialEq for MyStruct {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for MyStruct {
    #[inline]
    fn eq(&self, other: &MyStruct) -> bool {
        match *other {
            MyStruct {
                maybe: ref __self_1_0,
                config_files: ref __self_1_1,
                generate_config: ref __self_1_2,
            } => match *self {
                MyStruct {
                    maybe: ref __self_0_0,
                    config_files: ref __self_0_1,
                    generate_config: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &MyStruct) -> bool {
        match *other {
            MyStruct {
                maybe: ref __self_1_0,
                config_files: ref __self_1_1,
                generate_config: ref __self_1_2,
            } => match *self {
                MyStruct {
                    maybe: ref __self_0_0,
                    config_files: ref __self_0_1,
                    generate_config: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
fn main() {}
