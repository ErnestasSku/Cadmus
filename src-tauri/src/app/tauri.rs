use crate::{cadmus_app::CadmusApp, dto::StoryBlocks};
use tauri::AppHandle;

#[tauri::command]
pub async fn save_file(story_blocks: StoryBlocks, app: AppHandle) -> Result<(), String> {
    app.save_file(story_blocks)
}

#[tauri::command]
pub async fn load_file(story_path: String, app: AppHandle) -> Result<StoryBlocks, String> {
    app.load_file(&story_path)
}

#[tauri::command]
pub async fn synchronize_story_data(data: StoryBlocks, app: AppHandle) -> Result<(), String> {
    app.synchronize_story_data(data)
}

#[tauri::command]
pub async fn fetch_story_data(app: AppHandle) -> Result<String, String> {
    app.fetch_story_data()
}

#[tauri::command]
pub async fn update_path(new_path: String, app: AppHandle) -> Result<(), String> {
    app.update_path(new_path)
}
