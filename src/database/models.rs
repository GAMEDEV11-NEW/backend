use serde::{Deserialize, Serialize};
use bson::{DateTime, oid::ObjectId};
use chrono::Utc;

// Event-specific models for separate collections
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub token: i64,
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfoEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub device_id: String,
    pub device_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionErrorEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub status: String,
    pub error_code: String,
    pub error_type: String,
    pub field: String,
    pub message: String,
    pub details: bson::Document,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginSuccessEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub status: String,
    pub message: String,
    pub mobile_no: String,
    pub device_id: String,
    pub session_token: String,
    pub otp: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtpVerificationEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub timestamp: DateTime,
    pub mobile_no: String,
    pub session_token: String,
    pub otp: String,
    pub status: String,
    pub message: String,
    pub is_success: bool,
    pub attempts_count: i32,
}

// Helper functions for creating new instances
impl ConnectEvent {
    pub fn new(socket_id: String, token: i64, message: String, status: String) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            token,
            message,
            status,
        }
    }
}

impl DeviceInfoEvent {
    pub fn new(socket_id: String, device_id: String, device_type: String) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            device_id,
            device_type,
            manufacturer: None,
            model: None,
            firmware_version: None,
            capabilities: None,
        }
    }
}

impl ConnectionErrorEvent {
    pub fn new(socket_id: String, error_code: String, error_type: String, field: String, message: String, details: bson::Document) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            status: "error".to_string(),
            error_code,
            error_type,
            field,
            message,
            details,
        }
    }
}

impl LoginEvent {
    pub fn new(socket_id: String, mobile_no: String, device_id: String, fcm_token: String) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            mobile_no,
            device_id,
            fcm_token,
            email: None,
        }
    }
}

impl LoginSuccessEvent {
    pub fn new(socket_id: String, mobile_no: String, device_id: String, session_token: String, otp: i32) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            status: "success".to_string(),
            message: "Login successful".to_string(),
            mobile_no,
            device_id,
            session_token,
            otp,
        }
    }
}

impl OtpVerificationEvent {
    pub fn new(socket_id: String, mobile_no: String, session_token: String, otp: String, is_success: bool, attempts_count: i32) -> Self {
        let (status, message) = if is_success {
            ("success".to_string(), "OTP verification successful".to_string())
        } else {
            ("error".to_string(), "OTP verification failed".to_string())
        };
        
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            mobile_no,
            session_token,
            otp,
            status,
            message,
            is_success,
            attempts_count,
        }
    }
} 