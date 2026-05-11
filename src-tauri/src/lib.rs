// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod parsers;

use tauri::menu::{AboutMetadataBuilder, MenuBuilder, SubmenuBuilder};

const APP_NAME: &str = "Pathfinder";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn build_app_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<tauri::menu::Menu<R>> {
    let about_metadata = AboutMetadataBuilder::new()
        .name(Some(APP_NAME))
        .version(Some(app.package_info().version.to_string()))
        .build();

    let app_menu = SubmenuBuilder::new(app, APP_NAME)
        .about_with_text(format!("About {APP_NAME}"), Some(about_metadata))
        .separator()
        .services()
        .separator()
        .hide_with_text(format!("Hide {APP_NAME}"))
        .hide_others()
        .separator()
        .quit_with_text(format!("Quit {APP_NAME}"))
        .build()?;

    let file_menu = SubmenuBuilder::new(app, "File").close_window().build()?;

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let view_menu = SubmenuBuilder::new(app, "View").fullscreen().build()?;

    let window_menu = SubmenuBuilder::new(app, "Window")
        .minimize()
        .maximize()
        .separator()
        .close_window()
        .build()?;

    let help_menu = SubmenuBuilder::new(app, "Help").build()?;

    MenuBuilder::new(app)
        .item(&app_menu)
        .item(&file_menu)
        .item(&edit_menu)
        .item(&view_menu)
        .item(&window_menu)
        .item(&help_menu)
        .build()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .setup(|app| {
            let menu = build_app_menu(&app.handle())?;
            app.set_menu(menu)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
