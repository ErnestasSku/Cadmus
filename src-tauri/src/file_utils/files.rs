use crate::dto::*;
use std::fs;

pub fn write_cadmus_file(path: String, data: StoryBlocks) -> Result<(), String> {
    fs::write(
        path,
        serde_json::to_string(&data).map_err(|x| x.to_string())?,
    )
    .map_err(|x| x.to_string())
}

pub fn read_cadmus_file(path: &str) -> Result<StoryBlocks, String> {
    let content = fs::read(path);
    match content {
        Ok(content) => serde_json::from_slice::<StoryBlocks>(&content).map_err(|x| x.to_string()),
        Err(error) => Err(error.to_string()),
    }
}
