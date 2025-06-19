use socketioxide::extract::SocketRef;
use serde_json::json;
use chrono::Utc;
use rand::Rng;
use tracing::info;
use std::sync::Arc;
use crate::database::service::DataService;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn send_connect_response(socket: &SocketRef, data_service: Arc<DataService>) {
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
        
        // Store connect event in MongoDB
        let _ = data_service.store_connect_event(&socket.id.to_string(), token, "Welcome to the Game Admin Server!", "connected").await;
        
        // Send as connect response (using a custom event that appears as connect response)
        let _ = socket.emit("connect_response", connect_response);
        info!(
            "✅ Sent connect response to socket: {} with token: {}",
            socket.id, token
        );
    }
} 