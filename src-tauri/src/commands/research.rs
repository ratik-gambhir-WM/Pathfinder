use crate::common::write_summary;
use crate::services::research_service::{
    list_summarizable_files, summarize_dir, summarize_paths, SummarizableFile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDemoCommandPayload {
    pub email: String,
    pub source: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDemoCommandResponse {
    pub message: String,
    pub echoed_email: String,
    pub source: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirPathPayload {
    pub path: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedPathsPayload {
    pub paths: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirPathPayloadResponse {
    pub summary: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMarkdownSummaryPayload {
    pub path: String,
    pub summary: String,
}

#[tauri::command]
pub fn login_demo_command(payload: LoginDemoCommandPayload) -> LoginDemoCommandResponse {
    LoginDemoCommandResponse {
        message: format!("Rust received a command from {}", payload.source),
        echoed_email: payload.email,
        source: "tauri-command".to_string(),
    }
}

#[tauri::command]
pub fn list_summary_files(payload: DirPathPayload) -> Result<Vec<SummarizableFile>, String> {
    list_summarizable_files(payload.path)
}

#[tauri::command]
pub async fn summarize(payload: DirPathPayload) -> Result<String, String> {
    println!("Summarizing...");
    println!("{:?}", payload.path);
    let summary = summarize_dir(payload.path).await;
    match summary {
        Ok(sum) => Ok(sum),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn summarize_selected(payload: SelectedPathsPayload) -> Result<String, String> {
    println!("Summarizing selected files...");
    let summary = summarize_paths(payload.paths).await;
    match summary {
        Ok(sum) => Ok(sum),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn save_markdown_summary(payload: SaveMarkdownSummaryPayload) -> Result<(), String> {
    write_summary(&payload.summary, payload.path)
}
