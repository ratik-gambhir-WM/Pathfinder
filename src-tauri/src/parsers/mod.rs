pub mod docx;
pub mod image;
pub mod spreadsheet;

use crate::clients::helix::HelixClient;
use crate::models::document::{FileChunk, ParsedFileData, ParsedFileData2};
use crate::parsers::docx::{parse_docx_file};
use crate::utils::get_token_count;
use base64::Engine;


use std::path::Path;
use std::{env, fs, process};
use walkdir::{WalkDir, DirEntry};

pub(crate) const MAX_TOKEN_CHUNK: usize = 800;

pub struct TextChunk {
    chunk_index: i32,
    content: String,
}

fn build_text_chunk(content: &str, chunk_index: i32) -> TextChunk {
    TextChunk {
        chunk_index,
        content: content.to_string(),
    }
}


fn get_file_type(path: Box<Path>) -> String {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase());

    extension.unwrap_or_else(|| {
        String::new()
    })

}


fn gen_file_metadata(path: &Path, chunks: Vec<TextChunk>) -> ParsedFileData2 {
    let file_type = get_file_type(Box::from(path));
    let metadata =
        fs::metadata(&path).map_err(|err| format!("failed to read {}: {err}", path.display()));
    let file_byte_size = metadata.unwrap().len();
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("failed to derive filename from {}", path.display())).unwrap();
    ParsedFileData2 {
        file_id: "".to_string(),
        file_name: filename.to_string(),
        file_path: path.display().to_string(),
        file_type: file_type.to_string(),
        file_hash: "".to_string(),
        file_size_bytes: file_byte_size as i64,
        ingested_at: "".to_string(),
        total_tokens: 0,
        total_chunks: 0,
        file_chunks: chunks,
    }
}

#[allow(dead_code)]
pub fn gen_parsed_file(
    file: DirEntry,
    to_chunk: Option<bool>
) -> ParsedFileData2 {
    let path = file.path();
    let mut chunks: Vec<TextChunk> = Vec::new();
    if matches!(to_chunk, Some(true)) {
        let parsed = parse_docx_file(&path);
     chunks = parsed.unwrap_or_else(|_| Vec::new())
    }
    println!("{}", path.display());
    gen_file_metadata(path, chunks)
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
            let bytes = std::fs::read(path).unwrap_or_else(|e| return vec![]);
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
