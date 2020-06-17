mod arena_trait;
mod configopt_arg_to_os_string;
mod error;

use arena_trait::Arena;
use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    process,
};
use structopt::{
    clap::{App, Result as ClapResult},
    StructOpt,
};

pub use configopt_arg_to_os_string::ConfigOptArgToOsString;
pub use configopt_derive::{configopt_fields, ConfigOpt};
pub use error::{Error, Result};

lazy_static! {
    static ref DEFAULT_VALUE_STORE: SyncArena<OsString> = SyncArena::new();
}

// This is very hacky. It reaches deep into clap internals to set the default values, but it works!
// We need to set the defaults to prevent the CLI parsing from failing when a required argument is
// not on the CLI but it is set in a config file.
fn set_defaults_impl<'a>(
    app: &mut App<'_, 'a>,
    arg_path: &mut Vec<String>,
    defaults: &impl ConfigOptArgToOsString,
    arena: &'a impl Arena<OsString>,
) {
    for arg in &mut app.p.opts {
        arg_path.push(String::from(arg.b.name));
        if let Some(default) = defaults.arg_to_os_string(arg_path.as_slice()) {
            arg.v.default_val = Some(arena.alloc(default));
        }
        arg_path.pop();
    }
    for (_, arg) in &mut app.p.positionals {
        arg_path.push(String::from(arg.b.name));
        if let Some(default) = defaults.arg_to_os_string(arg_path.as_slice()) {
            arg.v.default_val = Some(arena.alloc(default));
        }
        arg_path.pop();
    }
    // Recursively set defaults for subcommands
    for app in &mut app.p.subcommands {
        arg_path.push(app.p.meta.name.clone());
        set_defaults_impl(app, arg_path, defaults, arena);
        arg_path.pop();
    }
}

/// CODO
pub fn from_toml_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let contents =
        fs::read_to_string(path).map_err(|e| Error::ConfigFile(path.to_path_buf(), e))?;
    toml::from_str(&contents).map_err(|e| Error::ConfigFile(path.to_path_buf(), e.into()))
}

/// Set the defaults for a `clap::App`
pub fn set_defaults(app: &mut App<'_, 'static>, defaults: &impl ConfigOptArgToOsString) {
    let mut arg_path = Vec::new();
    let arena = &*DEFAULT_VALUE_STORE;
    set_defaults_impl(app, &mut arg_path, defaults, arena);
}

fn filter_help<I>(iter: I) -> impl Iterator<Item = OsString>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    iter.into_iter()
        .map(<I::Item as Into<OsString>>::into)
        .filter(|a| a != "-h" && a != "--help")
}

/// CODO
pub trait IgnoreHelp: StructOpt + Sized {
    /// CODO
    fn from_args_ignore_help() -> Self {
        Self::from_iter_ignore_help(env::args())
    }

    /// CODO
    fn try_from_args_ignore_help() -> ClapResult<Self> {
        Self::try_from_iter_ignore_help(env::args())
    }

    /// CODO
    fn from_iter_ignore_help<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let iter = filter_help(iter);
        Self::from_iter(iter)
    }

    /// CODO
    fn try_from_iter_ignore_help<I>(iter: I) -> ClapResult<Self>
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let iter = filter_help(iter);
        Self::from_iter_safe(iter)
    }
}

/// CODO
pub trait ConfigOptType: ConfigOptArgToOsString + StructOpt {
    /// If the `--generate-config` flag is set, return the current configuration.
    fn maybe_config_file(&self) -> Option<String>;

    /// If the `--generate-config` flag is set output the current configuration to stdout and exit.
    fn maybe_generate_config_file_and_exit(&self) {
        if let Some(config) = self.maybe_config_file() {
            let out = io::stdout();
            writeln!(&mut out.lock(), "{}", config).expect("Error writing Error to stdout");
            process::exit(0);
        }
    }

    /// Patch with values from the `--config-files` argument
    fn patch_with_config_files(&mut self) -> Result<&mut Self>;

    #[doc(hidden)]
    fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String;

    /// Generate TOML configuration.
    fn toml_config(&self) -> String {
        self.toml_config_with_prefix(&[])
    }
}

/// CODO
pub trait ConfigOpt: Sized + StructOpt {
    type ConfigOptType: ConfigOptType + IgnoreHelp;

    /// Set argument default values then get the struct from the command line arguments.
    ///
    /// Print the error message and quit the program in case of failure.
    fn from_args_with_defaults(defaults: &impl ConfigOptArgToOsString) -> Self {
        Self::try_from_args_with_defaults(defaults).unwrap_or_else(|e| e.exit())
    }

    /// Set argument default values then get the struct from the command line arguments.
    ///
    /// Returns a `configopt::Error` in case of failure. This does not exit in the case of --help,
    /// --version, or --generated-config, to achieve that behavior you must call `exit()` on the
    /// error value.
    fn try_from_args_with_defaults(defaults: &impl ConfigOptArgToOsString) -> ClapResult<Self> {
        Self::try_from_iter_with_defaults(env::args(), defaults)
    }

    /// Set argument default values then get the struct from any iterator such as a Vec of your making.
    ///
    /// Print the error message and quit the program in case of failure.
    fn from_iter_with_defaults<I>(iter: I, defaults: &impl ConfigOptArgToOsString) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        Self::try_from_iter_with_defaults(iter, defaults).unwrap_or_else(|e| e.exit())
    }

    /// Set argument default values then get the struct from any iterator such as a Vec of your making.
    ///
    /// Returns a configopt::Error in case of failure. This does not exit in the case of --help,
    /// --version, or --generated-config, to achieve the same behavior as `from_iter()` you must
    /// call .exit() on the error value.
    fn try_from_iter_with_defaults<I>(
        iter: I,
        defaults: &impl ConfigOptArgToOsString,
    ) -> ClapResult<Self>
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let mut app = Self::clap();
        // An arena allocator is used to extend the lifetimes of the default value strings.
        let arena = UnsyncArena::new();
        let mut arg_path = Vec::new();
        set_defaults_impl(&mut app, &mut arg_path, defaults, &arena);
        let matches = app.get_matches_from_safe(iter)?;
        Ok(Self::from_clap(&matches))
    }

    /// Get the struct, taking into account config files, from the command line arguments.
    ///
    /// Print the error message and quit the program in case of failure.
    fn from_args_with_configopt() -> Self {
        Self::try_from_iter_with_configopt(env::args()).unwrap_or_else(|e| e.exit())
    }

    /// Get the struct, taking into account config files, from any iterator such as a Vec of your
    /// making.
    ///
    /// Returns a configopt::Error in case of failure. This does not exit in the case of --help,
    /// --version, or --generated-config, to achieve the same behavior as `from_iter()` you must
    /// call .exit() on the error value.
    fn try_from_args_with_configopt() -> Result<Self> {
        Self::try_from_iter_with_configopt(env::args())
    }

    /// Get the struct, taking into account config files, from the command line arguments.
    ///
    /// Print the error message and quit the program in case of failure.
    fn from_iter_with_configopt<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        Self::try_from_iter_with_configopt(iter).unwrap_or_else(|e| e.exit())
    }

    /// Get the struct, taking into account config files, from any iterator such as a Vec of your
    /// making.
    ///
    /// Returns a configopt::Error in case of failure. This does not exit in the case of --help,
    /// --version, or --generated-config, to achieve the same behavior as `from_iter()` you must
    /// call .exit() on the error value.
    fn try_from_iter_with_configopt<I>(iter: I) -> Result<Self>
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let iter = iter.into_iter().map(Into::into).collect::<Vec<_>>();
        // We ignore the help for the `configopt` type so the help message is rendered for the
        // actual app.
        match Self::ConfigOptType::try_from_iter_ignore_help(&iter) {
            Ok(mut configopt) => {
                configopt.patch_with_config_files()?;
                if let Some(config) = configopt.maybe_config_file() {
                    return Err(Error::ConfigGenerated(config));
                }
                // Take into account any values from config files by setting default values. This
                // is needed so we do not get failures for missing arguments when they are really
                // set in the config file.
                let mut s = Self::try_from_iter_with_defaults(&iter, &configopt)?;
                // Take into account any values from config files by taking the values from the
                // configopt type. This is needed for types that do not always set their value if
                // a default is set (eg Option<T>)
                // TODO (DM): This should be patch?
                <Self as ConfigOpt>::take(&mut s, &mut configopt);
                Ok(s)
            }
            Err(e) => {
                // Get the error using the actual app
                Self::from_iter_safe(&iter)?;
                // We always expect an error to be generated. If we do not get an error return this
                // error type. This helps with debugging. It would be confusing if ever returned an
                // `Ok(Self)` with no config file information applied.
                Err(Error::ExpectedError(e))
            }
        }
    }

    /// CODO
    fn get_help(&self) -> String {
        let mut help = Vec::new();
        let app = Self::clap();
        app.write_help(&mut help).expect("failed to write to help");
        String::from_utf8_lossy(&help).to_string()
    }

    /// CODO
    fn get_long_help(&self) -> String {
        let mut help = Vec::new();
        let mut app = Self::clap();
        app.write_long_help(&mut help)
            .expect("failed to write to long help");
        String::from_utf8_lossy(&help).to_string()
    }

    #[doc(hidden)]
    fn take(&mut self, configopt: &mut Self::ConfigOptType);
}
