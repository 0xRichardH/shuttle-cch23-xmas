use std::sync::{Arc, Mutex};

use shuttle_persist::PersistInstance;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub persist: Arc<PersistInstance>,
    pub db: sqlx::PgPool,
    pub chatroom_broadcaster: broadcast::Sender<ChatroomMessage>,
    pub chatroom_counter: Arc<Mutex<u64>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChatroomMessageBody {
    pub user: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ChatroomMessage {
    pub room: u64,
    pub body: ChatroomMessageBody,
}

impl AppState {
    pub fn new(persist: PersistInstance, db: sqlx::PgPool) -> Self {
        let (chatroom_broadcaster, _) = broadcast::channel(1024);
        Self {
            persist: Arc::new(persist),
            db,
            chatroom_broadcaster,
            chatroom_counter: Arc::new(Mutex::new(0)),
        }
    }
}
