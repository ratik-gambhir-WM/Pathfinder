use std::{env, process};

use pathfinder_lib::clients::helix::HelixClient;
use pathfinder_lib::clients::openai::OpenAiClient;

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
        Some("add-user") => {
            helix_db
                .add_user(args.get(2), args.get(3), args.get(4))
                .await
        }
        Some("get-user") => helix_db.get_user(args.get(2)).await,
        Some("embed") => match args.get(2) {
            Some(content) => match client.gen_embedding(content, None).await {
                Ok(embedding) => match serde_json::to_string(&embedding) {
                    Ok(json) => {
                        println!("{json}");
                        Ok(())
                    }
                    Err(err) => Err(format!("failed to serialize embedding: {err}")),
                },
                Err(err) => Err(err),
            },
            None => Err("missing content to embed".to_string()),
        },
        Some("response") => client
            .gen_model_response(
                Option::from("What is capital of Ohio and how was it founded?"),
                None,
                None,
            )
            .await
            .map(|_t| {}),

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
  embed <content>                      Generate an embedding and print it as JSON
  help            Show this help message
  version         Show the current version
"
    );
}
