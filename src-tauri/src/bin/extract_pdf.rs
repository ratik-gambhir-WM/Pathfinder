use quarry_lib::core::{
    clients::openai::OpenAiClient,
    parsers::pdf::{extract_pdf_image_descriptions, extract_pdf_text},
};
use std::{env, path::Path, process};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let args = env::args().skip(1).collect::<Vec<_>>();
    let result = match args.as_slice() {
        [] => Err("missing command or PDF path".to_string()),
        [path] => print_pdf_text(path),
        [command, path] if command == "text" => print_pdf_text(path),
        [command, path] if command == "images" => print_pdf_image_descriptions(path).await,
        [command, ..] => Err(format!("unknown command: {command}")),
    };

    if let Err(message) = result {
        eprintln!("error: {message}");
        eprintln!("usage: extract_pdf [text|images] <path-to.pdf>");
        process::exit(1);
    }
}

fn print_pdf_text(path: &str) -> Result<(), String> {
    let text = extract_pdf_text(Path::new(path))?;
    println!("{text}");
    Ok(())
}

async fn print_pdf_image_descriptions(path: &str) -> Result<(), String> {
    let openai_client = OpenAiClient::new()?;
    let descriptions = extract_pdf_image_descriptions(Path::new(path), &openai_client).await?;

    for (index, description) in descriptions.iter().enumerate() {
        println!("IMAGE DESCRIPTION {}", index + 1);
        println!("{description}");
        println!();
    }

    Ok(())
}
