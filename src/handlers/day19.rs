use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    tracing::info!("Serving websocket");
    ws.on_upgrade(handler_socket)
}

async fn handler_socket(mut socket: WebSocket) {
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
