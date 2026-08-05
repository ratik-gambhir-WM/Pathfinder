pub mod docx;
pub mod image;
pub mod pdf;
// pub mod powerpoint;
// pub mod spreadsheet;

use crate::core::parsers::docx::{parse_docx_chunks_from_bytes, DocxAssembly};
use crate::core::parsers::pdf::{parse_pdf_by_bytes, PdfDocumentAssembly};
// use crate::core::clients::openai::OpenAiClient;
// use crate::core::parsers::image::parse_image_file;
// use crate::core::parsers::powerpoint::parse_powerpoint_file;
// use crate::core::parsers::spreadsheet::parse_spreadsheet;
use crate::core::text_chunking::MAX_TOKEN_CHUNK;
use crate::utils::get_token_count;
use base64::Engine;

use std::fs::{self, File, Metadata};
use std::path::{Path, PathBuf};

pub struct TextChunk {
    chunk_index: i32,
    content: String,
}

impl TextChunk {
    pub fn new(chunk_index: i32, content: String) -> Self {
        Self {
            chunk_index,
            content,
        }
    }
}

#[derive(Debug)]
pub enum QuarryFile {
    Pdf { bytes: Vec<u8>, path: PathBuf },
    Docx { bytes: Vec<u8>, path: PathBuf },
    // Powerpoint { bytes: Vec<u8>, path: PathBuf },
    // Image { bytes: Vec<u8>, path: PathBuf },
    // Spreadsheet { bytes: Vec<u8>, path: PathBuf },
}

#[derive(Debug)]
pub enum ParsedQuarryFile {
    Pdf(PdfDocumentAssembly),
    Docx(DocxAssembly),
}

/// Reads filesystem metadata from an already-open file without consuming it.
pub fn generate_file_metadata(file: &File) -> Result<Metadata, String> {
    file.metadata()
        .map_err(|err| format!("failed to read file metadata: {err}"))
}

impl QuarryFile {
    pub fn from_local_path(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase())
            .ok_or_else(|| "invalid file format".to_owned())?;
        if !matches!(extension.as_str(), "pdf" | "docx") {
            return Err("invalid file format".to_owned());
        }

        let bytes =
            fs::read(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;

        match extension.as_str() {
            "pdf" => Ok(Self::Pdf { bytes, path }),
            "docx" => Ok(Self::Docx { bytes, path }),
            // "pptx" | "ppt" => Ok(Self::Powerpoint { bytes, path }),
            // "gif" | "jpg" | "jpeg" | "png" | "webp" => Ok(Self::Image { bytes, path }),
            //"xlsx" => Ok(Self::Spreadsheet { bytes, path }),
            _ => unreachable!("supported extensions were validated before opening the file"),
        }
    }

    pub async fn parse(self) -> Result<ParsedQuarryFile, String> {
        match self {
            // Self::Powerpoint { path, .. } => parse_powerpoint_file(&path),
            // Self::Image { path, .. } => {
            //     let openai_client = OpenAiClient::new()?;
            //     parse_image_file(&path, &openai_client).await
            // }
            // Self::Spreadsheet { path, .. } => {
            //     parse_spreadsheet(&path).map_err(|err| err.to_string())
            // }
            Self::Pdf { bytes, path } => {
                parse_pdf_by_bytes(bytes, Some(&path), "").map(ParsedQuarryFile::Pdf)
            }
            Self::Docx { bytes, path } => {
                parse_docx_chunks_from_bytes(bytes, Some(&path), "").map(ParsedQuarryFile::Docx)
            }
        }
    }
}

fn build_text_chunk(content: &str, chunk_index: i32) -> TextChunk {
    TextChunk::new(chunk_index, content.to_string())
}

pub(crate) fn chunk_text(text_content: &str) -> Vec<TextChunk> {
    let token_count = get_token_count(text_content);
    println!("Token count: {}", token_count);
    println!("Max tokens: {}", MAX_TOKEN_CHUNK);
    let chunk_limit = token_count.div_ceil(MAX_TOKEN_CHUNK);
    println!("Chunk limit: {}", chunk_limit);
    split_by_chunk_limit(text_content, chunk_limit.max(1))
        .into_iter()
        .enumerate()
        .map(|(index, chunk)| build_text_chunk(&chunk, index as i32 + 1))
        .collect()
}

fn split_by_chunk_limit(content: &str, chunk_limit: usize) -> Vec<String> {
    let chars = content.chars().collect::<Vec<char>>();
    let chunk_size = (chars.len() + chunk_limit - 1) / chunk_limit;
    chars
        .chunks(chunk_size.max(1))
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn read_and_encode_file(path: Box<Path>) -> Result<String, String> {
    let file_extension = path.extension().unwrap().to_str();
    match file_extension {
        Some("pdf") | Some("docx") | Some("xlsx") | Some("xls") => {
            let bytes = std::fs::read(path).unwrap_or_else(|_e| return vec![]);
            if bytes.is_empty() {
                return Err(String::from("Could not read file"));
            }
            let encode_string = base64::engine::general_purpose::STANDARD.encode(bytes);
            Ok(encode_string)
        }
        Some("txt") | Some("md") | Some("csv") => {
            let file_str = std::fs::read_to_string(path).unwrap();
            Ok(file_str)
        }
        _ => {
            println!("Unsupported file type: {:?}", file_extension);
            Err(String::from("Unsupported file type"))
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/core/parsers/mod_tests.rs"]
mod tests;
