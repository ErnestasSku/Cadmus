use serde::Serialize;

use crate::dto::StoryBlock;

#[tauri::command]
pub async fn save_file(story_blocks: Vec<StoryBlock>) -> Result<String, String> {
    println!("{:?}", story_blocks);

    Ok(serde_json::to_string(&story_blocks).unwrap())
}
