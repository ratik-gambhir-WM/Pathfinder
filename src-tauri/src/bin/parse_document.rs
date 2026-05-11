use chrono::Utc;
use helix_rs::{HelixDB, HelixDBClient, HelixError};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;
use docx_rust::DocxFile;
use walkdir::WalkDir;
use path_finder_lib::parsers::image_parser::describe_and_embed_image_file;
use path_finder_lib::parsers::docx_parser::parse_docx_file;



const APP_NAME: &str = "DataRoomCLI";

const CHUNK_TOKENS: usize = 1000; // good default
const MAX_BYTE_CHUNK: usize = CHUNK_TOKENS * 4;
const CHUNK_OVERLAP: usize = 100; // preserve context

struct TextChunk {
    chunk_index: i32,
    content: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let args: Vec<String> = env::args().collect();
    // let helix_db: HelixDB = HelixDB::new(Some("http://localhost"), Some(6969), None);
    // let openai_client = reqwest::Client::new();

    let result = match args.get(1).map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => {
            print_help();
            Ok(())
        }
        Some("-V") | Some("--version") | Some("version") => {
            println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        // Some("ingest") => ingest_data_room(args.get(2), &helix_db, &openai_client).await,
        Some("walk") => walk(args.get(2)).await,
        Some("docx") => parse_docx(args.get(2)).await,
        Some(command) => Err(format!("unknown command: {command}")),
    };

    if let Err(message) = result {
        eprintln!("error: {message}");
        eprintln!("run `dataroomcli help` for usage");
        process::exit(1);
    }
}

fn print_help() {
    println!(
        "{APP_NAME}

Usage:
  dataroomcli <command> [options]

Commands:
  ingest [root_folder] [priority_folders]     Start a new data room workspace
  status          Show current data room status
  help            Show this help message
  version         Show the current version
"
    );
}

async fn parse_docx(path: Option<&String>) -> Result<(), String> {
    println!("PARSE DOCX");

    for file in WalkDir::new(path.unwrap())
        .into_iter()
        .filter_map(Result::ok)
    {
        let file_path = file.path();
        let file_name = file_path.display();
        if !file_path.is_file() {
            println!("NOT FILE");
            continue;
        } else {
            parse_docx_file(&file_path).map_err(|err| err.to_string())?;
            break;

        }
    }
    Ok(())
}

async fn walk(path: Option<&String>) -> Result<(), String> {
    let openai_client = reqwest::Client::new();
    let helix_db: HelixDB = HelixDB::new(Some("http://localhost"), Some(6969), None);
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

        let Ok(file_content) = fs::read_to_string(file_path) else {
            println!("Could not read file named: {file_name}");
            failed_files.insert(file_path.to_string_lossy().to_string());
            continue;
        };

        if file_content.len() > MAX_BYTE_CHUNK {
            println!("BIGGER: {}", file_content.len());
            let result = chunk(&file_content).await;
            for chunk in result {
                let embedding_result =
                    gen_document_embeddings(&chunk.content, &helix_db, &openai_client).await;
                match embedding_result {
                    Ok(_embedding_result) => {
                        // add embeddings to Vec<DocumentEmbeddings>
                        continue;
                    }
                    Err(error) => {
                        return Err(error.to_string());
                    }
                }
            }
        }
    }
    Ok(())
}

fn build_text_chunk(content: &str, chunk_index: i32) -> TextChunk {
    TextChunk {
        chunk_index,
        content: content.to_string(),
    }
}

async fn chunk(file_content: &str) -> Vec<TextChunk> {
    let mut chunk_vec: Vec<TextChunk> = Vec::new();
    let chunk_limit = file_content.len().div_ceil(MAX_BYTE_CHUNK) as i32;
    let mut remaining_chunks_str = file_content;
    println!("limit: {}", chunk_limit.to_string(),);
    let mut current = String::new();
    let mut chunk_index: i32 = 1;
    let mut offset: usize = 0;
    while chunk_index < chunk_limit {
        let (chunk, remaining_chunks) = &file_content.split_at(offset + MAX_BYTE_CHUNK);
        remaining_chunks_str = remaining_chunks;
        println!("chunk: {}", chunk,);
        println!("CHUNK INDEX: {}", chunk_index.to_string(),);
        chunk_index += 1;

        if (chunk_index == chunk_limit) {
            println!("remaiining chunk: {}", remaining_chunks_str,);
            chunk_vec.push(build_text_chunk(chunk, chunk_index));
        }
        offset += MAX_BYTE_CHUNK;
        current.push_str(chunk);
        chunk_vec.push(build_text_chunk(chunk, chunk_index));
        println!("current: {}", current,);
    }
    chunk_vec
}

async fn save_embedding(helix_db: &HelixDB, result: &Vec<f64>) -> Result<(), String> {
    let result: Result<(), HelixError> = helix_db.query("CreateEmbedding", &result).await;
    match result {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

async fn embed_document(
    content: &str,
    openai_client: &reqwest::Client,
) -> Result<Vec<f64>, String> {
    if content.trim().is_empty() {
        return Err("cannot embed empty document content".to_string());
    }

    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable is not set".to_string())?;

    let response = openai_client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "text-embedding-3-small",
            "input": content,
            "encoding_format": "float"
        }))
        .send()
        .await
        .map_err(|err| format!("failed to call OpenAI embeddings API: {err}"))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|err| format!("failed to read OpenAI embeddings response: {err}"))?;

    if !status.is_success() {
        return Err(format!(
            "OpenAI embeddings API returned {status}: {response_body}"
        ));
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_body)
        .map_err(|err| format!("failed to parse OpenAI embeddings response: {err}"))?;

    let embedding_values = response_json["data"]
        .get(0)
        .and_then(|item| item["embedding"].as_array())
        .ok_or_else(|| "OpenAI embeddings response did not include an embedding".to_string())?;

    let embedding: Vec<f64> = embedding_values
        .iter()
        .map(|value| {
            value
                .as_f64()
                .ok_or_else(|| "OpenAI embedding contained a non-number value".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let embedded_at = Utc::now().to_rfc3339();
    println!(
        "embedded document at {embedded_at}; vector dimensions: {}",
        embedding.len()
    );

    Ok(embedding)
}

async fn gen_document_embeddings(
    content: &str,
    helix_db: &HelixDB,
    openai_client: &reqwest::Client,
) -> Result<(), String> {
    let result = embed_document(&content, openai_client).await;
    match result {
        Ok(embedding_result) => save_embedding(helix_db, &embedding_result).await,
        Err(err) => Err(err.to_string()),
    }
}
