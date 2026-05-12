#![allow(dead_code)]

use std::{env, fs, path::Path};

use crate::{
    clients::openai::{OpenAiClient, ResponsesFileInput},
    models::document::ImageEmbeddingResult,
    utils::openai_api_key,
};
use base64::{engine::general_purpose, Engine as _};

const DEFAULT_IMAGE_DESCRIPTION_MODEL: &str = "gpt-5.4";
const DEFAULT_EMBEDDING_MODEL: &str = "text-embedding-3-small";

const IMAGE_EMBEDDING_SUMMARY_PROMPT: &str = r#"Create a dense, retrieval-optimized description of this image for vector search.

Focus on stable visual facts that would help someone find this image later:
- primary subjects, objects, people, setting, scene type, visible text, charts, document layout, brand/product clues, colors, style, and notable relationships
- if the image is a screenshot, diagram, table, chart, form, or document page, describe its structure and any readable text
- avoid speculation, filler, markdown, and generic phrases like "this image shows"

Return one concise paragraph, roughly 80-160 words."#;

pub async fn describe_and_embed_image_file(
    image_path: &Path,
    openai_client: &reqwest::Client,
) -> Result<ImageEmbeddingResult, String> {
    let image = fs::read(image_path)
        .map_err(|err| format!("failed to read image file {}: {err}", image_path.display()))?;
    let mime_type = infer_image_mime_type(image_path)?;

    describe_and_embed_image(image, mime_type, openai_client).await
}

pub async fn describe_and_embed_image(
    image: Vec<u8>,
    mime_type: &str,
    openai_client: &reqwest::Client,
) -> Result<ImageEmbeddingResult, String> {
    if image.is_empty() {
        return Err("cannot describe and embed an empty image".to_string());
    }

    let normalized_mime_type = normalize_image_mime_type(mime_type)?;
    let description = describe_image(&image, normalized_mime_type, openai_client).await?;
    let embedding = embed_description(&description, openai_client).await?;

    Ok(ImageEmbeddingResult {
        image,
        description,
        embedding,
    })
}

async fn describe_image(
    image: &[u8],
    mime_type: &str,
    _openai_client: &reqwest::Client,
) -> Result<String, String> {
    let api_key = openai_api_key()?;
    let client = OpenAiClient::new(&api_key);
    let model = env::var("OPENAI_IMAGE_DESCRIPTION_MODEL")
        .unwrap_or_else(|_| DEFAULT_IMAGE_DESCRIPTION_MODEL.to_string());
    let image_base64 = general_purpose::STANDARD.encode(image);
    let file_inputs = [ResponsesFileInput::ImageData {
        mime_type,
        data_base64: image_base64.as_str(),
        detail: Some("auto"),
    }];
    let description = client
        .gen_model_response_with_files(
            Some(IMAGE_EMBEDDING_SUMMARY_PROMPT),
            None,
            Some(&model),
            Some(&file_inputs),
        )
        .await?;
    let description = description.trim().to_string();

    if description.is_empty() {
        return Err("OpenAI image analysis returned an empty description".to_string());
    }

    Ok(description)
}

async fn embed_description(
    description: &str,
    _openai_client: &reqwest::Client,
) -> Result<Vec<f64>, String> {
    if description.trim().is_empty() {
        return Err("cannot embed an empty image description".to_string());
    }

    let api_key = openai_api_key()?;
    let client = OpenAiClient::new(&api_key);
    let model =
        env::var("OPENAI_EMBEDDING_MODEL").unwrap_or_else(|_| DEFAULT_EMBEDDING_MODEL.to_string());

    client.gen_embedding(description, Some(&model)).await
}

fn infer_image_mime_type(image_path: &Path) -> Result<&'static str, String> {
    match image_path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => Ok("image/png"),
        Some("jpg") | Some("jpeg") => Ok("image/jpeg"),
        Some("webp") => Ok("image/webp"),
        Some("gif") => Ok("image/gif"),
        Some(extension) => Err(format!(
            "unsupported image extension .{extension}; expected png, jpg, jpeg, webp, or gif"
        )),
        None => Err(format!(
            "could not infer image type for {}; pass bytes with an explicit MIME type instead",
            image_path.display()
        )),
    }
}

fn normalize_image_mime_type(mime_type: &str) -> Result<&'static str, String> {
    match mime_type.trim().to_ascii_lowercase().as_str() {
        "image/png" => Ok("image/png"),
        "image/jpeg" | "image/jpg" => Ok("image/jpeg"),
        "image/webp" => Ok("image/webp"),
        "image/gif" => Ok("image/gif"),
        value => Err(format!(
            "unsupported image MIME type {value}; expected image/png, image/jpeg, image/webp, or image/gif"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_supported_mime_types() {
        assert_eq!(
            infer_image_mime_type(Path::new("screenshot.PNG")).unwrap(),
            "image/png"
        );
        assert_eq!(
            infer_image_mime_type(Path::new("photo.jpeg")).unwrap(),
            "image/jpeg"
        );
        assert_eq!(
            infer_image_mime_type(Path::new("graphic.webp")).unwrap(),
            "image/webp"
        );
    }
}
