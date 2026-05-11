#![allow(dead_code)]

use std::{env, fs, path::Path};

use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};

const DEFAULT_IMAGE_DESCRIPTION_MODEL: &str = "gpt-5.4";
const DEFAULT_EMBEDDING_MODEL: &str = "text-embedding-3-small";

const IMAGE_EMBEDDING_SUMMARY_PROMPT: &str = r#"Create a dense, retrieval-optimized description of this image for vector search.

Focus on stable visual facts that would help someone find this image later:
- primary subjects, objects, people, setting, scene type, visible text, charts, document layout, brand/product clues, colors, style, and notable relationships
- if the image is a screenshot, diagram, table, chart, form, or document page, describe its structure and any readable text
- avoid speculation, filler, markdown, and generic phrases like "this image shows"

Return one concise paragraph, roughly 80-160 words."#;

#[derive(Debug, Clone)]
pub struct ImageEmbeddingResult {
    pub image: Vec<u8>,
    pub description: String,
    pub embedding: Vec<f64>,
}

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
    openai_client: &reqwest::Client,
) -> Result<String, String> {
    let api_key = openai_api_key()?;
    let model = env::var("OPENAI_IMAGE_DESCRIPTION_MODEL")
        .unwrap_or_else(|_| DEFAULT_IMAGE_DESCRIPTION_MODEL.to_string());
    let image_base64 = general_purpose::STANDARD.encode(image);
    let image_url = format!("data:{mime_type};base64,{image_base64}");

    let response = openai_client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "input": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "input_text",
                            "text": IMAGE_EMBEDDING_SUMMARY_PROMPT
                        },
                        {
                            "type": "input_image",
                            "image_url": image_url,
                            "detail": "auto"
                        }
                    ]
                }
            ],
            "max_output_tokens": 500
        }))
        .send()
        .await
        .map_err(|err| format!("failed to call OpenAI image analysis API: {err}"))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|err| format!("failed to read OpenAI image analysis response: {err}"))?;

    if !status.is_success() {
        return Err(format!(
            "OpenAI image analysis API returned {status}: {response_body}"
        ));
    }

    let response_json: Value = serde_json::from_str(&response_body)
        .map_err(|err| format!("failed to parse OpenAI image analysis response: {err}"))?;
    let description = extract_response_text(&response_json)
        .ok_or_else(|| "OpenAI image analysis response did not include output text".to_string())?;
    let description = description.trim().to_string();

    if description.is_empty() {
        return Err("OpenAI image analysis returned an empty description".to_string());
    }

    Ok(description)
}

async fn embed_description(
    description: &str,
    openai_client: &reqwest::Client,
) -> Result<Vec<f64>, String> {
    if description.trim().is_empty() {
        return Err("cannot embed an empty image description".to_string());
    }

    let api_key = openai_api_key()?;
    let model =
        env::var("OPENAI_EMBEDDING_MODEL").unwrap_or_else(|_| DEFAULT_EMBEDDING_MODEL.to_string());

    let response = openai_client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "input": description,
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

    let response_json: Value = serde_json::from_str(&response_body)
        .map_err(|err| format!("failed to parse OpenAI embeddings response: {err}"))?;

    extract_embedding(&response_json)
}

fn openai_api_key() -> Result<String, String> {
    env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable is not set".to_string())
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

fn extract_response_text(response_json: &Value) -> Option<String> {
    if let Some(output_text) = response_json.get("output_text").and_then(Value::as_str) {
        return Some(output_text.to_string());
    }

    let output = response_json.get("output")?.as_array()?;
    let mut text_parts = Vec::new();

    for item in output {
        let Some(content) = item.get("content").and_then(Value::as_array) else {
            continue;
        };

        for part in content {
            if matches!(
                part.get("type").and_then(Value::as_str),
                Some("output_text") | Some("text")
            ) {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    text_parts.push(text);
                }
            }
        }
    }

    (!text_parts.is_empty()).then(|| text_parts.join("\n"))
}

fn extract_embedding(response_json: &Value) -> Result<Vec<f64>, String> {
    let embedding_values = response_json["data"]
        .get(0)
        .and_then(|item| item["embedding"].as_array())
        .ok_or_else(|| "OpenAI embeddings response did not include an embedding".to_string())?;

    embedding_values
        .iter()
        .map(|value| {
            value
                .as_f64()
                .ok_or_else(|| "OpenAI embedding contained a non-number value".to_string())
        })
        .collect()
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

    #[test]
    fn extracts_output_text_from_responses_api_shape() {
        let response_json = json!({
            "output": [
                {
                    "type": "message",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "A retrieval-ready image description."
                        }
                    ]
                }
            ]
        });

        assert_eq!(
            extract_response_text(&response_json).unwrap(),
            "A retrieval-ready image description."
        );
    }

    #[test]
    fn extracts_embedding_vector() {
        let response_json = json!({
            "data": [
                {
                    "embedding": [0.1, -0.2, 0.3]
                }
            ]
        });

        assert_eq!(
            extract_embedding(&response_json).unwrap(),
            vec![0.1, -0.2, 0.3]
        );
    }
}
