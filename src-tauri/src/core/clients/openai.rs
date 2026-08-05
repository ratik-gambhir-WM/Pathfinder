use std::{fs, path::Path};

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use serde_json::{json, Value};

use crate::utils::openai_api_key;

const DEFAULT_RESPONSES_MODEL: &str = "gpt-5.6";
const DEFAULT_RESPONSES_PROMPT: &str = "Provide a helpful response.";
const DEFAULT_SYSTEM_INSTRUCTIONS: &str = "You are a helpful assistant.";
const DEFAULT_EMBEDDING_MODEL: &str = "text-embedding-3-small";

pub struct OpenAiClient {
    api_key: String,
}

#[derive(Debug)]
pub enum ResponsesFileInput<'a> {
    FileId(&'a str),
    FileUrl(&'a str),
    FileData {
        filename: &'a str,
        mime_type: &'a str,
        data_base64: &'a str,
    },
    ImageData {
        mime_type: &'a str,
        data_base64: &'a str,
        detail: Option<&'a str>,
    },
    FilePath(&'a Path),
}

#[derive(Debug, Clone, Copy)]
pub struct ResponsesRequestOptions<'a> {
    pub prompt: Option<&'a str>,
    pub system_instructions: Option<&'a str>,
    pub model: Option<&'a str>,
    pub file_inputs: Option<&'a [ResponsesFileInput<'a>]>,
    pub reasoning_effort: Option<&'a str>,
    pub max_output_tokens: Option<u32>,
}

impl<'a> Default for ResponsesRequestOptions<'a> {
    fn default() -> Self {
        Self {
            prompt: None,
            system_instructions: None,
            model: None,
            file_inputs: None,
            reasoning_effort: Some("high"),
            max_output_tokens: None,
        }
    }
}

struct ResolvedResponsesRequest<'a> {
    prompt: &'a str,
    system_instructions: &'a str,
    model: &'a str,
    file_inputs: Option<&'a [ResponsesFileInput<'a>]>,
    reasoning_effort: Option<&'a str>,
    max_output_tokens: Option<u32>,
}

impl OpenAiClient {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            api_key: openai_api_key()?,
        })
    }

    pub async fn gen_model_response(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
    ) -> Result<String, String> {
        self.gen_model_response_with_options(ResponsesRequestOptions {
            prompt,
            system_instructions,
            model,
            ..Default::default()
        })
        .await
    }

    pub async fn gen_model_response_with_files(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
        file_inputs: Option<&[ResponsesFileInput<'_>]>,
    ) -> Result<String, String> {
        self.gen_model_response_with_options(ResponsesRequestOptions {
            prompt,
            system_instructions,
            model,
            file_inputs,
            ..Default::default()
        })
        .await
    }

    pub async fn gen_model_response_with_files_and_reasoning(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
        file_inputs: Option<&[ResponsesFileInput<'_>]>,
        reasoning_effort: Option<&str>,
    ) -> Result<String, String> {
        self.gen_model_response_with_options(ResponsesRequestOptions {
            prompt,
            system_instructions,
            model,
            file_inputs,
            reasoning_effort,
            ..Default::default()
        })
        .await
    }

    pub async fn gen_model_response_with_options(
        &self,
        options: ResponsesRequestOptions<'_>,
    ) -> Result<String, String> {
        let openai_client = reqwest::Client::new();
        let request = resolve_responses_request_options(options)?;
        let request_body = build_responses_request_body(&request)?;

        let response = openai_client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&request_body)
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

    pub async fn gen_model_response_with_files_streaming<F>(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
        file_inputs: Option<&[ResponsesFileInput<'_>]>,
        on_text_delta: F,
    ) -> Result<String, String>
    where
        F: FnMut(&str) + Send,
    {
        self.gen_model_response_streaming_with_options(
            ResponsesRequestOptions {
                prompt,
                system_instructions,
                model,
                file_inputs,
                ..Default::default()
            },
            on_text_delta,
        )
        .await
    }

    pub async fn gen_model_response_with_files_streaming_and_reasoning<F>(
        &self,
        prompt: Option<&str>,
        system_instructions: Option<&str>,
        model: Option<&str>,
        file_inputs: Option<&[ResponsesFileInput<'_>]>,
        reasoning_effort: Option<&str>,
        on_text_delta: F,
    ) -> Result<String, String>
    where
        F: FnMut(&str) + Send,
    {
        self.gen_model_response_streaming_with_options(
            ResponsesRequestOptions {
                prompt,
                system_instructions,
                model,
                file_inputs,
                reasoning_effort,
                ..Default::default()
            },
            on_text_delta,
        )
        .await
    }

    pub async fn gen_model_response_streaming_with_options<F>(
        &self,
        options: ResponsesRequestOptions<'_>,
        mut on_text_delta: F,
    ) -> Result<String, String>
    where
        F: FnMut(&str) + Send,
    {
        let openai_client = reqwest::Client::new();
        let request = resolve_responses_request_options(options)?;
        let mut request_body = build_responses_request_body(&request)?;
        request_body["stream"] = json!(true);

        let mut response = openai_client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|err| format!("failed to call OpenAI responses API: {err}"))?;

        let status = response.status();
        if !status.is_success() {
            let response_body = response
                .text()
                .await
                .map_err(|err| format!("failed to read OpenAI responses response: {err}"))?;
            return Err(format!(
                "OpenAI responses API returned {status}: {response_body}"
            ));
        }

        let mut pending = String::new();
        let mut streamed_text = String::new();
        let mut completed_text = None;

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|err| format!("failed to read OpenAI responses stream chunk: {err}"))?
        {
            let chunk_text = std::str::from_utf8(&chunk)
                .map_err(|err| format!("OpenAI responses stream contained invalid UTF-8: {err}"))?;
            pending.push_str(chunk_text);
            process_sse_events(
                &mut pending,
                &mut streamed_text,
                &mut completed_text,
                &mut on_text_delta,
            )?;
        }

        if !pending.trim().is_empty() {
            process_sse_event(
                &pending,
                &mut streamed_text,
                &mut completed_text,
                &mut on_text_delta,
            )?;
        }

        if !streamed_text.trim().is_empty() {
            return Ok(streamed_text);
        }

        completed_text
            .filter(|text| !text.trim().is_empty())
            .ok_or_else(|| "OpenAI responses stream did not include output text".to_string())
    }

    pub async fn gen_file_embeddings(&self, content: &str) -> Result<(), String> {
        let embedding = self.gen_embedding(content, None).await?;
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

    pub async fn gen_embedding(
        &self,
        content: &str,
        model: Option<&str>,
    ) -> Result<Vec<f64>, String> {
        if content.trim().is_empty() {
            return Err("cannot embed empty document content".to_string());
        }

        let contents = [content];
        let mut embeddings = self.gen_embeddings(&contents, model).await?;
        Ok(embeddings
            .pop()
            .expect("a successful single-input embedding request should return one embedding"))
    }

    pub async fn gen_embeddings(
        &self,
        contents: &[&str],
        model: Option<&str>,
    ) -> Result<Vec<Vec<f64>>, String> {
        let openai_client: reqwest::Client = reqwest::Client::new();
        let request_body = build_embeddings_request_body(contents, model)?;

        let response = openai_client
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&self.api_key)
            .json(&request_body)
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

        extract_embeddings(&response_json, contents.len())
    }
}

fn build_embeddings_request_body(contents: &[&str], model: Option<&str>) -> Result<Value, String> {
    let model = model.unwrap_or(DEFAULT_EMBEDDING_MODEL).trim();

    if contents.is_empty() {
        return Err("cannot embed an empty list of document contents".to_string());
    }

    if let Some(index) = contents
        .iter()
        .position(|content| content.trim().is_empty())
    {
        return Err(format!(
            "cannot embed empty document content at input index {index}"
        ));
    }

    if model.is_empty() {
        return Err("model cannot be empty".to_string());
    }

    Ok(json!({
        "model": model,
        "input": contents,
        "encoding_format": "float"
    }))
}

fn resolve_responses_request_options<'a>(
    options: ResponsesRequestOptions<'a>,
) -> Result<ResolvedResponsesRequest<'a>, String> {
    let prompt = options.prompt.unwrap_or(DEFAULT_RESPONSES_PROMPT).trim();
    let system_instructions = options
        .system_instructions
        .unwrap_or(DEFAULT_SYSTEM_INSTRUCTIONS)
        .trim();
    let model = options.model.unwrap_or(DEFAULT_RESPONSES_MODEL).trim();
    let reasoning_effort = options.reasoning_effort.map(str::trim);

    if prompt.is_empty() {
        return Err("prompt cannot be empty".to_string());
    }

    if system_instructions.is_empty() {
        return Err("system instructions cannot be empty".to_string());
    }

    if model.is_empty() {
        return Err("model cannot be empty".to_string());
    }

    if matches!(reasoning_effort, Some("")) {
        return Err("reasoning effort cannot be empty".to_string());
    }

    if matches!(options.max_output_tokens, Some(0)) {
        return Err("max output tokens must be greater than 0".to_string());
    }

    Ok(ResolvedResponsesRequest {
        prompt,
        system_instructions,
        model,
        file_inputs: options.file_inputs,
        reasoning_effort,
        max_output_tokens: options.max_output_tokens,
    })
}

fn build_responses_request_body(request: &ResolvedResponsesRequest<'_>) -> Result<Value, String> {
    let mut request_body = json!({
        "model": request.model,
        "instructions": request.system_instructions,
        "input": [
            {
                "role": "user",
                "content": build_user_input_content(request.prompt, request.file_inputs)?,
            }
        ],
    });

    if let Some(reasoning_effort) = request.reasoning_effort {
        request_body["reasoning"] = json!({
            "effort": reasoning_effort,
        });
    }

    if let Some(max_output_tokens) = request.max_output_tokens {
        request_body["max_output_tokens"] = json!(max_output_tokens);
    }

    Ok(request_body)
}

fn build_user_input_content(
    prompt: &str,
    file_inputs: Option<&[ResponsesFileInput<'_>]>,
) -> Result<Vec<Value>, String> {
    let mut content = Vec::new();

    if let Some(file_inputs) = file_inputs {
        for file_input in file_inputs {
            content.push(build_input_item(file_input)?);
        }
    }

    content.push(json!({
        "type": "input_text",
        "text": prompt,
    }));

    Ok(content)
}

fn build_input_item(file_input: &ResponsesFileInput<'_>) -> Result<Value, String> {
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
        ResponsesFileInput::ImageData {
            mime_type,
            data_base64,
            detail,
        } => {
            let mime_type = mime_type.trim();
            let data_base64 = data_base64.trim();
            let detail = detail.unwrap_or("auto").trim();

            if mime_type.is_empty() {
                return Err("mime_type cannot be empty".to_string());
            }

            if data_base64.is_empty() {
                return Err("image_data cannot be empty".to_string());
            }

            if detail.is_empty() {
                return Err("image detail cannot be empty".to_string());
            }

            Ok(json!({
                "type": "input_image",
                "image_url": build_base64_data_url(mime_type, data_base64),
                "detail": detail,
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
            let file_data =
                build_base64_data_url(mime_type, &general_purpose::STANDARD.encode(file_bytes));

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

fn process_sse_events<F>(
    pending: &mut String,
    streamed_text: &mut String,
    completed_text: &mut Option<String>,
    on_text_delta: &mut F,
) -> Result<(), String>
where
    F: FnMut(&str),
{
    while let Some((event_end, separator_len)) = find_sse_event_boundary(pending) {
        let raw_event = pending[..event_end].to_string();
        pending.drain(..event_end + separator_len);
        process_sse_event(&raw_event, streamed_text, completed_text, on_text_delta)?;
    }

    Ok(())
}

fn find_sse_event_boundary(pending: &str) -> Option<(usize, usize)> {
    match (pending.find("\n\n"), pending.find("\r\n\r\n")) {
        (Some(lf_index), Some(crlf_index)) if crlf_index < lf_index => Some((crlf_index, 4)),
        (Some(lf_index), _) => Some((lf_index, 2)),
        (None, Some(crlf_index)) => Some((crlf_index, 4)),
        (None, None) => None,
    }
}

fn process_sse_event<F>(
    raw_event: &str,
    streamed_text: &mut String,
    completed_text: &mut Option<String>,
    on_text_delta: &mut F,
) -> Result<(), String>
where
    F: FnMut(&str),
{
    let data = raw_event
        .lines()
        .filter_map(|line| {
            let line = line.trim_end_matches('\r');
            line.strip_prefix("data:").map(str::trim_start)
        })
        .collect::<Vec<_>>()
        .join("\n");

    if data.trim().is_empty() || data.trim() == "[DONE]" {
        return Ok(());
    }

    let event_json: Value = serde_json::from_str(&data)
        .map_err(|err| format!("failed to parse OpenAI responses stream event: {err}"))?;

    if let Some(delta) = extract_response_stream_delta(&event_json) {
        streamed_text.push_str(&delta);
        on_text_delta(&delta);
    }

    if matches!(
        event_json.get("type").and_then(Value::as_str),
        Some("response.completed")
    ) {
        if let Some(text) = event_json
            .get("response")
            .and_then(extract_response_text)
            .filter(|text| !text.trim().is_empty())
        {
            *completed_text = Some(text);
        }
    }

    Ok(())
}

fn extract_response_stream_delta(event_json: &Value) -> Option<String> {
    match event_json.get("type").and_then(Value::as_str) {
        Some("response.output_text.delta") | Some("response.refusal.delta") => event_json
            .get("delta")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        _ => None,
    }
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

fn extract_embeddings(
    response_json: &Value,
    expected_count: usize,
) -> Result<Vec<Vec<f64>>, String> {
    let embedding_items = response_json["data"]
        .as_array()
        .ok_or_else(|| "OpenAI embeddings response did not include embeddings".to_string())?;
    let mut embeddings = vec![None; expected_count];

    for item in embedding_items {
        let index = item["index"]
            .as_u64()
            .and_then(|index| usize::try_from(index).ok())
            .ok_or_else(|| {
                "OpenAI embeddings response included an invalid embedding index".to_string()
            })?;
        let destination = embeddings.get_mut(index).ok_or_else(|| {
            format!("OpenAI embeddings response included out-of-range embedding index {index}")
        })?;
        if destination.is_some() {
            return Err(format!(
                "OpenAI embeddings response included duplicate embedding index {index}"
            ));
        }

        let embedding_values = item["embedding"].as_array().ok_or_else(|| {
            format!("OpenAI embeddings response did not include embedding at index {index}")
        })?;
        let embedding = embedding_values
            .iter()
            .map(|value| {
                value
                    .as_f64()
                    .ok_or_else(|| "OpenAI embedding contained a non-number value".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        *destination = Some(embedding);
    }

    embeddings
        .into_iter()
        .enumerate()
        .map(|(index, embedding)| {
            embedding.ok_or_else(|| {
                format!("OpenAI embeddings response did not include embedding at index {index}")
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "../../../tests/core/clients/openai_tests.rs"]
mod tests;
