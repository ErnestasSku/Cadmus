use parking_lot::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

pub struct Handle<T>(Arc<Mutex<T>>);

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock()
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub type AppStateHandle = Handle<AppState>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppState {
    workspace_path: String,
}
