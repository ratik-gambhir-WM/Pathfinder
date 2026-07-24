use tauri::State;

use crate::{
    services::user_service::{add_user, get_user_by_email as fetch_user_by_email, AddUserInput, User},
    state::AppState,
};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn create_user(state: State<'_, AppState>, input: AddUserInput) -> Result<User, String> {
    add_user(&state, input)
}

#[tauri::command]
pub fn user_exists_by_email(state: State<'_, AppState>, email: String) -> Result<bool, String> {
    Ok(fetch_user_by_email(&state, &email)?.is_some())
}

#[tauri::command]
pub fn get_user_by_email(state: State<'_, AppState>, email: String) -> Result<Option<User>, String> {
    fetch_user_by_email(&state, &email)
}

#[tauri::command]
pub fn create_wm_user(state: State<'_, AppState>, input: AddUserInput) -> Result<User, String> {
    add_user(&state, input)
}
