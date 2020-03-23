mod arena_trait;
mod configopt_defaults_trait;

use arena_trait::Arena;
use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};
use lazy_static::lazy_static;
use std::io::Write;
use std::{env, ffi::OsString, io, process};
use structopt::{clap::App, StructOpt};

pub use configopt_defaults_trait::ConfigOptDefaults;
pub use configopt_derive::ConfigOpt;

lazy_static! {
    static ref DEFAULT_VALUE_STORE: SyncArena<OsString> = SyncArena::new();
}

// This is very hacky. It reaches deep into clap internals to set the default values, but it works!
fn set_defaults_impl<'a>(
    app: &mut App<'_, 'a>,
    arg_path: &mut Vec<String>,
    defaults: &impl ConfigOptDefaults,
    arena: &'a impl Arena<OsString>,
) {
    for _ in &app.p.flags {
        // TODO: How to set the default of a flag
    }
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

pub fn env_args_contains(s: &[&str]) -> bool {
    for argument in env::args() {
        if s.contains(&argument.as_str()) {
            return true;
        }
    }
    false
}

/// TODO
pub trait TomlConfigGenerator {
    fn toml_config(&self) -> String {
        self.toml_config_with_prefix(&[])
    }

    fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String;

    fn write_toml_config_and_exit(&self, code: i32) {
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", self.toml_config()).expect("Error writing Error to stdout");
        process::exit(code);
    }
}

/// TODO
/// --generate-config
/// --config-file
pub trait ConfigOpt: Sized + StructOpt {
    /// Construct an instance of a `structopt` struct using a set of defaults
    fn from_args_with_defaults(defaults: &impl ConfigOptDefaults) -> Self {
        from_args_with_defaults(defaults)
    }
}

/// Set the defaults for a `clap::App`
pub fn set_defaults(app: &mut App<'_, 'static>, defaults: &impl ConfigOptDefaults) {
    let mut arg_path = Vec::new();
    let arena = &*DEFAULT_VALUE_STORE;
    set_defaults_impl(app, &mut arg_path, defaults, arena);
}

/// Construct an instance of a `structopt` struct using a set of defaults
pub fn from_args_with_defaults<T: StructOpt>(defaults: &impl ConfigOptDefaults) -> T {
    let mut app = T::clap();

    // An arena allocator is used to extend the lifetimes of the default value strings.
    let arena = UnsyncArena::new();
    let mut arg_path = Vec::new();
    set_defaults_impl(&mut app, &mut arg_path, defaults, &arena);

    let matches = app.get_matches();
    T::from_clap(&matches)
}
