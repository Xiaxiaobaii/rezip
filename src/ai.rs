use std::path::{Path};

use file_format::FileFormat;
use image::{DynamicImage, ImageReader};
use tempfile::{TempDir, tempdir};
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace, options::EncoderOptions};
use zune_jpegxl::JxlSimpleEncoder;

fn u16_to_bytes(data: &[u16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 2);
    for value in data {
        out.extend_from_slice(&value.to_ne_bytes());
    }
    out
}

fn image_to_jxl_input(
    image: DynamicImage,
) -> (Vec<u8>, u32, u32, ColorSpace, BitDepth) {
    let width = image.width();
    let height = image.height();
    match image.color() {
        image::ColorType::L8 => (
            image.to_luma8().into_raw(),
            width,
            height,
            ColorSpace::Luma,
            BitDepth::Eight,
        ),
        image::ColorType::La8 => (
            image.to_luma_alpha8().into_raw(),
            width,
            height,
            ColorSpace::LumaA,
            BitDepth::Eight,
        ),
        image::ColorType::Rgb8 => (
            image.to_rgb8().into_raw(),
            width,
            height,
            ColorSpace::RGB,
            BitDepth::Eight,
        ),
        image::ColorType::Rgba8 => (
            image.to_rgba8().into_raw(),
            width,
            height,
            ColorSpace::RGBA,
            BitDepth::Eight,
        ),
        image::ColorType::L16 => (
            u16_to_bytes(&image.to_luma16().into_raw()),
            width,
            height,
            ColorSpace::Luma,
            BitDepth::Sixteen,
        ),
        image::ColorType::La16 => (
            u16_to_bytes(&image.to_luma_alpha16().into_raw()),
            width,
            height,
            ColorSpace::LumaA,
            BitDepth::Sixteen,
        ),
        image::ColorType::Rgb16 => (
            u16_to_bytes(&image.to_rgb16().into_raw()),
            width,
            height,
            ColorSpace::RGB,
            BitDepth::Sixteen,
        ),
        image::ColorType::Rgba16 => (
            u16_to_bytes(&image.to_rgba16().into_raw()),
            width,
            height,
            ColorSpace::RGBA,
            BitDepth::Sixteen,
        ),
        _ => (
            image.to_rgba8().into_raw(),
            width,
            height,
            ColorSpace::RGBA,
            BitDepth::Eight,
        ),
    }
}

fn encode_to_jxl(image: DynamicImage, output_path: &Path) -> Result<(), anyhow::Error> {
    let (pixels, width, height, colorspace, depth) = image_to_jxl_input(image);
    let options = EncoderOptions::new(width as usize, height as usize, colorspace, depth);
    let encoder = JxlSimpleEncoder::new(&pixels, options);
    let mut encoded = Vec::new();
    encoder
        .encode(&mut encoded)
        .map_err(|e| anyhow::Error::msg(format!("jpegxl encode failed: {e:?}")))?;
    std::fs::write(output_path, encoded)?;
    Ok(())
}

fn convert_image_to_jxl(input: &Path) -> Result<TempDir, anyhow::Error> {
    
    let temp_dir = tempdir()?;
    let mut file_name = input
        .to_path_buf();
    file_name.set_extension(FileFormat::JpegXl.extension());
    file_name
        .file_name()
        .and_then(|x| x.to_str())
        .unwrap_or_default();
    let mut output_path = temp_dir.path().to_path_buf();
    output_path.push(file_name);

    let image = ImageReader::open(input)?.decode()?;
    encode_to_jxl(image, &output_path)?;

    Ok(temp_dir)
}

pub fn convert_jpeg_to_jxl(input: &Path) -> Result<TempDir, anyhow::Error> {
    convert_image_to_jxl(input)
}

pub fn convert_png_to_jxl(input: &Path) -> Result<TempDir, anyhow::Error> {
    convert_image_to_jxl(input)
}
