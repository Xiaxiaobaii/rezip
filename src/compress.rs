use std::{
    fs::File,
    path::{Path, PathBuf},
};

use file_format::FileFormat;
use sevenz_rust2::{
    EncoderConfiguration, EncoderMethod, decompress_file, encoder_options::ZstandardOptions,
};
use tempfile::{TempDir, tempdir};
use zip::CompressionMethod;

use crate::config::CONFIG;

pub fn handler_zip(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let zip_file = File::open(zip_path)?;
    let mut archive = zip::read::ZipArchive::new(zip_file)?;
    let temp_dir = tempdir()?;
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.compression() != CompressionMethod::Stored {
            break;
        } else if !CONFIG.decompress_zstd_zip && file.compression() == CompressionMethod::Zstd {
            return Ok(temp_dir);
        }
    }
    archive.extract(&temp_dir)?;
    Ok(temp_dir)
}

pub fn handler_tar(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let temp_dir = tempdir()?;
    let mut archive = tar::Archive::new(File::open(zip_path)?);
    archive.unpack(temp_dir.path())?;
    Ok(temp_dir)
}

pub fn handler_sevenzip(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let archive = sevenz_rust2::Archive::open(zip_path)?;
    let temp_dir = tempdir()?;
    let block = archive.blocks.last();
    if let Some(block) = block {
        for coder in block.coders.clone() {
            if coder.encoder_method_id() == EncoderMethod::ID_ZSTD {
                return Ok(temp_dir);
            }
        }
    }
    decompress_file(zip_path, temp_dir.path())?;
    Ok(temp_dir)
}

pub fn handler_rar(zip_path: &Path) -> Result<TempDir, anyhow::Error> {
    let mut archive = unrar::Archive::new(zip_path).open_for_processing().unwrap();
    let temp_dir = tempdir()?;
    while let Some(arch) = archive.read_header()? {
        let mut fil = PathBuf::from(temp_dir.path());
        fil.push(arch.entry().filename.clone());
        archive = arch.extract_to(&fil)?;
    }
    Ok(temp_dir)
}

pub fn compress_to_7z_zstd(
    source_dir: &Path,
    mut output_path: PathBuf,
) -> Result<(), anyhow::Error> {
    output_path.set_extension(FileFormat::SevenZip.extension());
    let mut writer = sevenz_rust2::ArchiveWriter::create(output_path)?;
    writer.set_content_methods(vec![
        EncoderConfiguration::new(EncoderMethod::ZSTD).with_options(
            sevenz_rust2::encoder_options::EncoderOptions::Zstd(ZstandardOptions::from_level(
                CONFIG.zstd_compress_level,
            )),
        ),
    ]);
    writer.push_source_path(source_dir, |_| true)?;
    writer.finish()?;
    Ok(())
}
