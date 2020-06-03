use configopt::ConfigOpt;
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
struct NoDefaultConfigFile {
    #[structopt(long)]
    arg: String,
}

#[derive(ConfigOpt, StructOpt)]
// TODO: Make this use `=`
#[configopt(default_config_file("/my/default/config/file"))]
struct DefaultConfigFile {
    #[structopt(long)]
    arg: String,
}

#[test]
fn test_default_config_file() {
    assert_eq!(
        ConfigOptNoDefaultConfigFile::default_config_files(),
        Vec::<PathBuf>::new()
    );

    assert_eq!(
        ConfigOptDefaultConfigFile::default_config_files(),
        vec![PathBuf::from("/my/default/config/file")]
    );
}
