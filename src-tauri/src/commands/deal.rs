use serde::Serialize;
use tauri::State;

use crate::state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub database_path: String,
    pub user_version: i64,
}

#[tauri::command]
pub fn database_status(state: State<'_, AppState>) -> Result<DatabaseStatus, String> {
    let user_version = state
        .with_sqlite_db(|db| db.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0)))?;

    Ok(DatabaseStatus {
        database_path: state.sqlite_db_path().display().to_string(),
        user_version,
    })
}
