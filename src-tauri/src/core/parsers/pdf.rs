use std::path::Path;

use crate::core::{clients::openai::OpenAiClient, parsers::image::describe_image};
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use pdf_extract::{xobject::PdfImage, Document, Error as PdfError, Stream};

const PDF_IMAGE_DESCRIPTION_MIME_TYPE: &str = "image/png";
const JPEG_IMAGE_DESCRIPTION_MIME_TYPE: &str = "image/jpeg";

pub fn extract_pdf_text(path: &Path) -> Result<String, String> {
    ensure_supported_pdf_file(path)?;

    let pages = pdf_extract::extract_text_by_pages(path).map_err(|err| {
        format!(
            "failed to extract text from PDF file {}: {err}",
            path.display()
        )
    })?;

    Ok(pages
        .iter()
        .map(|page| clean_pdf_page_text(page))
        .filter(|page| !page.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n"))
}

pub fn parse_pdf_file(path: &Path) -> Result<String, String> {
    extract_pdf_text(path)
}

pub async fn extract_pdf_image_descriptions(
    path: &Path,
    openai_client: &OpenAiClient<'_>,
) -> Result<Vec<String>, String> {
    let images = extract_pdf_images(path)?;
    let mut descriptions = Vec::with_capacity(images.len());

    for image in images {
        let description = describe_image(&image.bytes, image.mime_type, openai_client)
            .await
            .map_err(|err| {
                format!(
                    "failed to describe image {} on PDF page {}: {err}",
                    image.image_index, image.page_number
                )
            })?;
        descriptions.push(description);
    }

    Ok(descriptions)
}

struct ExtractedPdfImage {
    page_number: u32,
    image_index: usize,
    bytes: Vec<u8>,
    mime_type: &'static str,
}

fn extract_pdf_images(path: &Path) -> Result<Vec<ExtractedPdfImage>, String> {
    ensure_supported_pdf_file(path)?;

    let document = Document::load(path)
        .map_err(|err| format!("failed to load PDF file {}: {err}", path.display()))?;
    let mut images = Vec::new();

    for (page_number, page_id) in document.get_pages() {
        let page_images = match document.get_page_images(page_id) {
            Ok(images) => images,
            Err(PdfError::DictKey(key)) if key == "Resources" || key == "XObject" => Vec::new(),
            Err(err) => {
                return Err(format!(
                    "failed to extract images from PDF page {page_number}: {err}"
                ))
            }
        };

        for (image_index, image) in page_images.iter().enumerate() {
            let (bytes, mime_type) = encode_pdf_image(image).map_err(|err| {
                format!(
                    "failed to encode image {} on PDF page {}: {err}",
                    image_index + 1,
                    page_number
                )
            })?;
            images.push(ExtractedPdfImage {
                page_number,
                image_index: image_index + 1,
                bytes,
                mime_type,
            });
        }
    }

    Ok(images)
}

fn encode_pdf_image(image: &PdfImage<'_>) -> Result<(Vec<u8>, &'static str), String> {
    if has_image_filter(image, "DCTDecode") {
        return Ok((image.content.to_vec(), JPEG_IMAGE_DESCRIPTION_MIME_TYPE));
    }

    if has_image_filter(image, "JPXDecode") {
        return Err("JPXDecode/JPEG 2000 PDF images are not supported yet".to_string());
    }

    let content = plain_image_content(image)?;
    let width = image_dimension("width", image.width)?;
    let height = image_dimension("height", image.height)?;
    let bits_per_component = image.bits_per_component.unwrap_or(8);

    if bits_per_component != 8 {
        return Err(format!(
            "unsupported PDF image bit depth {bits_per_component}; expected 8"
        ));
    }

    let (pixels, color_type) = match image.color_space.as_deref() {
        Some("DeviceGray") => (content, ColorType::L8),
        Some("DeviceRGB") => (content, ColorType::Rgb8),
        Some("DeviceCMYK") => (cmyk_to_rgb(&content)?, ColorType::Rgb8),
        Some(color_space) => {
            return Err(format!(
                "unsupported PDF image color space {color_space}; expected DeviceGray, DeviceRGB, or DeviceCMYK"
            ))
        }
        None => return Err("PDF image is missing color space metadata".to_string()),
    };

    validate_pixel_len(&pixels, width, height, color_type)?;
    Ok((
        encode_png(&pixels, width, height, color_type)?,
        PDF_IMAGE_DESCRIPTION_MIME_TYPE,
    ))
}

fn plain_image_content(image: &PdfImage<'_>) -> Result<Vec<u8>, String> {
    let stream = Stream::new(image.origin_dict.clone(), image.content.to_vec());
    stream
        .get_plain_content()
        .map_err(|err| format!("failed to decode PDF image stream: {err}"))
}

fn has_image_filter(image: &PdfImage<'_>, filter_name: &str) -> bool {
    image
        .filters
        .as_ref()
        .map(|filters| filters.iter().any(|filter| filter == filter_name))
        .unwrap_or(false)
}

fn image_dimension(name: &str, value: i64) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| format!("invalid PDF image {name}: {value}"))
}

fn validate_pixel_len(
    pixels: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
) -> Result<(), String> {
    let channels = match color_type {
        ColorType::L8 => 1usize,
        ColorType::Rgb8 => 3usize,
        _ => return Err(format!("unsupported PNG color type {color_type:?}")),
    };
    let expected_len = width as usize * height as usize * channels;

    if pixels.len() == expected_len {
        Ok(())
    } else {
        Err(format!(
            "decoded PDF image had {} bytes; expected {expected_len} for {width}x{height} {color_type:?}",
            pixels.len()
        ))
    }
}

fn cmyk_to_rgb(cmyk: &[u8]) -> Result<Vec<u8>, String> {
    if cmyk.len() % 4 != 0 {
        return Err(format!(
            "invalid CMYK image data length {}; expected a multiple of 4",
            cmyk.len()
        ));
    }

    Ok(cmyk
        .chunks_exact(4)
        .flat_map(|pixel| {
            let cyan = pixel[0] as u16;
            let magenta = pixel[1] as u16;
            let yellow = pixel[2] as u16;
            let black = pixel[3] as u16;
            [
                255u8.saturating_sub((cyan + black).min(255) as u8),
                255u8.saturating_sub((magenta + black).min(255) as u8),
                255u8.saturating_sub((yellow + black).min(255) as u8),
            ]
        })
        .collect())
}

fn encode_png(
    pixels: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
) -> Result<Vec<u8>, String> {
    let mut png = Vec::new();
    let encoder = PngEncoder::new(&mut png);
    encoder
        .write_image(pixels, width, height, color_type)
        .map_err(|err| format!("failed to encode PDF image as PNG: {err}"))?;
    Ok(png)
}

fn ensure_supported_pdf_file(path: &Path) -> Result<(), String> {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("pdf") => Ok(()),
        Some(extension) => Err(format!(
            "unsupported PDF extension .{extension}; expected pdf"
        )),
        None => Err(format!(
            "could not infer PDF type for {}; expected .pdf",
            path.display()
        )),
    }
}

fn clean_pdf_page_text(page: &str) -> String {
    let normalized = page.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = Vec::new();
    let mut previous_line_was_blank = false;

    for raw_line in normalized.lines() {
        let line = raw_line.trim_end();
        let line_is_blank = line.trim().is_empty();

        if line_is_blank {
            if !previous_line_was_blank && !lines.is_empty() {
                lines.push(String::new());
            }
        } else {
            lines.push(line.to_string());
        }

        previous_line_was_blank = line_is_blank;
    }

    lines.join("\n").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_page_text_without_destroying_line_order() {
        let text = "Title  \r\n\r\n\r\n  Indented line  \nNext line\n\n";

        assert_eq!(
            clean_pdf_page_text(text),
            "Title\n\n  Indented line\nNext line"
        );
    }

    #[test]
    fn converts_cmyk_pixels_to_rgb() {
        let cmyk = [0, 255, 255, 0, 255, 0, 255, 0];

        assert_eq!(cmyk_to_rgb(&cmyk).unwrap(), vec![255, 0, 0, 0, 255, 0]);
    }

    #[test]
    fn encodes_rgb_pixels_as_png() {
        let rgb = [255, 0, 0, 0, 255, 0];

        let png = encode_png(&rgb, 2, 1, ColorType::Rgb8).unwrap();

        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    }
}
