use configopt;
use partial_derive::Partial;
use std::env;
use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// struct Config {
//     /// The supervising faerie.
//     #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
//     supervising_faerie: String,
//     /// The faerie tree this cookie is being made in.
//     tree: Option<String>,
//     #[structopt(subcommand)] // Note that we mark a field as a subcommand
//     cmd: Command,
// }

// #[derive(StructOpt, Debug)]
// enum Command {
//     /// Pound acorns into flour for cookie dough.
//     Pound {
//         acorns: u32,
//     },
//     /// Add magical sparkles -- the secret ingredient!
//     Sparkle {
//         /// How magical should we make it?
//         #[structopt(short, parse(from_occurrences))]
//         magicality: u64,
//         /// The color of the sparkles.
//         #[structopt(short)]
//         color: String,
//     },
//     Finish(Finish),
// }

// // Subcommand can also be externalized by using a 1-uple enum variant
// #[derive(StructOpt, Debug)]
// struct Finish {
//     /// The time to finish.
//     #[structopt(short)]
//     time: u32,
//     #[structopt(subcommand)] // Note that we mark a field as a subcommand
//     finish_type: FinishType,
// }

// // subsubcommand!
// #[derive(StructOpt, Debug)]
// enum FinishType {
//     Glaze {
//         /// How many times to apply the glaze.
//         applications: u32,
//     },
//     Powder {
//         /// The flavor of the powder.
//         #[structopt(long)]
//         flavor: String,
//         /// The number of dips in the powder.
//         dips: u32,
//     },
// }

// struct Test;

// impl ConfigOptDefaults for Test {
//     fn arg_default(&self, arg_path: &[String]) -> Option<String> {
//         match arg_path
//             .iter()
//             .map(String::as_ref)
//             .collect::<Vec<_>>()
//             .as_slice()
//         {
//             ["supervisor"] => Some(String::from("Gwenie")),
//             ["finish", "powder", "flavor"] => Some(String::from("sugar")),
//             ["finish", "powder", "dips"] => Some(String::from("42")),
//             _ => None,
//         }
//     }
// }

#[derive(Debug, Partial, StructOpt)]
#[partial(derive(Default, StructOpt))]
#[partial(attrs(structopt))]
struct Config {
    /// You need some help!
    #[structopt(long)]
    x_y_z: u32,
}

fn main() {
    let mut partial = PartialConfig::default();
    partial.x_y_z = Some(42);
    let config: Config = configopt::from_args_with_defaults(&partial);
    println!("Config {:?}", config);
}
