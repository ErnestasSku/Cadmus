#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod dto;
mod state;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, app::tauri::save_file])
        .setup(move |_app| {
            // let a = app.path_resolver().app_data_dir();
            // app.path_resolver().l

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
