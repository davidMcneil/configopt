mod arena_trait;
mod configopt_defaults_trait;

use arena_trait::Arena;
use colosseum::{sync::Arena as SyncArena, unsync::Arena as UnsyncArena};
use lazy_static::lazy_static;
use std::ffi::OsString;
use structopt::{clap::App, StructOpt};

pub use configopt_defaults_trait::{ConfigOptDefaults, ConfigOptToString};
pub use configopt_derive::ConfigOptDefaults;
pub use partial_derive::Partial;

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
            arg.v.default_val = Some(arena.alloc(default.into()));
        }
        arg_path.pop();
    }
    for (_, arg) in &mut app.p.positionals {
        arg_path.push(String::from(arg.b.name));
        if let Some(default) = defaults.arg_default(arg_path.as_slice()) {
            arg.v.default_val = Some(arena.alloc(default.into()));
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