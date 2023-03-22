use crate::{dto::StoryBlock, file_utils, state};

pub trait CadmusApp {
    fn state(&self) -> state::AppStateHandle;
    fn save_file(&self, story_blocks: Vec<StoryBlock>) -> Result<(), String>;
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
}
