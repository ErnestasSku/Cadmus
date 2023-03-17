use parking_lot::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

use crate::{dto::StoryBlock, handle::Handle};

pub type AppStateHandle = Handle<AppState>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub workspace_path: Option<String>,
    pub story_data: Vec<StoryBlock>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            workspace_path: None,
            story_data: Vec::new(),
        }
    }
}

impl AppState {}
