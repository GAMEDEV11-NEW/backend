use socketioxide::extract::SocketRef;
use serde_json::json;
use chrono::Utc;
use rand::Rng;
use tracing::{info, warn, error};
use std::sync::Arc;
use crate::database::service::DataService;

pub struct ConnectionManager;

impl ConnectionManager {
    /// Mark a socket as problematic for disconnection
    pub fn mark_problematic_socket(socket_id: &str) {
        // This would be called when a socket causes issues
        warn!("⚠️ Marking socket {} as problematic for disconnection", socket_id);
        
        // In a real implementation, you would store this in a global state
        // For now, we'll just log it
        error!("🔌 Socket {} marked for disconnection due to problematic behavior", socket_id);
    }

    /// Check if a socket should be disconnected
    pub fn should_disconnect_socket(socket_id: &str) -> bool {
        // This would check if the socket has been marked as problematic
        // For now, return false to avoid false positives
        false
    }

    pub async fn send_connect_response(socket: &SocketRef, data_service: Arc<DataService>) {
        // Generate random token (6-digit number)
        let token = rand::thread_rng().gen_range(100000..999999);
        
        // Create structured JSON response
        let connect_response = json!({
            "token": token,
            "message": "Welcome to the Game Admin Server!",
            "timestamp": Utc::now().to_rfc3339(),
            "socket_id": socket.id.to_string(),
            "status": "connected",
            "event": "connect",
            "server_info": {
                "version": "1.0.0",
                "heartbeat_interval": 60000,
                "ping_timeout": 60000,
                "max_payload": 1048576
            }
        });
        
        // Log the connect response data
        info!("📨 Connect response data: {:?}", connect_response);
        
        // Store connect event in MongoDB
        match data_service.store_connect_event(&socket.id.to_string(), token, "Welcome to the Game Admin Server!", "connected").await {
            Ok(_) => info!("📝 Stored connect event for socket: {}", socket.id),
            Err(e) => warn!("⚠️ Failed to store connect event for socket {}: {}", socket.id, e),
        }
        
        // Send connect response with proper error handling
        match socket.emit("connect_response", connect_response) {
            Ok(_) => info!("✅ Sent connect response to socket: {} with token: {}", socket.id, token),
            Err(e) => {
                error!("❌ Failed to send connect response to socket {}: {}", socket.id, e);
                // Mark socket as problematic if it fails to send messages
                Self::mark_problematic_socket(&socket.id.to_string());
                
                // Try sending a simple error message
                if let Err(e2) = socket.emit("error", json!({"message": "connection_failed", "socket_id": socket.id.to_string()})) {
                    error!("❌ Failed to send error message to socket {}: {}", socket.id, e2);
                }
            }
        }

        // Send initial heartbeat to establish connection health
        let heartbeat = json!({
            "type": "heartbeat",
            "timestamp": Utc::now().to_rfc3339(),
            "socket_id": socket.id.to_string()
        });
        
        match socket.emit("heartbeat", heartbeat) {
            Ok(_) => info!("💓 Sent initial heartbeat to socket: {}", socket.id),
            Err(e) => {
                warn!("⚠️ Failed to send initial heartbeat to socket {}: {}", socket.id, e);
                // Mark socket as problematic if heartbeat fails
                Self::mark_problematic_socket(&socket.id.to_string());
            }
        }
        
        // Send welcome message
        let welcome_message = json!({
            "type": "welcome",
            "message": "Welcome to Game Admin Server! You are now connected.",
            "socket_id": socket.id.to_string(),
            "timestamp": Utc::now().to_rfc3339()
        });
        
        match socket.emit("welcome", welcome_message) {
            Ok(_) => info!("👋 Sent welcome message to socket: {}", socket.id),
            Err(e) => {
                warn!("⚠️ Failed to send welcome message to socket {}: {}", socket.id, e);
                // Mark socket as problematic if welcome message fails
                Self::mark_problematic_socket(&socket.id.to_string());
            }
        }
    }
} 