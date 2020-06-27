use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
pub enum Empty {}

#[test]
fn test_empty_enum() {}
