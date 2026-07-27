use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub api_key: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct CreateUserRecord<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub api_key: &'a str,
    pub role: &'a str,
}

pub fn create_user(state: &AppState, record: CreateUserRecord<'_>) -> Result<User, String> {
    state.gen_sqlite_db_client().execute(
        r#"
        INSERT INTO users (first_name, last_name, email, api_key, role)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![
            record.first_name,
            record.last_name,
            record.email,
            record.api_key,
            record.role
        ],
    )?;

    get_user_by_email(state, record.email)?.ok_or_else(|| {
        format!(
            "failed to fetch user after insert for email `{}`",
            record.email
        )
    })
}

pub fn get_user_by_email(state: &AppState, email: &str) -> Result<Option<User>, String> {
    state.with_sqlite_db(|connection| query_user_by_email(connection, email))
}

fn query_user_by_email(connection: &Connection, email: &str) -> rusqlite::Result<Option<User>> {
    connection
        .query_row(
            r#"
        SELECT id, first_name, last_name, email, api_key, role, created_at, updated_at
        FROM users
        WHERE email = ?1 COLLATE NOCASE
        ORDER BY id
        LIMIT 1
        "#,
            [email],
            user_from_row,
        )
        .optional()
}

fn user_from_row(row: &Row<'_>) -> rusqlite::Result<User> {
    Ok(User {
        id: row.get("id")?,
        first_name: row.get("first_name")?,
        last_name: row.get("last_name")?,
        email: row.get("email")?,
        api_key: row.get("api_key")?,
        role: row.get("role")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_user_by_email_matches_email_case_insensitively() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                r#"
                CREATE TABLE users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    first_name TEXT NOT NULL,
                    last_name TEXT NOT NULL,
                    email TEXT NOT NULL UNIQUE,
                    api_key TEXT NOT NULL,
                    role TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                );

                INSERT INTO users (first_name, last_name, email, api_key, role)
                VALUES ('Sam', 'Example', 'SAM@gmail.com', 'test-key', 'user');
                "#,
            )
            .unwrap();

        let user = query_user_by_email(&connection, "sam@gmail.com")
            .unwrap()
            .expect("case-insensitive email lookup should find the existing user");

        assert_eq!(user.email, "SAM@gmail.com");
    }
}
