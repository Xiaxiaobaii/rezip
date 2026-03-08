use std::sync::LazyLock;

use clap::Parser;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

#[derive(Parser)]
pub struct Config {
    pub select_dir: String,

    pub output_dir: String,

    #[arg(short('l'), long, default_value_t = 1)]
    pub max_depth: usize,
}