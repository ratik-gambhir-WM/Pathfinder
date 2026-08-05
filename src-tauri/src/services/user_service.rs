use serde::Deserialize;
use serde_json::Value;

use crate::{
    core::nodes::user_node::UserNode,
    repository::user_repository::{
        create_user, get_user_by_email as fetch_user_by_email,
        get_wm_user_by_email as fetch_wm_user_by_email, upsert_wm_user, CreateUserRecord,
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

/// Persists a complete user through the parallel Helix flow.
pub async fn add_wm_user(state: &AppState, input: UserNode) -> Result<Value, String> {
    upsert_wm_user(state, input).await
}

/// Fetches the matching user from Helix.
pub async fn get_wm_user_by_email(state: &AppState, email: &str) -> Result<Value, String> {
    fetch_wm_user_by_email(state, email.trim()).await
}

pub fn get_user_by_email(state: &AppState, email: &str) -> Result<Option<User>, String> {
    fetch_user_by_email(state, email.trim())
}
