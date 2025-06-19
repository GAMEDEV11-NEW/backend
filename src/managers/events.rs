use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use serde_json::json;
use tracing::info;

use crate::managers::connection::ConnectionManager;
use crate::managers::validation::ValidationManager;

pub struct EventManager;

impl EventManager {
    pub fn register_custom_events(io: &SocketIo) {
        // Register handlers for the default namespace
        io.ns("/", |socket: SocketRef| async move {
            info!("🔌 New client connected: {}", socket.id);
            ConnectionManager::send_connect_response(&socket).await;

            // Handle device info event
            socket.on("device:info", |socket: SocketRef, Data::<serde_json::Value>(data)| async move {
                info!("📱 Received device info from {}: {:?}", socket.id, data);
                
                // Validate the device info data
                match ValidationManager::validate_device_info(&data) {
                    Ok(_) => {
                        // Send acknowledgment response
                        let ack_response = json!({
                            "status": "success",
                            "message": "Device info received and validated",
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "socket_id": socket.id.to_string(),
                            "event": "device:info:ack"
                        });
                        
                        socket.emit("device:info:ack", ack_response);
                        info!("Sent device info acknowledgment to: {}", socket.id);
                    }
                    Err(error_details) => {
                        // Send connection error with detailed JSON error information
                        let error_response = json!({
                            "status": "error",
                            "error_code": error_details.code,
                            "error_type": error_details.error_type,
                            "field": error_details.field,
                            "message": error_details.message,
                            "details": error_details.details,
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "socket_id": socket.id.to_string(),
                            "event": "connection_error"
                        });
                        
                        socket.emit("connection_error", error_response);
                        info!(
                            "Sent connection error to {}: {:?}",
                            socket.id, error_details
                        );
                    }
                }
            });

            // Handle disconnect event
            socket.on("disconnect", |socket: SocketRef| async move {
                info!("🔌 Client disconnected: {}", socket.id);
            });
        });
    }
} 