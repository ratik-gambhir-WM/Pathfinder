use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNode {
    pub document_id: String,
    pub user_id: String,
    pub file_name: String,
    pub source_type: String,
    pub rendered_pdf_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNode {
    pub page_id: String,
    pub document_id: String,
    pub page_number: u32,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkNode {
    pub chunk_id: String,
    pub document_id: String,
    pub user_id: String,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub sequence_number: u32,
    pub page_numbers: Vec<u32>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub token_count: u32,
    pub content_hash: String,
    pub section_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHasPage {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHasChunk {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkNext {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAppearsOnPage {
    pub user_id: String,
    pub chunk_start_offset: usize,
    pub chunk_end_offset: usize,
    pub page_start_offset: Option<usize>,
    pub page_end_offset: Option<usize>,
}
