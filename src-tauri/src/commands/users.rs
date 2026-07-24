use tauri::State;

use crate::{
    commands::{CommandResult, CommandResultExt},
    services::user_service::{
        add_user, get_user_by_email as fetch_user_by_email, AddUserInput, User,
    },
    state::AppState,
};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn create_user(state: State<'_, AppState>, input: AddUserInput) -> CommandResult<User> {
    add_user(&state, input).command_context("create_user")
}

#[tauri::command]
pub fn user_exists_by_email(state: State<'_, AppState>, email: String) -> CommandResult<bool> {
    fetch_user_by_email(&state, &email)
        .map(|user| user.is_some())
        .command_context("user_exists_by_email")
}

#[tauri::command]
pub fn get_user_by_email(state: State<'_, AppState>, email: String) -> CommandResult<Option<User>> {
    fetch_user_by_email(&state, &email).command_context("get_user_by_email")
}

#[tauri::command]
pub fn create_wm_user(state: State<'_, AppState>, input: AddUserInput) -> CommandResult<User> {
    add_user(&state, input).command_context("create_wm_user")
}
