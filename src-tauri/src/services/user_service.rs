use serde::Deserialize;

use crate::{
    repository::user_repository::{
        create_user, get_user_by_email as fetch_user_by_email, CreateUserRecord,
    },
    state::AppState,
};

pub use crate::repository::user_repository::User;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub api_key: String,
    pub role: String,
}

pub fn add_user(state: &AppState, input: AddUserInput) -> Result<User, String> {
    validate_user_input(&input)?;
    create_user(
        state,
        CreateUserRecord {
            first_name: input.first_name.trim(),
            last_name: input.last_name.trim(),
            email: input.email.trim(),
            api_key: input.api_key.trim(),
            role: input.role.trim(),
        },
    )
}

pub fn get_user_by_email(state: &AppState, email: &str) -> Result<Option<User>, String> {
    let email = email.trim();
    if email.is_empty() {
        return Err("email is required".to_string());
    }

    fetch_user_by_email(state, email)
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
