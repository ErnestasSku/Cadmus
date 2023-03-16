use serde::Serialize;

use crate::dto::StoryBlock;
use crate::state::AppState;

#[tauri::command]
pub async fn save_file(story_blocks: Vec<StoryBlock>) -> Result<String, String> {
    println!("{:?}", story_blocks);

    let a = AppState {
        workspace_path: None,
    };

    Ok(serde_json::to_string(&a).unwrap())
}
