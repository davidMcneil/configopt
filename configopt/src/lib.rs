mod arena_trait;
mod configopt_defaults_trait;

use arena_trait::Arena;
use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};
use lazy_static::lazy_static;
use std::ffi::OsString;
use structopt::{
    clap::{App, Result as ClapResult},
    StructOpt,
};

pub use configopt_defaults_trait::ConfigOptDefaults;
pub use configopt_derive::{configopt_fields, ConfigOpt};

lazy_static! {
    static ref DEFAULT_VALUE_STORE: SyncArena<OsString> = SyncArena::new();
}

// This is very hacky. It reaches deep into clap internals to set the default values, but it works!
// We need to set the defaults to prevent the CLI parsing from failing when a required argument is
// not on the CLI but it is set in a config file.
fn set_defaults_impl<'a>(
    app: &mut App<'_, 'a>,
    arg_path: &mut Vec<String>,
    defaults: &impl ConfigOptDefaults,
    arena: &'a impl Arena<OsString>,
) {
    for arg in &mut app.p.opts {
        arg_path.push(String::from(arg.b.name));
        if let Some(default) = defaults.arg_default(arg_path.as_slice()) {
            arg.v.default_val = Some(arena.alloc(default));
        }
        arg_path.pop();
    }
    for (_, arg) in &mut app.p.positionals {
        arg_path.push(String::from(arg.b.name));
        if let Some(default) = defaults.arg_default(arg_path.as_slice()) {
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

/// Set the defaults for a `clap::App`
pub fn set_defaults(app: &mut App<'_, 'static>, defaults: &impl ConfigOptDefaults) {
    let mut arg_path = Vec::new();
    let arena = &*DEFAULT_VALUE_STORE;
    set_defaults_impl(app, &mut arg_path, defaults, arena);
}

pub trait ConfigOptType: ConfigOptDefaults + StructOpt {
    /// If the `--generate-config` flag is set output the current configuration to stdout and exit.
    fn maybe_generate_config_file_and_exit(&mut self);

    /// Patch with values from the `--config-files` argument
    fn patch_with_config_files(&mut self) -> std::result::Result<&mut Self, ::std::io::Error>;

    fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String;

    /// Generate TOML configuration.
    fn toml_config(&self) -> String {
        self.toml_config_with_prefix(&[])
    }
}

pub trait ConfigOpt: Sized + StructOpt {
    type ConfigOptType: ConfigOptType;

    /// Set default values. Then gets the struct from the command line arguments. Print the error
    /// message and quit the program in case of failure.
    fn from_args_with_defaults(defaults: &impl ConfigOptDefaults) -> Self {
        let mut app = Self::clap();
        // An arena allocator is used to extend the lifetimes of the default value strings.
        let arena = UnsyncArena::new();
        let mut arg_path = Vec::new();
        set_defaults_impl(&mut app, &mut arg_path, defaults, &arena);
        let matches = app.get_matches();
        Self::from_clap(&matches)
    }

    /// Set default values. Then gets the struct from any iterator such as a Vec of your making.
    /// Print the error message and quit the program in case of failure.
    fn from_iter_with_defaults<I>(iter: I, defaults: &impl ConfigOptDefaults) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let mut app = Self::clap();
        // An arena allocator is used to extend the lifetimes of the default value strings.
        let arena = UnsyncArena::new();
        let mut arg_path = Vec::new();
        set_defaults_impl(&mut app, &mut arg_path, defaults, &arena);
        let matches = app.get_matches_from(iter);
        Self::from_clap(&matches)
    }

    /// Set default values. Then gets the struct from any iterator such as a Vec of your making.
    ///
    /// Returns a clap::Error in case of failure. This does not exit in the case of --help or
    /// --version, to achieve the same behavior as from_iter() you must call .exit() on the error
    /// value.
    fn try_from_iter_with_defaults<I>(
        iter: I,
        defaults: &impl ConfigOptDefaults,
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

    /// Gets the struct from the command line arguments taking into account `configopt` arguments.
    /// Print the error message and quit the program in case of failure.
    fn from_args() -> Self {
        match Self::ConfigOptType::from_iter_safe(::std::env::args()) {
            Ok(mut configopt) => {
                configopt.maybe_generate_config_file_and_exit();
                match configopt.patch_with_config_files() {
                    Ok(configopt) => {
                        let mut s = Self::from_args_with_defaults(configopt);
                        <Self as ConfigOpt>::take(&mut s, configopt);
                        s
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }

    fn take(&mut self, configopt: &mut Self::ConfigOptType);
}
