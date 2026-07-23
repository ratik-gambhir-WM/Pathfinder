use rusqlite::{params, Row};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub api_key: String,
    pub role: String,
}

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

pub fn add_user(state: &AppState, input: AddUserInput) -> Result<User, String> {
    validate_user_input(&input)?;
    let email = input.email.trim().to_string();

    state.gen_sqlite_db_client().execute(
        r#"
        INSERT INTO users (first_name, last_name, email, api_key, role)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![
            input.first_name.trim(),
            input.last_name.trim(),
            email,
            input.api_key.trim(),
            input.role.trim()
        ],
    )?;

    get_user_by_email(state, &email)?
        .ok_or_else(|| format!("failed to fetch user after insert for email `{email}`"))
}

pub fn get_user_by_email(state: &AppState, email: &str) -> Result<Option<User>, String> {
    let email = email.trim();
    if email.is_empty() {
        return Err("email is required".to_string());
    }

    let users = state.gen_sqlite_db_client().query_rows(
        r#"
        SELECT id, first_name, last_name, email, api_key, role, created_at, updated_at
        FROM users
        WHERE email = ?1
        "#,
        [email],
        user_from_row,
    )?;

    Ok(users.into_iter().next())
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

fn validate_user_input(input: &AddUserInput) -> Result<(), String> {
    if input.first_name.trim().is_empty() {
        return Err("first_name is required".to_string());
    }

    if input.last_name.trim().is_empty() {
        return Err("last_name is required".to_string());
    }

    if input.email.trim().is_empty() {
        return Err("email is required".to_string());
    }

    if input.api_key.trim().is_empty() {
        return Err("api_key is required".to_string());
    }

    if input.role.trim().is_empty() {
        return Err("role is required".to_string());
    }

    Ok(())
}
