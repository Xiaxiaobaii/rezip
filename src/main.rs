use std::fs::{self, File};
use std::sync::LazyLock;
use std::{env, process};
use std::path::{Path, PathBuf};
use file_format::FileFormat;
use sevenz_rust2::encoder_options::ZstandardOptions;
use sevenz_rust2::{EncoderConfiguration, EncoderMethod, decompress_file};
use tempfile::{TempDir, tempdir};
use walkdir::{DirEntry, WalkDir};
use zip::CompressionMethod;

static SUPPORTED_EXTENSIONS: LazyLock<Vec<FileFormat>> = std::sync::LazyLock::new(|| {
    vec![FileFormat::Zip, FileFormat::RoshalArchive, FileFormat::SevenZip]
});

fn handler_zip(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let zip_file = File::open(zip_path)?;
    let mut archive = zip::read::ZipArchive::new(zip_file)?;
    let temp_dir = tempdir()?;
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.compression() == CompressionMethod::Zstd {
            return Ok(temp_dir);
        }else if file.compression() != CompressionMethod::Stored {
            break;
        }
    }
    archive.extract(&temp_dir)?;
    Ok(temp_dir)
}

fn handler_sevenzip(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let archive = sevenz_rust2::Archive::open(zip_path)?;
    let temp_dir = tempdir()?;
    let block = archive.blocks.last();
    if let Some(block) = block {
        for coder in block.coders.clone() {
            if coder.encoder_method_id() == EncoderMethod::ID_ZSTD {
                return Ok(temp_dir)
            }
        }
    }
    decompress_file(zip_path, temp_dir.path())?;
    Ok(temp_dir)
}

fn handler_rar(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let mut archive =
        unrar::Archive::new(zip_path)
            .open_for_processing()
            .unwrap();
    let temp_dir = tempdir()?;
    while let Some(arch) = archive.read_header()? {
        let mut fil = PathBuf::from(temp_dir.path());
        fil.push(arch.entry().filename.clone());
        archive = arch.extract_to(&fil)?;
    }
    Ok(temp_dir)
}
fn compress_to_7z_zstd(source_dir: &Path, output_path: &Path) -> Result<(), anyhow::Error> {
    let mut writer = sevenz_rust2::ArchiveWriter::create(output_path)?;
    writer.set_content_methods(vec![EncoderConfiguration::new(EncoderMethod::ZSTD).with_options(sevenz_rust2::encoder_options::EncoderOptions::Zstd(ZstandardOptions::from_level(16)))]);
    writer.push_source_path(source_dir, |_| { true })?;
    writer.finish()?;
    Ok(())
}

fn select_file(entry: &DirEntry, target_path: &PathBuf) -> Result<(), anyhow::Error> {
    let file_path = entry.path();
    let file_name = file_path.file_name().and_then(|x| x.to_str()).unwrap_or_default();
    let format = file_format::FileFormat::from_file(file_path)?;
    if SUPPORTED_EXTENSIONS.contains(&format) {
        println!("Decompress file {}", file_name);
        let tempdir = match format {
            FileFormat::Zip => {
                handler_zip(file_path)
            },
            FileFormat::SevenZip => {
                handler_sevenzip(file_path)
            },
            FileFormat::RoshalArchive => {
                handler_rar(file_path)
            },
            _ => {
                Err(anyhow::Error::msg("unsupport Archive."))
            },
        };
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
                    compress_to_7z_zstd(tempdir.path(), &target_path)?;
                }
            }
            Err(err) => {
                println!("handler archive error: {err}");
            }
        }
    }
    Ok(())
}

fn running(select_dir: &String, output_dir: &String) -> Result<(), anyhow::Error> {
    let output_dir = PathBuf::from(output_dir);
    if output_dir == PathBuf::from(select_dir) {
        return Err(anyhow::Error::msg("output dir with select dir is Same."));
    }else if output_dir.is_file() {
        return Err(anyhow::Error::msg("output_dir Path is file."));
    }
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }
    for entry in WalkDir::new(select_dir)
        .min_depth(1)
        .max_depth(1)
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
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: rezip <select_dir> <output_dir>");
        process::exit(0)
    }
    let mut argdir = Vec::new();
    let mut cursor = 1;
    while cursor < args.len() {
        match args[cursor].as_str() {
            _ => {
                argdir.push(&args[cursor]);
                cursor += 1;
            }
        }
    }
    if let Err(e) = running(argdir[0], argdir[1]) {
        println!("wrose error: {e}");
    }
}