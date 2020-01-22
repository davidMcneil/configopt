use clap::{App, Arg, ArgMatches, ArgSettings, Clap, FromArgMatches, IntoApp};
use std::path::PathBuf;

#[derive(Clap, Debug)]
struct Config {
    #[clap(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[clap(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(Clap, Debug)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32,
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[clap(short = "m", parse(from_occurrences))]
        magicality: u64,
        #[clap(short = "c")]
        color: String,
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(Clap, Debug)]
struct Finish {
    #[clap(short = "t")]
    time: u32,
    #[clap(subcommand)] // Note that we mark a field as a subcommand
    finish_type: FinishType,
}

// subsubcommand!
#[derive(Clap, Debug)]
enum FinishType {
    Glaze { applications: u32 },
    Powder { flavor: String, dips: u32 },
}

fn print_args(app: &App) {
    println!("=== {} ===", app.name);
    for arg in &app.args.args {
        println!("{}", arg.name);
    }
    for app in &app.subcommands {
        print_args(app);
    }
}

fn main() {
    let mut app = Config::into_app();
    print_args(&app);
    let matches = app.get_matches();
    let config = Config::from_argmatches(&matches);
    println!("Config {:?}", config);
}
