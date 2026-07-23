use std::path::Path;

use rusqlite::Connection;
use tauri::AppHandle;

use crate::clients::{helix::HelixClient, sqlite::SqliteClient};

pub struct AppState {
    pub helix_client: HelixClient,
    pub sqlite_client: SqliteClient,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        Ok(Self {
            helix_client: HelixClient::new()?,
            sqlite_client: SqliteClient::new(app)?,
        })
    }

    pub fn db_path(&self) -> &Path {
        &self.sqlite_client.db_path
    }

    pub fn with_db<T>(
        &self,
        f: impl FnOnce(&Connection) -> rusqlite::Result<T>,
    ) -> Result<T, String> {
        let db = self
            .sqlite_client
            .db
            .lock()
            .map_err(|_| "sqlite connection lock was poisoned".to_string())?;

        f(&db).map_err(|err| format!("sqlite error: {err}"))
    }
}
