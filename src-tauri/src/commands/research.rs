use crate::{
    commands::{CommandResult, CommandResultExt},
    core::write_summary,
    services::research_service::{
        list_summarizable_files, summarize_dir, summarize_paths, SummarizableFile,
    },
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
pub fn list_summary_files(payload: DirPathPayload) -> CommandResult<Vec<SummarizableFile>> {
    list_summarizable_files(payload.path).command_context("list_summary_files")
}

#[tauri::command]
pub async fn summarize(payload: DirPathPayload) -> CommandResult<String> {
    println!("Summarizing...");
    println!("{:?}", payload.path);
    summarize_dir(payload.path)
        .await
        .command_context("summarize")
}

#[tauri::command]
pub async fn summarize_selected(payload: SelectedPathsPayload) -> CommandResult<String> {
    println!("Summarizing selected files...");
    summarize_paths(payload.paths)
        .await
        .command_context("summarize_selected")
}

#[tauri::command]
pub fn save_markdown_summary(payload: SaveMarkdownSummaryPayload) -> CommandResult<()> {
    write_summary(&payload.summary, payload.path).command_context("save_markdown_summary")
}
