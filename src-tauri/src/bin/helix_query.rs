use std::{env, process};

use quarry_lib::core::clients::helix::{
    add_file_chunk, file_chunk_by_chunk_id, file_chunks_by_file_id, AddFileChunkInput, HelixClient,
};
use quarry_lib::core::clients::openai::OpenAiClient;
use serde_json::Value;

const APP_NAME: &str = "DataRoomCLI";

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let args: Vec<String> = env::args().collect();

    if let Err(message) = run(args).await {
        eprintln!("error: {message}");
        eprintln!("run `dataroomcli help` for usage");
        process::exit(1);
    }
}

async fn run(args: Vec<String>) -> Result<(), String> {
    let helix_db: HelixClient = HelixClient::new()?;

    match args.get(1).map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => {
            print_help();
            Ok(())
        }
        Some("-V") | Some("--version") | Some("version") => {
            println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("add-file-chunk") => match args.get(2) {
            Some(payload) => {
                let input = parse_add_file_chunk_payload(payload)?;
                let result: Value = helix_db
                    .execute_dynamic_query(move || {
                        add_file_chunk(
                            input.chunk_id,
                            input.file_id,
                            input.file_name,
                            input.file_path,
                            input.text,
                            input.text_hash,
                            input.chunk_index,
                            input.token_count,
                            input.page_start,
                            input.page_end,
                            input.embedded_at.unwrap_or_default(),
                        )
                    })
                    .await?;
                print_json(&result)
            }
            None => Err("missing file chunk JSON payload".to_string()),
        },
        Some("file-chunk") => match args.get(2) {
            Some(chunk_id) => {
                let chunk_id = chunk_id.to_string();
                let result: Value = helix_db
                    .execute_dynamic_query(move || file_chunk_by_chunk_id(chunk_id))
                    .await?;
                print_json(&result)
            }
            None => Err("missing chunk_id".to_string()),
        },
        Some("file-chunks") => match args.get(2) {
            Some(file_id) => {
                let file_id = file_id.to_string();
                let result: Value = helix_db
                    .execute_dynamic_query(move || file_chunks_by_file_id(file_id))
                    .await?;
                print_json(&result)
            }
            None => Err("missing file_id".to_string()),
        },
        Some("embed") => match args.get(2) {
            Some(content) => {
                let client = OpenAiClient::new()?;
                match client.gen_embedding(content, None).await {
                    Ok(embedding) => match serde_json::to_string(&embedding) {
                        Ok(json) => {
                            println!("{json}");
                            Ok(())
                        }
                        Err(err) => Err(format!("failed to serialize embedding: {err}")),
                    },
                    Err(err) => Err(err),
                }
            }
            None => Err("missing content to embed".to_string()),
        },
        Some("response") => {
            let client = OpenAiClient::new()?;
            client
                .gen_model_response(
                    Option::from("What is capital of Ohio and how was it founded?"),
                    None,
                    None,
                )
                .await
                .map(|_t| {})
        }

        Some(command) => Err(format!("unknown command: {command}")),
    }
}

fn parse_add_file_chunk_payload(payload: &str) -> Result<AddFileChunkInput, String> {
    serde_json::from_str(payload).map_err(|err| format!("failed to parse file chunk JSON: {err}"))
}

fn print_json(value: &Value) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(value)
        .map_err(|err| format!("failed to serialize Helix response: {err}"))?;
    println!("{pretty}");
    Ok(())
}

fn print_help() {
    println!(
        "{APP_NAME}

Usage:
  dataroomcli <command> [options]

Commands:
  add-file-chunk <json_payload>        Insert a FileChunk node into Helix
  file-chunk <chunk_id>                Fetch a FileChunk by chunk_id
  file-chunks <file_id>                Fetch FileChunk nodes for a file_id
  embed <content>                      Generate an embedding and print it as JSON
  help            Show this help message
  version         Show the current version
"
    );
}
