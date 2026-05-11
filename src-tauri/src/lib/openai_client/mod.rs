use std::{fs, path::Path};

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use serde_json::{json, Value};
use crate::utils::get_token_count;

const DEFAULT_RESPONSES_MODEL: &str = "gpt-5.5";
const DEFAULT_RESPONSES_PROMPT: &str = "Provide a helpful response.";
const DEFAULT_SYSTEM_INSTRUCTIONS: &str = "You are a helpful assistant.";

pub struct OpenAiClient<'a> {
    api_key: &'a str,
}

pub enum ResponsesFileInput<'a> {
    FileId(&'a str),
    FileUrl(&'a str),
    FileData {
        filename: &'a str,
        mime_type: &'a str,
        data_base64: &'a str,
    },
    FilePath(&'a Path),
}

impl<'a> OpenAiClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        OpenAiClient {api_key }
    }

    pub async fn gen_model_response(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
    ) -> Result<String, String> {
        let token_count = get_token_count(prompt);
        println!("Token count: {}", token_count);
        self.gen_model_response_with_files(prompt, system_instructions, model, None)
            .await
    }

    pub async fn gen_model_response_with_files(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
        file_inputs: Option<&[ResponsesFileInput<'_>]>,
    ) -> Result<String, String> {
        let openai_client = reqwest::Client::new();
        let prompt = prompt.unwrap_or(DEFAULT_RESPONSES_PROMPT).trim();
        let system_instructions = system_instructions
            .unwrap_or(DEFAULT_SYSTEM_INSTRUCTIONS)
            .trim();
        let model = model.unwrap_or(DEFAULT_RESPONSES_MODEL).trim();
        let mut content = Vec::new();

        if prompt.is_empty() {
            return Err("prompt cannot be empty".to_string());
        }

        if system_instructions.is_empty() {
            return Err("system instructions cannot be empty".to_string());
        }

        if model.is_empty() {
            return Err("model cannot be empty".to_string());
        }

        if let Some(file_inputs) = file_inputs {
            for file_input in file_inputs {
                content.push(build_file_input_item(file_input)?);
            }
        }

        content.push(json!({
            "type": "input_text",
            "text": prompt,
        }));

        let response = openai_client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(self.api_key)
            .json(&json!({
                "model": model,
                "instructions": system_instructions,
                "input": [
                    {
                        "role": "user",
                        "content": content
                    }
                ],
            }))
            .send()
            .await
            .map_err(|err| format!("failed to call OpenAI responses API: {err}"))?;

        let status = response.status();
        let response_body = response
            .text()
            .await
            .map_err(|err| format!("failed to read OpenAI responses response: {err}"))?;

        if !status.is_success() {
            return Err(format!(
                "OpenAI responses API returned {status}: {response_body}"
            ));
        }

        let response_json: Value = serde_json::from_str(&response_body)
            .map_err(|err| format!("failed to parse OpenAI responses response: {err}"))?;

        extract_response_text(&response_json)
            .ok_or_else(|| "OpenAI responses API did not include output text".to_string())
    }

    pub async fn gen_document_embeddings(
        &self,
        content: &str,
    ) -> Result<(), String> {
        let openai_client: reqwest::Client = reqwest::Client::new();
        if content.trim().is_empty() {
            return Err("cannot embed empty document content".to_string());
        }

        let response = openai_client
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(self.api_key)
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

        for embed in embedding {
            let string = embed.to_string();
            println!("embedded document at {string}");
        }

        Ok(())
    }
}

fn build_file_input_item(file_input: &ResponsesFileInput<'_>) -> Result<Value, String> {
    match file_input {
        ResponsesFileInput::FileId(file_id) => {
            let file_id = file_id.trim();
            if file_id.is_empty() {
                return Err("file_id cannot be empty".to_string());
            }

            Ok(json!({
                "type": "input_file",
                "file_id": file_id,
            }))
        }
        ResponsesFileInput::FileUrl(file_url) => {
            let file_url = file_url.trim();
            if file_url.is_empty() {
                return Err("file_url cannot be empty".to_string());
            }

            Ok(json!({
                "type": "input_file",
                "file_url": file_url,
            }))
        }
        ResponsesFileInput::FileData {
            filename,
            mime_type,
            data_base64,
        } => {
            let filename = filename.trim();
            let mime_type = mime_type.trim();
            let data_base64 = data_base64.trim();

            if filename.is_empty() {
                return Err("filename cannot be empty".to_string());
            }

            if mime_type.is_empty() {
                return Err("mime_type cannot be empty".to_string());
            }

            if data_base64.is_empty() {
                return Err("file_data cannot be empty".to_string());
            }

            Ok(json!({
                "type": "input_file",
                "filename": filename,
                "file_data": build_base64_data_url(mime_type, data_base64),
            }))
        }
        ResponsesFileInput::FilePath(path) => {
            let file_bytes = fs::read(path)
                .map_err(|err| format!("failed to read file input {}: {err}", path.display()))?;
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("failed to derive filename from {}", path.display()))?;
            let mime_type = infer_file_mime_type(path);
            let file_data = build_base64_data_url(
                mime_type,
                &general_purpose::STANDARD.encode(file_bytes),
            );

            Ok(json!({
                "type": "input_file",
                "filename": filename,
                "file_data": file_data,
            }))
        }
    }
}

fn build_base64_data_url(mime_type: &str, data_base64: &str) -> String {
    format!("data:{mime_type};base64,{data_base64}")
}

fn infer_file_mime_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .as_deref()
    {
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        Some("md") => "text/markdown",
        Some("json") => "application/json",
        Some("html") => "text/html",
        Some("csv") => "text/csv",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}

fn extract_response_text(response_json: &Value) -> Option<String> {
    if let Some(output_text) = response_json.get("output_text").and_then(Value::as_str) {
        return Some(output_text.trim().to_string());
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
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        text_parts.push(trimmed);
                    }
                }
            }
        }
    }

    if text_parts.is_empty() {
        None
    } else {
        let text = text_parts.join("\n");
        println!("{}", text);
        Some(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, time::{SystemTime, UNIX_EPOCH}};

    #[test]
    fn build_file_input_item_uses_input_file_type_for_file_id() {
        let item = build_file_input_item(&ResponsesFileInput::FileId("file-123")).unwrap();

        assert_eq!(
            item,
            json!({
                "type": "input_file",
                "file_id": "file-123",
            })
        );
    }

    #[test]
    fn build_file_input_item_wraps_base64_payload_in_data_url() {
        let item = build_file_input_item(&ResponsesFileInput::FileData {
            filename: "draconomicon.pdf",
            mime_type: "application/pdf",
            data_base64: "YWJjMTIz",
        })
        .unwrap();

        assert_eq!(
            item,
            json!({
                "type": "input_file",
                "filename": "draconomicon.pdf",
                "file_data": "data:application/pdf;base64,YWJjMTIz",
            })
        );
    }

    #[test]
    fn build_file_input_item_reads_file_path_as_input_file_data_url() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file_path = env::temp_dir().join(format!("openai-client-test-{unique}.pdf"));
        fs::write(&file_path, b"abc123").unwrap();

        let item = build_file_input_item(&ResponsesFileInput::FilePath(&file_path)).unwrap();

        assert_eq!(
            item,
            json!({
                "type": "input_file",
                "filename": file_path.file_name().and_then(|name| name.to_str()).unwrap(),
                "file_data": "data:application/pdf;base64,YWJjMTIz",
            })
        );

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn infer_file_mime_type_returns_pdf_for_pdf_extension() {
        assert_eq!(
            infer_file_mime_type(Path::new("draconomicon.pdf")),
            "application/pdf"
        );
    }

    #[test]
    fn infer_file_mime_type_falls_back_to_octet_stream_for_unknown_extension() {
        assert_eq!(
            infer_file_mime_type(Path::new("archive.unknown")),
            "application/octet-stream"
        );
    }

    #[test]
    fn extract_response_text_prefers_output_text_field() {
        let response_json = json!({
            "output_text": "  First dragon: Aasterinian  "
        });

        assert_eq!(
            extract_response_text(&response_json),
            Some("First dragon: Aasterinian".to_string())
        );
    }

    #[test]
    fn extract_response_text_collects_text_parts_from_output_content() {
        let response_json = json!({
            "output": [
                {
                    "content": [
                        {
                            "type": "output_text",
                            "text": "First paragraph"
                        },
                        {
                            "type": "text",
                            "text": "Second paragraph"
                        }
                    ]
                }
            ]
        });

        assert_eq!(
            extract_response_text(&response_json),
            Some("First paragraph\nSecond paragraph".to_string())
        );
    }

    #[test]
    fn extract_response_text_returns_none_when_no_text_is_present() {
        let response_json = json!({
            "output": [
                {
                    "content": [
                        {
                            "type": "input_file",
                            "filename": "draconomicon.pdf"
                        }
                    ]
                }
            ]
        });

        assert_eq!(extract_response_text(&response_json), None);
    }
}
