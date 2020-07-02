use configopt::ConfigOpt;
use std::path::PathBuf;
use structopt::StructOpt;

fn empty() -> Vec<PathBuf> {
    vec![]
}

fn some_paths() -> Vec<PathBuf> {
    vec![PathBuf::from("/path/1"), PathBuf::from("/path/2")]
}

#[derive(ConfigOpt, StructOpt)]
struct NoDefaultConfigFile {
    #[structopt(long)]
    arg: String,
}

#[derive(ConfigOpt, StructOpt)]
#[configopt(default_config_file("/my/default/config/file"))]
struct DefaultConfigFile {
    #[structopt(long)]
    arg: String,
}

#[derive(ConfigOpt, StructOpt)]
#[configopt(default_config_file(empty))]
struct DefaultConfigFile2 {
    #[structopt(long)]
    arg: String,
}

#[derive(ConfigOpt, StructOpt)]
#[configopt(default_config_file(some_paths))]
struct DefaultConfigFile3 {
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

    assert_eq!(
        ConfigOptDefaultConfigFile2::default_config_files(),
        Vec::<PathBuf>::new()
    );

    assert_eq!(
        ConfigOptDefaultConfigFile3::default_config_files(),
        vec![PathBuf::from("/path/1"), PathBuf::from("/path/2")]
    );
}
