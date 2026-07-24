use serde::Serialize;
use tauri::State;

use crate::{
    commands::{CommandResult, CommandResultExt},
    services::deal_service::{
        extract_deal_questions_and_thesis_for_selected_files,
        save_deal_and_extract as save_deal_and_extract_in_service,
        ExtractDealQuestionsAndThesisInput, SaveDealAndExtractInput, SaveDealAndExtractResponse,
        SaveDealAndFindFilesResponse,
    },
    state::AppState,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub database_path: String,
    pub user_version: i64,
}

#[tauri::command]
pub fn database_status(state: State<'_, AppState>) -> CommandResult<DatabaseStatus> {
    let user_version = state
        .with_sqlite_db(|db| db.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0)))
        .command_context("database_status")?;

    Ok(DatabaseStatus {
        database_path: state.sqlite_db_path().display().to_string(),
        user_version,
    })
}

#[tauri::command]
pub async fn save_deal_and_extract(
    state: State<'_, AppState>,
    input: SaveDealAndExtractInput,
) -> CommandResult<SaveDealAndFindFilesResponse> {
    save_deal_and_extract_in_service(&state, input)
        .await
        .command_context("save_deal_and_extract")
}

#[tauri::command]
pub async fn extract_deal_questions_and_thesis(
    state: State<'_, AppState>,
    input: ExtractDealQuestionsAndThesisInput,
) -> CommandResult<SaveDealAndExtractResponse> {
    extract_deal_questions_and_thesis_for_selected_files(&state, input)
        .await
        .command_context("extract_deal_questions_and_thesis")
}
