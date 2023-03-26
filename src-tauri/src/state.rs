use serde::{Deserialize, Serialize};

use crate::{dto::StoryBlock, handle::Handle};

pub type AppStateHandle = Handle<AppState>;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub workspace_path: Option<String>,
    pub story_data: Vec<StoryBlock>,
}

impl AppState {
    pub fn update_path(&mut self, new_path: String) {
        self.workspace_path = Some(new_path);
    }
}
