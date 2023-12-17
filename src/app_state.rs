use std::sync::Arc;

use shuttle_persist::PersistInstance;

#[derive(Clone)]
pub struct AppState {
    pub persist: Arc<PersistInstance>,
    pub db: sqlx::PgPool,
}

impl AppState {
    pub fn new(persist: PersistInstance, db: sqlx::PgPool) -> Self {
        Self {
            persist: Arc::new(persist),
            db,
        }
    }
}
