use colosseum::sync::Arena as SyncArena;
use lazy_static::lazy_static;
use std::ffi::OsString;
use structopt::{clap::App, StructOpt};
use typed_arena::Arena;

lazy_static! {
    static ref DEFAULT_VALUE_STORE: SyncArena<OsString> = SyncArena::new();
}

// This is very hacky. It reaches deep into clap internals to set the default values, but it works!
fn set_defaults_impl<'a>(
    app: &mut App<'_, 'a>,
    arg_path: &mut Vec<String>,
    defaults: &impl ConfigOptDefaults,
    arena: &'a Arena<OsString>,
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

fn set_defaults_static_impl(
    app: &mut App<'_, 'static>,
    arg_path: &mut Vec<String>,
    defaults: &impl ConfigOptDefaults,
) {
    let arena = &DEFAULT_VALUE_STORE;
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
        set_defaults_static_impl(app, arg_path, defaults);
        arg_path.pop();
    }
}

pub fn set_defaults(app: &mut App<'_, 'static>, defaults: &impl ConfigOptDefaults) {
    let mut arg_path = Vec::new();
    set_defaults_static_impl(app, &mut arg_path, defaults);
}

pub fn from_args_with_defaults<T: StructOpt>(defaults: &impl ConfigOptDefaults) -> T {
    let mut app = T::clap();

    // An arena allocator is used to extend the lifetimes of the default value strings.
    let arena = Arena::new();
    let mut arg_path = Vec::new();
    set_defaults_impl(&mut app, &mut arg_path, defaults, &arena);

    let matches = app.get_matches();
    T::from_clap(&matches)
}

pub trait ConfigOptDefaults {
    fn arg_default(&self, arg_path: &[String]) -> Option<String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
