#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod cadmus_app;
mod dto;
mod file_utils;
mod handle;
mod state;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(state::AppStateHandle::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            app::tauri::save_file,
            app::tauri::update_path,
            app::tauri::fetch_story_data,
            app::tauri::synchronize_story_data
        ])
        .setup(move |_app| {
            // let a = app.path_resolver().app_data_dir();
            // app.path_resolver().l

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
