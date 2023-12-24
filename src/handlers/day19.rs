use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::app_state::{AppState, ChatroomMessage, ChatroomMessageBody};

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
    let chatroom_broadcaster = state.chatroom_broadcaster;
    ws.on_upgrade(move |socket| handle_chatroom_socket(socket, room_id, user, chatroom_broadcaster))
}

async fn handle_chatroom_socket(
    socket: WebSocket,
    room_id: u64,
    user: String,
    chatroom_broadcaster: broadcast::Sender<ChatroomMessage>,
) {
    let rx = chatroom_broadcaster.subscribe();

    let (sender, receiver) = socket.split();
    let mut send_task = tokio::spawn(read_from_chatroom(
        receiver,
        room_id,
        user.clone(),
        chatroom_broadcaster,
    ));
    let mut recv_task = tokio::spawn(write_to_chatroom(sender, room_id, user.clone(), rx));

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

#[derive(Debug, Deserialize)]
struct RecvMsg {
    message: String,
}

async fn write_to_chatroom(
    mut sender: SplitSink<WebSocket, Message>,
    room_id: u64,
    user: String,
    mut rx: broadcast::Receiver<ChatroomMessage>,
) {
    while let Ok(send_msg) = rx.recv().await {
        if send_msg.room != room_id {
            continue;
        }

        let Ok(body) = serde_json::to_string(&send_msg.body) else {
            tracing::error!(
                "Room {}: User {}: Failed to serialize message",
                room_id,
                user
            );
            continue;
        };
        if let Err(e) = sender.send(Message::Text(body)).await {
            tracing::error!(
                "Room {}: User {}: Failed to send message: {:?}",
                room_id,
                user,
                e
            )
        }
    }
}

async fn read_from_chatroom(
    mut receiver: SplitStream<WebSocket>,
    room_id: u64,
    user: String,
    tx: broadcast::Sender<ChatroomMessage>,
) {
    while let Some(msg) = receiver.next().await {
        if let Err(e) = msg {
            tracing::error!(
                "Room {}: User {}: Error receiving message: {:?}",
                room_id,
                user.clone(),
                e
            );
            continue;
        }

        tracing::info!(
            "Room {}: Received message from User {}: {:?}",
            room_id,
            user.clone(),
            msg
        );

        if let Message::Text(t) = msg.unwrap() {
            let Ok(recv_msg) = serde_json::from_str::<RecvMsg>(t.as_str()) else {
                tracing::error!(
                    "Room {}: User {}: Failed to parse message: {:?}",
                    room_id,
                    user.clone(),
                    t
                );
                continue;
            };

            tracing::debug!(
                "Room {}: User {}: Received message: {:?}",
                room_id,
                user.clone(),
                recv_msg
            );

            let body = ChatroomMessageBody {
                user: user.clone(),
                message: recv_msg.message,
            };
            let send_msg = ChatroomMessage {
                room: room_id,
                body,
            };
            if let Err(e) = tx.send(send_msg) {
                tracing::error!(
                    "Room {}: User {}: Failed to send message: {:?}",
                    room_id,
                    user.clone(),
                    e
                )
            }
        }
    }
}
