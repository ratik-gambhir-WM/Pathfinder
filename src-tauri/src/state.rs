use std::path::Path;

use rusqlite::Connection;
use tauri::AppHandle;

use crate::core::clients::{helix::HelixClient, sqlite::SqliteClient};

pub struct AppState {
    helix_client: HelixClient,
    sqlite_client: SqliteClient,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        Ok(Self {
            helix_client: HelixClient::new()?,
            sqlite_client: SqliteClient::new(app)?,
        })
    }

    #[cfg(test)]
    pub fn new_for_test() -> Result<Self, String> {
        Ok(Self {
            helix_client: HelixClient::new()?,
            sqlite_client: SqliteClient::new_in_memory()?,
        })
    }

    pub fn gen_helix_db_client(&self) -> &HelixClient {
        &self.helix_client
    }

    pub fn gen_sqlite_db_client(&self) -> &SqliteClient {
        &self.sqlite_client
    }

    pub fn sqlite_db_path(&self) -> &Path {
        &self.sqlite_client.db_path
    }

    pub fn with_sqlite_db<T>(
        &self,
        f: impl FnOnce(&Connection) -> rusqlite::Result<T>,
    ) -> Result<T, String> {
        self.sqlite_client.with_connection(f)
    }
}
