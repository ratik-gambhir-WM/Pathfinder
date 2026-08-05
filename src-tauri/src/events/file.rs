use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Listener, Runtime};

use crate::services::document_service::process_files;

pub const PROCESS_FILES_EVENT: &str = "files:process";
pub const FILE_PROCESSED_EVENT: &str = "files:processed";
pub const FILE_BATCH_FINISHED_EVENT: &str = "files:batch-finished";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessFilesEvent {
    pub request_id: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileProcessedEvent {
    pub request_id: String,
    pub path: String,
    pub file_id: Option<String>,
    pub completed: usize,
    pub total: usize,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBatchFinishedEvent {
    pub request_id: String,
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
}

pub fn register_file_events<R: Runtime>(app: &AppHandle<R>) {
    let app_handle = app.clone();

    app.listen(PROCESS_FILES_EVENT, move |event| {
        let payload = event.payload().to_string();
        let app_handle = app_handle.clone();

        tauri::async_runtime::spawn(async move {
            let request = match serde_json::from_str::<ProcessFilesEvent>(&payload) {
                Ok(request) => request,
                Err(error) => {
                    eprintln!("failed to parse {PROCESS_FILES_EVENT} payload: {error}");
                    return;
                }
            };

            process_files(app_handle, request).await;
        });
    });
}

#[cfg(test)]
#[path = "../../tests/events/file_tests.rs"]
mod tests;
