use std::sync::LazyLock;

use clap::Parser;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

#[derive(Parser)]
pub struct Config {
    /// origin file dir.
    pub select_dir: String,

    /// output dir.
    pub output_dir: String,

    /// Scan the directory depth of "select_dir".
    #[arg(short('l'), long, default_value_t = 1)]
    pub max_depth: usize,

    /// set zstd compress level.
    #[arg(long, default_value_t = 16)]
    pub zstd_compress_level: u32,

    /// delete origin file instead of keep.
    #[arg(short('d'), long, default_value_t = false)]
    pub delete_origin: bool,

    /// decompress zip (zstf) instead of move.
    #[arg(long, default_value_t = false)]
    pub decompress_zstd_zip: bool,

    /// enable test features. jpeg/png to jpeg XL.
    #[arg(long, default_value_t = false)]
    pub enable_image_test: bool,
}
