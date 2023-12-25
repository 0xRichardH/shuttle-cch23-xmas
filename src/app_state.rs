use std::sync::{atomic::AtomicU64, Arc};

use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub secrect_store: Arc<SecretStore>,
    pub persist: Arc<PersistInstance>,
    pub db: sqlx::PgPool,
    pub chatroom_broadcaster: broadcast::Sender<ChatroomMessage>,
    pub chatroom_counter: Arc<AtomicU64>,
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
    pub fn new(secrect_store: SecretStore, persist: PersistInstance, db: sqlx::PgPool) -> Self {
        let (chatroom_broadcaster, _) = broadcast::channel(1024);
        Self {
            secrect_store: Arc::new(secrect_store),
            persist: Arc::new(persist),
            db,
            chatroom_broadcaster,
            chatroom_counter: Arc::new(AtomicU64::new(0)),
        }
    }
}
