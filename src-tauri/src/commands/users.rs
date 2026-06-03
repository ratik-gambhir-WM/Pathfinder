#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn create_wm_user(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
