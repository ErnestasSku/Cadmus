use crate::{
    dto::{StoryBlock, StoryBlocks},
    file_utils, state,
};

pub trait CadmusApp {
    fn state(&self) -> state::AppStateHandle;
    fn save_file(&self, story_blocks: Vec<StoryBlock>) -> Result<(), String>;
    fn load_file(&self, story_path: &String) -> Result<StoryBlocks, String>;
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

    fn load_file(&self, story_path: &String) -> Result<StoryBlocks, String> {
        let state_handle = self.state();
        let mut state = state_handle.lock();

        match file_utils::files::read_cadmus_file(&story_path) {
            Ok(story) => {
                state.story_data = story.clone();
                Ok(story)
            }
            Err(err) => Err(err),
        }
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
