#![allow(dead_code)]

use docx_rust::{DocxError, DocxFile};
use std::path::Path;
use docx_rust::document::{BodyContent, Table};
use crate::models::document::FileChunk;
use crate::parsers::{chunk_text, TextChunk, MAX_TOKEN_CHUNK};
use crate::utils::get_token_count;

pub fn parse_docx_file(path: &Path) -> Result<Vec<TextChunk>, String> {
    println!("parse_docx_file");
    let file = DocxFile::from_file(path).map_err(|err| err);
    match file {
        Ok(file) => {
            println!("parse_docx_file - ok");
            chunk_docx_file(&file)
        }
        Err(err) => {
            println!("parse_docx_file - error {}", err.to_string());
            Err(err.to_string())
        }
    }
}

fn gen_docx_chunk(content: &str, chunk_index: i32) -> FileChunk {
    FileChunk {
        chunk_id: "".to_string(),
        text: content.to_string(),
        text_hash: "".to_string(),
        chunk_index: chunk_index as i64,
        token_count: 0,
        page_start: 0,
        page_end: 0,
        embedded_at: None,
    }
}

fn extract_table_text(table: &Table) -> String {
    String::new()
}

fn chunk_docx_file(file: &DocxFile) -> Result<Vec<TextChunk>, String> {
    println!("chunk_docx_file");

    let docx = file.parse().map_err(|err| err.to_string())?;
    let full_text = &docx.document.body.text();
    
    if get_token_count(full_text) > MAX_TOKEN_CHUNK {
        Ok(chunk_text(full_text))
    } else {
        // Return text chunk here
        Err("Not enough tokens".to_string())
    }
}
