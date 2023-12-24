use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use crate::app_state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    tracing::info!("Serving websocket");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut is_game_started = false;

    while let Some(msg) = socket.recv().await {
        tracing::info!("Received message: {:?}", msg);
        let Ok(msg) = msg else {
            tracing::error!("Error receiving message: {:?}", msg);
            return;
        };

        if let Message::Close(c) = msg {
            tracing::info!("Received close message: {:?}", c);
            break;
        }

        if let Message::Text(t) = msg {
            match t.trim() {
                "serve" => {
                    is_game_started = true;
                }
                "ping" => {
                    if is_game_started {
                        let result = socket.send(Message::Text("pong".to_string())).await;
                        if let Err(e) = result {
                            tracing::error!("Error sending message: {:?}", e);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub async fn chatroom(
    Path((room_id, user)): Path<(u64, String)>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Serving chatroom {} for user {}", room_id, user);
    ws.on_upgrade(move |socket| handle_chatroom_socket(socket, room_id, user))
}

async fn handle_chatroom_socket(mut socket: WebSocket, room_id: u64, user: String) {
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(read_from_chatroom(receiver, room_id, user.clone()));
    let mut recv_task = tokio::spawn(write_to_chatroom(sender, room_id, user.clone()));

    tokio::select! {
        rv_a = &mut send_task => {
            match rv_a {
                Ok(_) => {
                    tracing::info!("Room {}: User {}: Finished sending messages", room_id, user.as_str());
                },
                Err(_) => {
                    tracing::error!("Room {}: User {}: Error sending messages", room_id, user.as_str());
                },
            }

        },

        rv_b = &mut recv_task => {
            match rv_b {
                Ok(_) => {
                    tracing::info!("Room {}: User {}: Finished receiving messages", room_id, user.as_str());
                },
                Err(_) => {
                    tracing::error!("Room {}: User {}: Error receiving messages", room_id, user.as_str());
                },
            }
        }
    }
}

async fn write_to_chatroom(sender: SplitSink<WebSocket, Message>, room_id: u64, user: String) {}

async fn read_from_chatroom(mut receiver: SplitStream<WebSocket>, room_id: u64, user: String) {
    while let Some(msg) = receiver.next().await {
        if let Err(e) = msg {
            tracing::error!(
                "Room {}: User {}: Error receiving message: {:?}",
                room_id,
                user,
                e
            );
            continue;
        }

        tracing::info!(
            "Room {}: Received message from User {}: {:?}",
            room_id,
            user,
            msg
        );
    }
}
