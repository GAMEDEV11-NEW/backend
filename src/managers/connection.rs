use socketioxide::extract::SocketRef;
use serde_json::json;
use chrono::Utc;
use rand::Rng;
use tracing::info;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn send_connect_response(socket: &SocketRef) {
        // Generate random token (6-digit number)
        let token = rand::thread_rng().gen_range(100000..999999);
        
        // Create structured JSON response that appears as connect response
        let connect_response = json!({
            "token": token,
            "message": "Welcome to the Game Admin Server!",
            "timestamp": Utc::now().to_rfc3339(),
            "socket_id": socket.id.to_string(),
            "status": "connected",
            "event": "connect"
        });
        
        // Log the connect response data
        info!("📨 Connect response data: {:?}", connect_response);
        
        // Send as connect response (using a custom event that appears as connect response)
        socket.emit("connect_response", connect_response);
        info!(
            "✅ Sent connect response to socket: {} with token: {}",
            socket.id, token
        );
    }
} 