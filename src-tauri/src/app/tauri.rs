use serde::Serialize;

use crate::{cadmus_app::CadmusApp, dto::StoryBlock, file_utils, state::AppStateHandle};
use tauri::{AppHandle, State};

use crate::state::AppState;

type StoryBlocks = Vec<StoryBlock>;

#[tauri::command]
pub async fn save_file(story_blocks: StoryBlocks, app: AppHandle) -> Result<(), String> {
    app.save_file(story_blocks)
}

#[tauri::command]
pub async fn synchronize_story_data(
    data: Vec<StoryBlock>,
    state: State<'_, AppStateHandle>,
) -> Result<(), String> {
    let mut st = state.lock();
    st.story_data = data;
    Ok(())
}

#[tauri::command]
pub async fn fetch_story_data(state: State<'_, AppStateHandle>) -> Result<String, String> {
    let st = state.lock();
    Ok(serde_json::to_string(&st.story_data).unwrap())
}

#[tauri::command]
pub async fn update_path(new_path: String, state: State<'_, AppStateHandle>) -> Result<(), String> {
    let mut st = state.lock();
    st.update_path(new_path);
    Ok(())
}
