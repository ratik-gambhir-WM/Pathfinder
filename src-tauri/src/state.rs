use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

const DATABASE_FILE_NAME: &str = "pathfinder.sqlite3";

pub struct AppState {
    db: Mutex<Connection>,
    db_path: PathBuf,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let db_path = database_path(app)?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create database directory: {err}"))?;
        }

        let connection = Connection::open(&db_path)
            .map_err(|err| format!("failed to open sqlite database: {err}"))?;

        run_migrations(&connection)?;

        Ok(Self {
            db: Mutex::new(connection),
            db_path,
        })
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn with_db<T>(
        &self,
        f: impl FnOnce(&Connection) -> rusqlite::Result<T>,
    ) -> Result<T, String> {
        let db = self
            .db
            .lock()
            .map_err(|_| "sqlite connection lock was poisoned".to_string())?;

        f(&db).map_err(|err| format!("sqlite error: {err}"))
    }
}

fn database_path(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(path) = std::env::var("PATHFINDER_DATABASE_PATH") {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("failed to resolve app data directory: {err}"))?;

    Ok(app_data_dir.join(DATABASE_FILE_NAME))
}

fn run_migrations(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS app_metadata (
                key TEXT PRIMARY KEY NOT NULL,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                api_key TEXT NOT NULL,
                role TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS reminders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                reminder TEXT NOT NULL,
                notes TEXT NOT NULL,
                date TEXT NOT NULL,
                link TEXT NOT NULL,
                time TEXT,
                deal TEXT,
                tag TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            "#,
        )
        .map_err(|err| format!("failed to initialize sqlite database: {err}"))?;

    let user_version = connection
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(|err| format!("failed to read sqlite schema version: {err}"))?;

    if user_version < 1 {
        connection
            .pragma_update(None, "user_version", 1)
            .map_err(|err| format!("failed to set sqlite schema version: {err}"))?;
    }

    Ok(())
}
