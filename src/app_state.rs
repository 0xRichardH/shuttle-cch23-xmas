use std::sync::Arc;

use shuttle_persist::PersistInstance;

#[derive(Clone)]
pub struct AppState {
    pub persist: Arc<PersistInstance>,
}

impl AppState {
    pub fn new(persist: PersistInstance) -> Self {
        Self {
            persist: Arc::new(persist),
        }
    }
}
