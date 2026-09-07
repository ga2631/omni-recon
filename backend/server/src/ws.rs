use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tracing::info;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("New WebSocket client connected for real-time alerts & progress");

    // Send initial welcome & status message
    let welcome_msg = serde_json::json!({
        "event": "connected",
        "message": "Connected to OmniRecon Live Stream"
    });

    if socket.send(Message::Text(welcome_msg.to_string())).await.is_err() {
        return;
    }

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if let Message::Text(text) = msg {
                // Echo or handle client heartbeats
                if text == "ping" {
                    let _ = socket.send(Message::Text("pong".to_string())).await;
                }
            }
        } else {
            break;
        }
    }
}
