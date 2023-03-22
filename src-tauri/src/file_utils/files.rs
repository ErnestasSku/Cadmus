use crate::dto::*;
use std::fs;

pub fn write_cadmus_file(path: String, data: Vec<StoryBlock>) {
    let _ = fs::write(path, serde_json::to_string(&data).unwrap());
}
