use crate::{dto::StoryBlock, file_utils, state};

pub trait CadmusApp {
    fn state(&self) -> state::AppStateHandle;
    fn save_file(&self, story_blocks: Vec<StoryBlock>) -> Result<(), String>;
    fn synchronize_story_data(&self, data: Vec<StoryBlock>) -> Result<(), String>;
    fn fetch_story_data(&self) -> Result<String, String>;
    fn update_path(&self, new_path: String) -> Result<(), String>;
}

impl CadmusApp for tauri::AppHandle {
    fn state(&self) -> state::AppStateHandle {
        let state = tauri::Manager::state::<state::AppStateHandle>(self);
        state::AppStateHandle::clone(&state)
    }

    fn save_file(&self, story_blocks: Vec<StoryBlock>) -> Result<(), String> {
        let state_handle = self.state();
        let state = state_handle.lock();
        if let Some(path) = &state.workspace_path {
            file_utils::files::write_cadmus_file(path.to_string(), story_blocks);
        }

        Ok(())
    }

    fn synchronize_story_data(&self, data: Vec<StoryBlock>) -> Result<(), String> {
        let state_handle = self.state();
        let mut state = state_handle.lock();
        state.story_data = data;
        Ok(())
    }

    fn fetch_story_data(&self) -> Result<String, String> {
        let state_handle = self.state();
        let state = state_handle.lock();
        let data = &state.story_data;

        serde_json::to_string(data).map_err(|x| format!("{x:?}"))
    }

    fn update_path(&self, new_path: String) -> Result<(), String> {
        let state_handle = self.state();
        let mut state = state_handle.lock();
        state.update_path(new_path);
        Ok(())
    }
}
