use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
pub enum Empty {
    Test {
        #[structopt()]
        x: Option<u32>,
        #[structopt(flatten)]
        testing: Testing,
    },
}
#[derive(ConfigOpt, StructOpt)]
pub struct Testing {
    #[structopt()]
    x: Option<String>,
}

fn main() {}
