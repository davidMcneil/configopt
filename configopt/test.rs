#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use configopt::ConfigOpt;
use structopt::StructOpt;
pub enum Empty {}
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
pub enum ConfigOptEmpty {}
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
impl ::structopt::StructOpt for ConfigOptEmpty {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt")
            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        <ConfigOptEmpty as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand())
            .unwrap()
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
impl ::structopt::StructOptInternal for ConfigOptEmpty {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        let app = app;
        app.version("0.1.0")
    }
    fn from_subcommand<'a, 'b>(
        sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>),
    ) -> Option<Self> {
        match sub {
            other => {
                None
            }
        }
    }
    fn is_subcommand() -> bool {
        true
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ConfigOptEmpty: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ConfigOptEmpty {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {}
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 0",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
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
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
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
                marker: _serde::export::PhantomData<ConfigOptEmpty>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ConfigOptEmpty;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "enum ConfigOptEmpty")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    _serde::export::Result::map(
                        _serde::de::EnumAccess::variant::<__Field>(__data),
                        |(__impossible, _)| match __impossible {},
                    )
                }
            }
            const VARIANTS: &'static [&'static str] = &[];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ConfigOptEmpty",
                VARIANTS,
                __Visitor {
                    marker: _serde::export::PhantomData::<ConfigOptEmpty>,
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
impl ConfigOptEmpty {
    /// Take each field from `other` and set it in `self`
    pub fn take(&mut self, other: &mut ConfigOptEmpty) {
        match (self, other) {
            _ => {}
        }
    }
    /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
    pub fn patch(&mut self, other: &mut ConfigOptEmpty) {
        match (self, other) {
            _ => {}
        }
    }
    /// Take each field from `self` and set it in `other`
    pub fn take_for(&mut self, other: &mut Empty) {
        match (self, other) {
            _ => {}
        }
    }
    /// For each field in `other` if it is `None`, take the value from `self` and set it in `other`
    pub fn patch_for(&mut self, other: &mut Empty) {
        match (self, other) {
            _ => {}
        }
    }
    /// Check if all fields of `self` are `Some` applied recursively
    pub fn is_complete(&self) -> bool {
        match self {}
    }
    /// Check if `self` can be converted into a full version
    pub fn is_convertible(&self) -> bool {
        match self {}
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
impl ::std::convert::From<Empty> for ConfigOptEmpty {
    fn from(other: Empty) -> Self {
        match other {}
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
impl ::std::convert::TryFrom<ConfigOptEmpty> for Empty {
    type Error = ConfigOptEmpty;
    fn try_from(configopt: ConfigOptEmpty) -> ::std::result::Result<Self, Self::Error> {
        use ::std::convert::TryInto;
        if !configopt.is_convertible() {
            return Err(configopt);
        }
        match configopt {}
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
impl ::configopt::ConfigOptArgToOsString for ConfigOptEmpty {
    fn arg_to_os_string(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
        match self {
            _ => None,
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
impl ::configopt::IgnoreHelp for ConfigOptEmpty {}
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
impl ::configopt::ConfigOptType for ConfigOptEmpty {
    fn maybe_config_file(&self) -> Option<String> {
        match self {
            _ => {}
        }
        None
    }
    fn patch_with_config_files(&mut self) -> ::configopt::Result<&mut ConfigOptEmpty> {
        match self {
            _ => {}
        }
        Ok(self)
    }
    fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
        {
            ::std::rt::begin_panic("not yet implemented")
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
impl ::configopt::ConfigOpt for Empty {
    type ConfigOptType = ConfigOptEmpty;
    fn patch(&mut self, other: &mut Self::ConfigOptType) {
        other.patch_for(self);
    }
    fn take(&mut self, other: &mut Self::ConfigOptType) {
        other.patch_for(self);
    }
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
impl ::structopt::StructOpt for Empty {
    fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
        let app = ::structopt::clap::App::new("configopt")
            .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
        <Self as ::structopt::StructOptInternal>::augment_clap(app)
    }
    fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
        <Empty as ::structopt::StructOptInternal>::from_subcommand(matches.subcommand()).unwrap()
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
impl ::structopt::StructOptInternal for Empty {
    fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
        let app = app;
        app.version("0.1.0")
    }
    fn from_subcommand<'a, 'b>(
        sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>),
    ) -> Option<Self> {
        match sub {
            other => {
                None
            }
        }
    }
    fn is_subcommand() -> bool {
        true
    }
}
