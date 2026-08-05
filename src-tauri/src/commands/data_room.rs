use crate::{
    commands::{CommandResult, CommandResultExt},
    services::data_room_service::{
        build_document_preview, list_deal_data_room as list_deal_data_room_in_service,
        DealDataRoom, DocumentPreview,
    },
};

#[tauri::command]
pub fn list_deal_data_room(deal_id: String) -> CommandResult<DealDataRoom> {
    list_deal_data_room_in_service(deal_id).command_context("list_deal_data_room")
}

#[tauri::command]
pub async fn preview_deal_document(
    deal_id: String,
    relative_path: String,
) -> CommandResult<DocumentPreview> {
    tauri::async_runtime::spawn_blocking(move || build_document_preview(&deal_id, &relative_path))
        .await
        .map_err(|err| format!("document preview worker failed: {err}"))
        .and_then(|result| result)
        .command_context("preview_deal_document")
}

#[cfg(test)]
#[path = "../../tests/commands/data_room_tests.rs"]
mod tests;
