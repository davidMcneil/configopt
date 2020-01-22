use std::path::PathBuf;
use structopt::{clap::App, StructOpt};

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32,
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[structopt(short, parse(from_occurrences))]
        magicality: u64,
        #[structopt(short)]
        color: String,
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt, Debug)]
struct Finish {
    #[structopt(short)]
    time: u32,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    finish_type: FinishType,
}

// subsubcommand!
#[derive(StructOpt, Debug)]
enum FinishType {
    Glaze { applications: u32 },
    Powder { flavor: String, dips: u32 },
}

fn print_args(app: &App) {
    println!("=== {} ===", app.p.meta.name);
    for arg in &app.p.flags {
        println!("{}", arg.b.name);
    }
    for arg in &app.p.opts {
        println!("{} {:?}", arg.b.name, arg.v.default_val);
    }
    for (_, arg) in &app.p.positionals {
        println!("{} {:?}", arg.b.name, arg.v.default_val);
    }
    for app in &app.p.subcommands {
        print_args(app);
    }
}

fn main() {
    let mut app = Config::clap();
    print_args(&app);
    let matches = app.get_matches();
    let config = Config::from_args();
    println!("Config {:?}", config);
}
