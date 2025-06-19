use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tracing::info;
use serde_json::json;
use chrono::Utc;
use rand::Rng;

pub struct SocketManager;

impl SocketManager {
    pub fn register_handlers(io: &SocketIo) {
        io.ns("/", |socket: SocketRef| async move {
            info!("Socket connected: {}", socket.id);
            
            // Send welcome data immediately after connection (appears as connect response)
            Self::send_connect_response(&socket).await;
            
            // Handle disconnect event
            socket.on("disconnect", |socket: SocketRef| async move {
                info!("Socket disconnected: {}", socket.id);
            });

            // Handle custom events
            Self::register_custom_handlers(socket).await;
        });
    }

    async fn send_connect_response(socket: &SocketRef) {
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
        info!("✅ Sent connect response to socket: {} with token: {}", socket.id, token);
    }

    async fn send_welcome_message(socket: &SocketRef) {
        // This method is now replaced by send_connect_response
        Self::send_connect_response(socket).await;
    }

    async fn handle_connection(socket: &SocketRef) {
        // This method is now replaced by send_welcome_message
        // Keeping for backward compatibility if needed
        Self::send_welcome_message(socket).await;
    }

    async fn register_custom_handlers(socket: SocketRef) {
        // Handle device info events
        socket.on(
            "device:info",
            |socket: SocketRef, Data::<serde_json::Value>(data)| async move {
                info!("Received device info from {}: {:?}", socket.id, data);
                
                // Create structured acknowledgment response
                let ack_response = json!({
                    "status": "success",
                    "message": "Device info received",
                    "timestamp": Utc::now().to_rfc3339(),
                    "socket_id": socket.id.to_string(),
                    "data_received": data
                });
                
                socket.emit("device:info:ack", ack_response);
                info!("Sent device info acknowledgment to: {}", socket.id);
            },
        );

        // Handle device status events
        socket.on(
            "device:status",
            |socket: SocketRef, Data::<serde_json::Value>(data)| async move {
                info!("Received device status from {}: {:?}", socket.id, data);
                
                // Create structured acknowledgment response
                let ack_response = json!({
                    "status": "success",
                    "message": "Device status received",
                    "timestamp": Utc::now().to_rfc3339(),
                    "socket_id": socket.id.to_string(),
                    "data_received": data
                });
                
                socket.emit("device:status:ack", ack_response);
                info!("Sent device status acknowledgment to: {}", socket.id);
            },
        );

        // Add more custom event handlers here
    }
} 