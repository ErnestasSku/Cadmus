use serde::Serialize;

use crate::{dto::StoryBlock, file_utils, state::AppStateHandle};
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub async fn save_file(
    story_blocks: Vec<StoryBlock>,
    state: State<'_, AppStateHandle>,
) -> Result<(), String> {
    let st = state.lock();
    if let Some(path) = &st.workspace_path {
        file_utils::files::write_cadmus_file(path.to_string(), story_blocks);
    }

    Ok(())
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
    st.workspace_path = Some(new_path);
    Ok(())
}
