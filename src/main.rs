use axum::{Router, routing::get, response::IntoResponse};
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::Server;
use tokio_tungstenite::tungstenite::protocol::Message as TungsteniteMessage;  // alias for tokio_tungstenite's Message
use futures_util::{SinkExt, StreamExt};

#[derive(Debug, Default)]
struct AppState {
    // Your shared application state
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        match socket.next().await {
            Some(Ok(axum::extract::ws::Message::Text(text))) => {  // Explicitly use axum's Message
                println!("Received: {}", text);
                // Process the received job or message
                // You can implement your job processing logic here
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let app = Router::new()
        .route("/ws", get(websocket_handler));

    println!("🚀 WebSocket Server running on http://{}", addr);

    if let Err(e) = axum::Server::bind(&addr).serve(app.into_make_service()).await {
        eprintln!("Server error: {:?}", e);
    }
}
