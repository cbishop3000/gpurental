use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpListener;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let server_address = "ws://<SERVER_IP>:3000/ws"; // Update with the server's IP address
    
    let (mut ws_stream, _) = connect_async(server_address)
        .await
        .expect("Failed to connect to WebSocket");

    println!("Connected to WebSocket!");

    // Send a test message to the server
    let message = "Hello from worker!".to_string();
    ws_stream.send(Message::Text(message)).await.expect("Failed to send message");

    // Keep listening for messages (you can adjust this to handle real job processing)
    loop {
        let msg = ws_stream.next().await;

        match msg {
            Some(Ok(Message::Text(text))) => {
                println!("Received message: {}", text);
                // Handle the job or task here, such as processing GPU tasks
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
