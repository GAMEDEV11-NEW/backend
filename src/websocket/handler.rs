use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Send a welcome message
    if let Err(e) = socket.send(Message::Text(String::from("Welcome to the WebSocket server!"))).await {
        println!("Error sending welcome message: {}", e);
        return;
    }

    // Handle incoming messages
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("Received message: {}", text);
                    // Echo the message back
                    if let Err(e) = socket.send(Message::Text(text)).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    break;
                }
                _ => {}
            }
        } else {
            println!("Client disconnected");
            break;
        }
    }
} 