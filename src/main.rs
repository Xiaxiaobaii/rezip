use std::collections::HashMap;
use std::fs::{self};
use std::sync::LazyLock;
use std::path::{Path, PathBuf};
use file_format::FileFormat;
use tempfile::{TempDir};
use walkdir::{DirEntry, WalkDir};

use crate::config::CONFIG;
mod compress;
mod config;

static SUPPORTED_EXTENSIONS: LazyLock<HashMap<&str, fn(&Path) -> Result<TempDir, anyhow::Error>>> = std::sync::LazyLock::new(|| {
    let mut ret: HashMap<&str, fn(&Path) -> Result<TempDir, anyhow::Error>> = HashMap::new();
    ret.insert(FileFormat::Zip.extension(), compress::handler_zip);
    ret.insert(FileFormat::RoshalArchive.extension(), compress::handler_rar);
    ret.insert(FileFormat::SevenZip.extension(), compress::handler_sevenzip);
    ret
});

fn select_file(entry: &DirEntry, target_path: &PathBuf) -> Result<(), anyhow::Error> {
    let file_path = entry.path();
    let file_name = file_path.file_name().and_then(|x| x.to_str()).unwrap_or_default();
    let format = file_format::FileFormat::from_file(file_path)?;
    match SUPPORTED_EXTENSIONS.get(format.extension()) {
        Some(handle) => {
            println!("Decompress file {}", file_name);
            let tempdir = handle(file_path);
            let mut target_path = target_path.clone();
            target_path.push(file_name);
            
            match tempdir {
                Ok(tempdir) => {
                    let temp_path: Vec<Result<std::fs::DirEntry, std::io::Error>> = tempdir.path().read_dir().unwrap().collect();
                    if temp_path.len() == 0 {
                        fs::copy(file_path, target_path)?;
                        println!("Copy {},because before compressed zstd.", file_name);
                    }else {
                        println!("Convert {} to ZSTD", file_name);
                        target_path.set_extension("7z");
                        compress::compress_to_7z_zstd(tempdir.path(), &target_path)?;
                    }
                }
                Err(err) => {
                    println!("handler archive error: {err}");
                }
            }
        }
        None => {
            return Err(anyhow::Error::msg("unsupport Archive."));
        }
    }
    Ok(())
}

fn running(select_dir: &String, output_dir: &String) -> Result<(), anyhow::Error> {
    let output_dir = PathBuf::from(output_dir);
    if output_dir == *select_dir {
        return Err(anyhow::Error::msg("output dir with select dir is Same."));
    }else if output_dir.is_file() {
        return Err(anyhow::Error::msg("output_dir Path is file."));
    }
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }
    for entry in WalkDir::new(select_dir)
        .min_depth(1)
        .max_depth(CONFIG.max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) {
        if let Err(e) = select_file(&entry, &output_dir) {
            println!("read DirEntry {:?} wrose error: {e}", entry.file_name());
        }
    }
    println!("All archives processed.");
    Ok(())
}

fn main() {
    if let Err(e) = running(&CONFIG.select_dir, &CONFIG.output_dir) {
        println!("wrose error: {e}");
    }
}