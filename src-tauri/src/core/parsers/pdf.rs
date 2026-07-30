use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use crate::core::{
    clients::openai::OpenAiClient,
    nodes::document_node::{ChunkNode, DocumentNode},
    parsers::image::describe_image,
    text_chunking::token_bounded_ranges,
};
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use pdf_extract::{xobject::PdfImage, Document, Error as PdfError, Stream};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const PDF_IMAGE_DESCRIPTION_MIME_TYPE: &str = "image/png";
const JPEG_IMAGE_DESCRIPTION_MIME_TYPE: &str = "image/jpeg";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfPage {
    pub page_number: u32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocumentAssembly {
    pub document: DocumentNode,
    pub chunks: Vec<ChunkNode>,
}

#[derive(Debug, Clone, Copy)]
struct PdfPageTextRange {
    page_number: u32,
    start_offset: usize,
    end_offset: usize,
}

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

pub fn extract_pdf_text_from_bytes(bytes: &[u8]) -> Result<Vec<PdfPage>, String> {
    let pages = pdf_extract::extract_text_from_mem_by_pages(bytes)
        .map_err(|err| format!("failed to extract text from PDF bytes: {err}"))?;

    Ok(pdf_pages_from_texts(&pages))
}

pub fn extract_pdf_pages(path: &Path) -> Result<Vec<PdfPage>, String> {
    ensure_supported_pdf_file(path)?;

    let pages = pdf_extract::extract_text_by_pages(path).map_err(|err| {
        format!(
            "failed to extract text from PDF file {}: {err}",
            path.display()
        )
    })?;
    Ok(pdf_pages_from_texts(&pages))
}

/// Parses a PDF into a graph-ready document and flat chunks.
///
/// Chunk offsets are exclusive UTF-8 byte offsets into the document-wide text
/// formed by joining non-empty page text with two newlines. Chunk sequence
/// numbers are one-based across the document, and each chunk records every PDF
/// page whose text it overlaps.
pub fn parse_pdf_document(
    path: &Path,
    user_id: impl Into<String>,
) -> Result<PdfDocumentAssembly, String> {
    let pages = extract_pdf_pages(path)?;
    let source_path = path.canonicalize().unwrap_or_else(|_| PathBuf::from(path));
    let file_size_bytes = fs::metadata(&source_path)
        .map_err(|err| {
            format!(
                "failed to read PDF metadata for {}: {err}",
                source_path.display()
            )
        })?
        .len();
    let content_hash = sha256_file(&source_path)?;

    Ok(parse_pdf_file_with_metadata(
        Some(&source_path),
        &user_id.into(),
        &pages,
        file_size_bytes,
        content_hash,
    ))
}

/// Parses and chunks raw PDF bytes into the same graph-ready assembly as the
/// path-based parser. An optional path provides the document's filename and
/// stable local-path identity; when omitted, `local_path` remains `None`.
pub fn parse_pdf_by_bytes(
    bytes: Vec<u8>,
    path: Option<&Path>,
    user_id: impl Into<String>,
) -> Result<PdfDocumentAssembly, String> {
    let file_size_bytes = u64::try_from(bytes.len())
        .map_err(|_| format!("PDF byte length `{}` does not fit in u64", bytes.len()))?;
    let content_hash = sha256_bytes(&bytes);
    let pages = extract_pdf_text_from_bytes(&bytes)?;
    let source_path = path.map(|path| path.canonicalize().unwrap_or_else(|_| PathBuf::from(path)));

    Ok(parse_pdf_file_with_metadata(
        source_path.as_deref(),
        &user_id.into(),
        &pages,
        file_size_bytes,
        content_hash,
    ))
}

pub fn parse_pdf_file(path: &Path) -> Result<String, String> {
    extract_pdf_text(path)
}

pub fn parse_pdf_from_bytes(bytes: &[u8]) -> Result<String, String> {
    extract_pdf_text_from_bytes(bytes).map(|pages| {
        pages
            .into_iter()
            .map(|page| page.text)
            .collect::<Vec<_>>()
            .join("\n\n")
    })
}

fn parse_pdf_file_with_metadata(
    path: Option<&Path>,
    user_id: &str,
    pages: &[PdfPage],
    file_size_bytes: u64,
    content_hash: String,
) -> PdfDocumentAssembly {
    let local_path = path.map(|path| path.to_string_lossy().into_owned());
    let document_identity = local_path.as_deref().unwrap_or(&content_hash);
    let document_id = deterministic_id(&format!("{user_id}\0pdf\0{document_identity}"));
    let file_name = path
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .unwrap_or("Document.pdf")
        .to_string();
    let (document_text, page_ranges) = document_text_with_page_ranges(pages);
    let chunks: Vec<ChunkNode> = token_bounded_ranges(&document_text)
        .into_iter()
        .enumerate()
        .map(|(sequence_index, range)| {
            let sequence_number =
                u32::try_from(sequence_index + 1).expect("PDF chunk count should fit in u32");
            let start_offset = range.start_offset;
            let end_offset = range.end_offset;
            let text = &document_text[start_offset..end_offset];
            let content_hash = deterministic_id(text);
            let chunk_id = deterministic_id(&format!(
                "{user_id}\0{document_id}\0{sequence_number}\0{content_hash}"
            ));
            let page_numbers = overlapping_page_numbers(start_offset, end_offset, &page_ranges);

            ChunkNode {
                chunk_id,
                document_id: document_id.clone(),
                user_id: user_id.to_string(),
                text: text.to_string(),
                embedding: None,
                sequence_number,
                page_numbers: Some(page_numbers),
                start_offset,
                end_offset,
                token_count: u32::try_from(range.token_count)
                    .expect("PDF chunk token count should fit in u32"),
                content_hash,
                section_title: None,
            }
        })
        .collect();
    let token_count = chunks
        .iter()
        .map(|chunk| u64::from(chunk.token_count))
        .sum();
    let document = DocumentNode {
        document_id: document_id.clone(),
        user_id: user_id.to_string(),
        file_name,
        source_type: "pdf".to_string(),
        local_path: local_path.clone(),
        file_size_bytes,
        token_count,
        content_hash,
        rendered_pdf_path: local_path,
    };

    PdfDocumentAssembly { document, chunks }
}

#[cfg(test)]
fn assemble_pdf_pages(path: &Path, user_id: &str, pages: &[PdfPage]) -> PdfDocumentAssembly {
    let (document_text, _) = document_text_with_page_ranges(pages);

    parse_pdf_file_with_metadata(
        Some(path),
        user_id,
        pages,
        u64::try_from(document_text.len()).unwrap(),
        deterministic_id(&document_text),
    )
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path)
        .map_err(|err| format!("failed to open PDF for hashing {}: {err}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|err| format!("failed to hash PDF {}: {err}", path.display()))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn document_text_with_page_ranges(pages: &[PdfPage]) -> (String, Vec<PdfPageTextRange>) {
    let mut document_text = String::new();
    let mut page_ranges = Vec::new();

    for page in pages.iter().filter(|page| !page.text.trim().is_empty()) {
        if !document_text.is_empty() {
            document_text.push_str("\n\n");
        }

        let start_offset = document_text.len();
        document_text.push_str(&page.text);
        page_ranges.push(PdfPageTextRange {
            page_number: page.page_number,
            start_offset,
            end_offset: document_text.len(),
        });
    }

    (document_text, page_ranges)
}

fn overlapping_page_numbers(
    chunk_start_offset: usize,
    chunk_end_offset: usize,
    page_ranges: &[PdfPageTextRange],
) -> Vec<u32> {
    let mut page_numbers = Vec::new();

    for page in page_ranges {
        if chunk_start_offset < page.end_offset && chunk_end_offset > page.start_offset {
            if !page_numbers.contains(&page.page_number) {
                page_numbers.push(page.page_number);
            }
        }
    }

    page_numbers
}

fn deterministic_id(value: &str) -> String {
    Sha256::digest(value.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn pdf_pages_from_texts(page_texts: &[String]) -> Vec<PdfPage> {
    page_texts
        .iter()
        .enumerate()
        .map(|(page_index, text)| PdfPage {
            page_number: u32::try_from(page_index + 1).expect("PDF page count should fit in u32"),
            text: clean_pdf_page_text(text),
        })
        .collect()
}

pub async fn extract_pdf_image_descriptions(
    path: &Path,
    openai_client: &OpenAiClient,
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
    use std::{
        env, process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;
    use crate::core::text_chunking::MAX_TOKEN_CHUNK;
    use pdf_extract::{
        content::{Content, Operation},
        dictionary, Object,
    };

    fn pdf_bytes_with_text(text: &str) -> Vec<u8> {
        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = document.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 12.into()]),
                Operation::new("Td", vec![100.into(), 700.into()]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id =
            document.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
        let page_id = document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });
        document.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
                "Resources" => resources_id,
                "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            }),
        );
        let catalog_id = document.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        document.trailer.set("Root", catalog_id);

        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn parses_pdf_bytes_into_document_and_chunk_nodes_without_a_local_path() {
        let bytes = pdf_bytes_with_text("Quarterly quarry report.");
        let expected_size = u64::try_from(bytes.len()).unwrap();
        let expected_hash = sha256_bytes(&bytes);

        let assembly = parse_pdf_by_bytes(bytes, None, "user-1").unwrap();

        assert_eq!(assembly.document.user_id, "user-1");
        assert_eq!(assembly.document.file_name, "Document.pdf");
        assert_eq!(assembly.document.source_type, "pdf");
        assert_eq!(assembly.document.local_path, None);
        assert_eq!(assembly.document.rendered_pdf_path, None);
        assert_eq!(assembly.document.file_size_bytes, expected_size);
        assert_eq!(assembly.document.content_hash, expected_hash);
        assert_eq!(assembly.chunks.len(), 1);
        assert_eq!(
            assembly.chunks[0].document_id,
            assembly.document.document_id
        );
        assert_eq!(assembly.chunks[0].text, "Quarterly quarry report.");
        assert_eq!(assembly.chunks[0].page_numbers, Some(vec![1]));
    }

    #[test]
    fn assembles_ordered_document_chunks_with_global_offsets() {
        let first_page_text = format!("{}🙂", "First page content. ".repeat(MAX_TOKEN_CHUNK));
        let pdf_pages = vec![
            PdfPage {
                page_number: 1,
                text: first_page_text.clone(),
            },
            PdfPage {
                page_number: 2,
                text: "Second page body.".to_string(),
            },
        ];

        let assembly = assemble_pdf_pages(Path::new("/documents/report.pdf"), "user-1", &pdf_pages);

        assert_eq!(assembly.document.user_id, "user-1");
        assert_eq!(assembly.document.file_name, "report.pdf");
        assert_eq!(assembly.document.source_type, "pdf");
        assert_eq!(
            assembly.document.local_path.as_deref(),
            Some("/documents/report.pdf")
        );
        assert!(assembly.chunks.len() > 1);
        let expected_text = format!("{first_page_text}\n\nSecond page body.");
        assert_eq!(
            assembly.document.file_size_bytes,
            u64::try_from(expected_text.len()).unwrap()
        );
        assert_eq!(
            assembly.document.token_count,
            assembly
                .chunks
                .iter()
                .map(|chunk| u64::from(chunk.token_count))
                .sum::<u64>()
        );
        assert_eq!(
            assembly.document.content_hash,
            deterministic_id(&expected_text)
        );
        assert_eq!(
            assembly
                .chunks
                .iter()
                .map(|chunk| chunk.text.as_str())
                .collect::<String>(),
            expected_text
        );

        for (chunk_index, chunk) in assembly.chunks.iter().enumerate() {
            assert_eq!(chunk.document_id, assembly.document.document_id);
            assert_eq!(chunk.user_id, "user-1");
            assert_eq!(
                chunk.sequence_number,
                u32::try_from(chunk_index + 1).unwrap()
            );
            assert_eq!(
                &expected_text[chunk.start_offset..chunk.end_offset],
                chunk.text
            );
            assert!(chunk.token_count as usize <= MAX_TOKEN_CHUNK);
            assert!(chunk
                .page_numbers
                .as_ref()
                .is_some_and(|page_numbers| !page_numbers.is_empty()));
            assert_eq!(
                chunk.start_offset,
                assembly
                    .chunks
                    .get(chunk_index.wrapping_sub(1))
                    .map(|previous| previous.end_offset)
                    .unwrap_or(0)
            );
        }
        assert_eq!(
            assembly.chunks.last().unwrap().end_offset,
            expected_text.len()
        );
    }

    #[test]
    fn a_chunk_crossing_pages_records_each_page_number() {
        let pdf_pages = vec![
            PdfPage {
                page_number: 4,
                text: "End of page four.".to_string(),
            },
            PdfPage {
                page_number: 5,
                text: "Start of page five.".to_string(),
            },
        ];

        let assembly = assemble_pdf_pages(Path::new("/documents/report.pdf"), "user-1", &pdf_pages);

        assert_eq!(assembly.chunks.len(), 1);
        assert_eq!(
            assembly.chunks[0].text,
            "End of page four.\n\nStart of page five."
        );
        assert_eq!(assembly.chunks[0].page_numbers, Some(vec![4, 5]));
        assert_eq!(assembly.chunks[0].start_offset, 0);
        assert_eq!(assembly.chunks[0].end_offset, assembly.chunks[0].text.len());
    }

    #[test]
    fn assembly_ids_are_stable_and_embeddings_serialize_as_null() {
        let pdf_pages = vec![PdfPage {
            page_number: 1,
            text: "A short PDF page.".to_string(),
        }];

        let first = assemble_pdf_pages(Path::new("/documents/report.pdf"), "user-1", &pdf_pages);
        let second = assemble_pdf_pages(Path::new("/documents/report.pdf"), "user-1", &pdf_pages);
        let chunk = &first.chunks[0];
        let json = serde_json::to_value(chunk).unwrap();

        assert_eq!(first.document.document_id, second.document.document_id);
        assert_eq!(chunk.chunk_id, second.chunks[0].chunk_id);
        assert_eq!(chunk.content_hash.len(), 64);
        assert_eq!(chunk.page_numbers, Some(vec![1]));
        assert_eq!(json["page_numbers"], serde_json::json!([1]));
        assert!(json["embedding"].is_null());
    }

    #[test]
    fn hashes_actual_file_bytes_with_sha256() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "quarry-pdf-hash-test-{}-{unique}.pdf",
            process::id()
        ));
        fs::write(&path, b"mock pdf bytes").unwrap();

        let hash = sha256_file(&path).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(
            hash,
            "58b499442dcfe0024d5b87c73d10cf3f43831f621a443f5c23532c49a8056761"
        );
    }

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

    #[test]
    fn builds_numbered_pages_with_clean_text() {
        let raw_pages = vec![
            "First page  \r\n\r\nBody\n".to_string(),
            "\r\n  \r\n".to_string(),
            "Third page\n\n\nDetail  ".to_string(),
        ];

        let pages = pdf_pages_from_texts(&raw_pages);

        assert_eq!(
            pages,
            vec![
                PdfPage {
                    page_number: 1,
                    text: "First page\n\nBody".to_string(),
                },
                PdfPage {
                    page_number: 2,
                    text: String::new(),
                },
                PdfPage {
                    page_number: 3,
                    text: "Third page\n\nDetail".to_string(),
                },
            ]
        );
    }
}
