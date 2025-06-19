use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use serde_json::json;
use tracing::{info, warn};
use rand::Rng;
use std::sync::Arc;
use bson::to_document;

use crate::managers::connection::ConnectionManager;
use crate::managers::validation::ValidationManager;
use crate::database::service::DataService;

pub struct EventManager;

impl EventManager {
    pub fn register_custom_events(io: &SocketIo, data_service: Arc<DataService>) {
        io.ns("/", move |socket: SocketRef| {
            let data_service = data_service.clone();
            async move {
                info!("🔌 New client connected: {}", socket.id);
                ConnectionManager::send_connect_response(&socket, data_service.clone()).await;

                // Handle device info event
                let ds1 = data_service.clone();
                socket.on("device:info", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds1 = ds1.clone();
                    async move {
                        info!("📱 Received device info from {}: {:?}", socket.id, data);
                        let _ = ds1.store_device_info_event(&socket.id.to_string(), &data).await;
                        match ValidationManager::validate_device_info(&data) {
                            Ok(_) => {
                                let ack_response = json!({
                                    "status": "success",
                                    "message": "Device info received and validated",
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "device:info:ack"
                                });
                                let _ = socket.emit("device:info:ack", ack_response);
                                info!("Sent device info acknowledgment to: {}", socket.id);
                            }
                            Err(error_details) => {
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
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds1.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("Sent connection error to {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle login event
                let ds2 = data_service.clone();
                socket.on("login", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds2 = ds2.clone();
                    async move {
                        info!("🔐 Received login request from {}: {:?}", socket.id, data);
                        let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                        let device_id = data["device_id"].as_str().unwrap_or("unknown");
                        let fcm_token = data["fcm_token"].as_str().unwrap_or("unknown");
                        let email = data["email"].as_str();
                        let _ = ds2.store_login_event(&socket.id.to_string(), mobile_no, device_id, fcm_token, email).await;
                        match ValidationManager::validate_login_data(&data) {
                            Ok(_) => {
                                let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                let device_id = data["device_id"].as_str().unwrap_or("unknown");
                                let session_token = rand::thread_rng().gen_range(100000000..999999999).to_string();
                                let otp = rand::thread_rng().gen_range(100000..999999);
                                let login_response = json!({
                                    "status": "success",
                                    "message": "Login successful",
                                    "mobile_no": mobile_no,
                                    "device_id": device_id,
                                    "session_token": session_token,
                                    "otp": otp,
                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                    "socket_id": socket.id.to_string(),
                                    "event": "login:success"
                                });
                                let store_result = ds2.store_login_success_event(&socket.id.to_string(), mobile_no, device_id, &session_token, otp).await;
                                if let Err(e) = store_result {
                                    warn!("Failed to store login success event: {}", e);
                                }
                                let _ = socket.emit("login:success", login_response);
                                info!("✅ Login successful for mobile: {} (device: {}, socket: {})", mobile_no, device_id, socket.id);
                            }
                            Err(error_details) => {
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
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds2.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("❌ Login failed for socket {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle OTP verification event
                let ds3 = data_service.clone();
                socket.on("verify:otp", move |socket: SocketRef, Data::<serde_json::Value>(data)| {
                    let ds3 = ds3.clone();
                    async move {
                        info!("🔢 Received OTP verification request from {}: {:?}", socket.id, data);
                        match ValidationManager::validate_otp_data(&data) {
                            Ok(_) => {
                                let mobile_no = data["mobile_no"].as_str().unwrap_or("unknown");
                                let otp = data["otp"].as_str().unwrap_or("unknown");
                                let session_token = data["session_token"].as_str().unwrap_or("unknown");
                                
                                // Verify the OTP
                                let verify_result = ds3.verify_otp(&socket.id.to_string(), mobile_no, session_token, otp).await;
                                match verify_result {
                                    Ok(is_success) => {
                                        if is_success {
                                            let success_response = json!({
                                                "status": "success",
                                                "message": "OTP verification successful. Authentication completed.",
                                                "mobile_no": mobile_no,
                                                "session_token": session_token,
                                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                                "socket_id": socket.id.to_string(),
                                                "event": "otp:verified"
                                            });
                                            let _ = socket.emit("otp:verified", success_response);
                                            info!("✅ OTP verification successful for mobile: {} (socket: {})", mobile_no, socket.id);
                                        } else {
                                            let attempts_result = ds3.get_otp_attempts_count(mobile_no, session_token).await;
                                            let attempts_count = attempts_result.unwrap_or(0);
                                            let remaining_attempts = 5 - attempts_count;
                                            
                                            if remaining_attempts <= 0 {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "MAX_ATTEMPTS_EXCEEDED",
                                                    "error_type": "OTP_ERROR",
                                                    "field": "otp",
                                                    "message": "Maximum OTP verification attempts exceeded. Please login again.",
                                                    "details": json!({
                                                        "max_attempts": 5,
                                                        "attempts_used": attempts_count,
                                                        "remaining_attempts": 0
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "connection_error"
                                                });
                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds3.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "MAX_ATTEMPTS_EXCEEDED",
                                                    "OTP_ERROR",
                                                    "otp",
                                                    "Maximum OTP verification attempts exceeded. Please login again.",
                                                    payload_doc
                                                ).await;
                                                let _ = socket.emit("connection_error", error_response);
                                                info!("❌ OTP verification failed: Maximum attempts exceeded for mobile: {} (socket: {})", mobile_no, socket.id);
                                            } else {
                                                let error_response = json!({
                                                    "status": "error",
                                                    "error_code": "INVALID_OTP",
                                                    "error_type": "OTP_ERROR",
                                                    "field": "otp",
                                                    "message": "Invalid OTP. Please try again.",
                                                    "details": json!({
                                                        "max_attempts": 5,
                                                        "attempts_used": attempts_count,
                                                        "remaining_attempts": remaining_attempts
                                                    }),
                                                    "timestamp": chrono::Utc::now().to_rfc3339(),
                                                    "socket_id": socket.id.to_string(),
                                                    "event": "connection_error"
                                                });
                                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                                let _ = ds3.store_connection_error_event(
                                                    &socket.id.to_string(),
                                                    "INVALID_OTP",
                                                    "OTP_ERROR",
                                                    "otp",
                                                    "Invalid OTP. Please try again.",
                                                    payload_doc
                                                ).await;
                                                let _ = socket.emit("connection_error", error_response);
                                                info!("❌ OTP verification failed for mobile: {} (attempt {}/{}, socket: {})", mobile_no, attempts_count, 5, socket.id);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = e.to_string();
                                        let error_response = json!({
                                            "status": "error",
                                            "error_code": "VERIFICATION_ERROR",
                                            "error_type": "SYSTEM_ERROR",
                                            "field": "otp",
                                            "message": "OTP verification failed due to system error",
                                            "details": json!({
                                                "error": error_msg
                                            }),
                                            "timestamp": chrono::Utc::now().to_rfc3339(),
                                            "socket_id": socket.id.to_string(),
                                            "event": "connection_error"
                                        });
                                        let payload_doc = to_document(&error_response).unwrap_or_default();
                                        let _ = ds3.store_connection_error_event(
                                            &socket.id.to_string(),
                                            "VERIFICATION_ERROR",
                                            "SYSTEM_ERROR",
                                            "otp",
                                            "OTP verification failed due to system error",
                                            payload_doc
                                        ).await;
                                        let _ = socket.emit("connection_error", error_response);
                                        info!("❌ OTP verification system error for mobile: {} (socket: {}): {}", mobile_no, socket.id, error_msg);
                                    }
                                }
                            }
                            Err(error_details) => {
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
                                let payload_doc = to_document(&error_response).unwrap_or_default();
                                let _ = ds3.store_connection_error_event(
                                    &socket.id.to_string(),
                                    &error_details.code,
                                    &error_details.error_type,
                                    &error_details.field,
                                    &error_details.message,
                                    payload_doc
                                ).await;
                                let _ = socket.emit("connection_error", error_response);
                                info!("❌ OTP verification validation failed for socket {}: {:?}", socket.id, error_details);
                            }
                        }
                    }
                });

                // Handle disconnect event
                socket.on("disconnect", |socket: SocketRef| async move {
                    info!("🔌 Client disconnected: {}", socket.id);
                });
            }
        });
    }
} 