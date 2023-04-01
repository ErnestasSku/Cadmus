use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoryBlock {
    index: i32,
    top: f32,
    left: f32,
    connections: Vec<Connection>,
    #[serde(default)]
    initializing: bool,
    #[serde(default)]
    story_id: String,
    #[serde(default)]
    story_content: String,
}

pub type StoryBlocks = Vec<StoryBlock>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    index: i32,
    connected_element_id: i32,
    path_label: String,
    path_description: String,
    start_x: f32,
    start_y: f32,
    end_y: f32,
    end_x: f32,
    empty: bool,
    connected: bool,
    visible: bool,
}
