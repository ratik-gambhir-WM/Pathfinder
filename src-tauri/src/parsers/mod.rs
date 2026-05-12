pub mod docx;
pub mod image;
pub mod spreadsheet;

use std::collections::HashSet;
use std::fs::DirEntry;
use base64::Engine;
use walkdir::WalkDir;
use crate::models::document::ParsedFile;
use crate::utils::get_token_count;

pub(crate) const MAX_TOKEN_CHUNK: usize = 800;

#[allow(dead_code)]
pub(crate) struct TextChunk {
    chunk_index: i32,
    content: String,
}

fn build_text_chunk(content: &str, chunk_index: i32) -> TextChunk {
    TextChunk {
        chunk_index,
        content: content.to_string(),
    }
}

#[allow(dead_code)]
fn build_parsed_file(
    _file_id: String,
    _file_name: String,
    _file_path: String,
    _file_type: String,
    _file_hash: String,
    _file_size_bytes: i64,
    _ingested_at: String,
    _total_tokens: i64,
    _total_chunks: i64,
) -> ParsedFile {
    ParsedFile {
        file_id: "".to_string(),
        file_name: "".to_string(),
        file_path: "".to_string(),
        file_type: "".to_string(),
        file_hash: "".to_string(),
        file_size_bytes: 0,
        ingested_at: "".to_string(),
        total_tokens: 0,
        total_chunks: 0,
    }
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

async fn walk(path: Option<&String>) -> Result<(), String> {

    for file in WalkDir::new(path.unwrap())
        .into_iter()
        .filter_map(Result::ok)
    {
        let mut succeeded_files: HashSet<String> = HashSet::new();
        let mut failed_files: HashSet<String> = HashSet::new();
        let file_path = file.path();
        let file_name = file_path.display();
        if !file_path.is_file() {
            continue;
        }
        let file_extension = file_path.extension().unwrap().to_str();

    }
    Ok(())
}



fn parse_file(file: DirEntry) -> Result<(), String> {
    let file_path = file.path();
    let file_extension = file_path.extension().unwrap().to_str();
    match file_extension {
        Some("pdf") | Some("docx") | Some("xlsx") | Some("xls") => {
            let bytes = std::fs::read(file_path).unwrap_or_else(|e| {
                return vec![]
            });
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);

            // send encoded file data to OpenAI
        }

        Some("txt") | Some("md") | Some("csv") => {
            let text = std::fs::read_to_string(file_path).unwrap();

            // send text to OpenAI
        }

        _ => {
            println!("Unsupported file type: {:?}", file_extension);
        }
    }
    Ok(())
}

