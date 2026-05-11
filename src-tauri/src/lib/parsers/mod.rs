pub mod image_parser;
pub mod docx_parser;
mod spreadsheet_parser;

use serde::{Deserialize, Serialize};
use crate::utils::get_token_count;

const MAX_TOKEN_CHUNK: usize = 800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFile {
    pub file_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub file_hash: String,
    pub file_size_bytes: i64,
    pub ingested_at: String,
    pub total_tokens: i64,
    pub total_chunks: i64,
    pub chunks: Vec<DocxChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocxChunk {
    pub chunk_id: String,
    pub text: String,
    pub text_hash: String,
    pub chunk_index: i64,
    pub token_count: i64,
    pub page_start: i64,
    pub page_end: i64,
    pub embedded_at: Option<String>,
}

struct TextChunk {
    chunk_index: i32,
    content: String
}

//
// async fn walk(path: Option<&String>) -> Result<(), String> {
//     let openai_client = reqwest::Client::new();
//     let helix_db: HelixDB = HelixDB::new(Some("http://localhost"), Some(6969), None);
//     for file in WalkDir::new(path.unwrap())
//         .into_iter()
//         .filter_map(Result::ok)
//     {
//         let mut succeeded_files: HashSet<String> = HashSet::new();
//         let mut failed_files: HashSet<String> = HashSet::new();
//         let file_path = file.path();
//         let file_name = file_path.display();
//         if !file_path.is_file() {
//             continue;
//         }
//
//         let Ok(file_content) = fs::read_to_string(file_path) else {
//             println!("Could not read file named: {file_name}");
//             failed_files.insert(file_path.to_string_lossy().to_string());
//             continue;
//         };
//
//         if file_content.len() > MAX_BYTE_CHUNK {
//             println!("BIGGER: {}", file_content.len());
//             let result = chunk(&file_content).await;
//             for chunk in result {
//                 let embedding_result =
//                     gen_document_embeddings(&chunk.content, &helix_db, &openai_client).await;
//                 match embedding_result {
//                     Ok(_embedding_result) => {
//                         // add embeddings to Vec<DocumentEmbeddings>
//                         continue;
//                     }
//                     Err(error) => {
//                         return Err(error.to_string());
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }
//
fn build_text_chunk(content: &str, chunk_index: i32) -> TextChunk {
    TextChunk {
        chunk_index,
        content: content.to_string(),
    }
}
//
async fn chunk(file_content: &str) -> Vec<TextChunk> {
    let mut chunk_vec: Vec<TextChunk> = Vec::new();
    let token_count = get_token_count(Option::from(file_content));
    let chunk_limit = token_count.div_ceil(MAX_TOKEN_CHUNK) as i32;
    let mut remaining_chunks_str = file_content;
    println!("limit: {}", chunk_limit.to_string(),);
    let mut current = String::new();
    let mut chunk_index: i32 = 1;
    let mut offset: usize = 0;
    while chunk_index < chunk_limit {
        if(offset < 1) {

        }
        let (chunk, remaining_chunks) = &file_content.split_at(offset + MAX_TOKEN_CHUNK);
        remaining_chunks_str = remaining_chunks;
        println!("chunk: {}", chunk,);
        println!("CHUNK INDEX: {}", chunk_index.to_string(),);
        chunk_index += 1;

        if (chunk_index == chunk_limit) {
            println!("remaiining chunk: {}", remaining_chunks_str,);
           // chunk_vec.push(build_text_chunk(chunk, chunk_index));
        }
        offset += MAX_TOKEN_CHUNK;
        current.push_str(chunk);
        chunk_vec.push(build_text_chunk(chunk, chunk_index));
        println!("current: {}", current,);
    }
    chunk_vec
}
