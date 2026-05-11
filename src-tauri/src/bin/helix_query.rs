use helix_rs::{HelixDB, HelixDBClient};
use serde::Serialize;
use serde_json::Value;
use std::{env, process};

use path_finder_lib::helix_client::{GetUserByEmailInput, InsertUserInput, HelixClient};
use path_finder_lib::openai_client::OpenAiClient;

const APP_NAME: &str = "DataRoomCLI";



#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let args: Vec<String> = env::args().collect();
    let helix_db: HelixClient = HelixClient::new();
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("error: OPENAI_API_KEY environment variable is not set");
            process::exit(1);
        }
    };
    let client = OpenAiClient::new(api_key.as_str());

    let result = match args.get(1).map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => {
            print_help();
            Ok(())
        }
        Some("-V") | Some("--version") | Some("version") => {
            println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("add-user") => helix_db.add_user(args.get(2), args.get(3), args.get(4)).await,
        Some("get-user") => helix_db.get_user(args.get(2)).await,
        Some("embed") => client.gen_document_embeddings(args.get(2).unwrap()).await,
        Some("response") => client.gen_model_response(Option::from("What is capital of Ohio and how was it founded?"), None, None).await.map(|_t| {}),

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
  add-user <name> <email> <api_key>   Insert a test user into Helix
  get-user <email>                     Fetch a user from Helix by email
  help            Show this help message
  version         Show the current version
"
    );
}

