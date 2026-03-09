use std::collections::HashMap;
use std::fs::{self};
use std::sync::LazyLock;
use std::path::{Path, PathBuf};
use file_format::{FileFormat, Kind};
use tempfile::{TempDir};
use walkdir::{DirEntry, WalkDir};

use crate::config::CONFIG;
mod compress;
mod config;

type ExtensionHandlerType = fn(&Path) -> Result<TempDir, anyhow::Error>;
type ConvertHandlerType = fn(&Path, PathBuf) -> Result<(), anyhow::Error>;

static SUPPORTED_EXTENSIONS: LazyLock<HashMap<FileFormat, ExtensionHandlerType>> = std::sync::LazyLock::new(|| {
    let mut ret: HashMap<FileFormat, ExtensionHandlerType> = HashMap::new();
    ret.insert(FileFormat::Zip, compress::handler_zip);
    ret.insert(FileFormat::RoshalArchive, compress::handler_rar);
    ret.insert(FileFormat::SevenZip, compress::handler_sevenzip);
    ret
});


static CONVERT_HANDLERS: LazyLock<HashMap<Kind, ConvertHandlerType>> = LazyLock::new(|| {
    let mut ret: HashMap<Kind, ConvertHandlerType> = HashMap::new();
    ret.insert(Kind::Archive, compress::compress_to_7z_zstd);
    ret
});

fn select_file(entry: &DirEntry, target_path: &Path) -> Result<(), anyhow::Error> {
    let file_path = entry.path();

    let file_name = file_path.file_name().and_then(|x| x.to_str()).unwrap_or_default();
    let format = file_format::FileFormat::from_file(file_path)?;
    match SUPPORTED_EXTENSIONS.get(&format) {
        Some(handle) => {
            println!("Decompress file {}", file_name);
            let tempdir = handle(file_path);
            let mut target_path = target_path.to_path_buf();
            target_path.push(file_name);

            match tempdir {
                Ok(tempdir) => {
                    let temp_path: Vec<Result<std::fs::DirEntry, std::io::Error>> = tempdir.path().read_dir().unwrap().collect();
                    if temp_path.is_empty() {
                        fs::rename(file_name, target_path)?;
                        println!("Move {}. because don't need convert.", file_name);
                    }else if let Some(handler) = CONVERT_HANDLERS.get(&format.kind()) {
                        println!("Convert {}.", file_name);
                        handler(tempdir.path(), target_path)?;
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
    }
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }else if output_dir.is_file() {
        return Err(anyhow::Error::msg("output_dir Path is file."));
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