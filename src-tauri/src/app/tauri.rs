use serde::Serialize;

use crate::{dto::StoryBlock, file_utils, state::AppStateHandle};
use tauri::State;
// use crate::file::files;

use crate::state::AppState;

#[tauri::command]
pub async fn save_file(
    story_blocks: Vec<StoryBlock>,
    state: State<'_, AppStateHandle>,
) -> Result<(), String> {
    let ct = state.lock();
    if let Some(path) = &ct.workspace_path {
        file_utils::files::write_cadmus_file(path.to_string(), story_blocks);
    }

    Ok(())
}

// #[tauri::command]
// pub async fn save_file() -> Result<String, String> {
//     println!("{:?}", story_blocks);

//     Ok(serde_json::to_string(&a).unwrap())
// }

#[tauri::command]
pub async fn update_path(new_path: String, state: State<'_, AppStateHandle>) -> Result<(), String> {
    let mut st = state.lock();
    st.workspace_path = Some(new_path);
    Ok(())
}
