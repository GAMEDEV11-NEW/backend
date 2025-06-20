use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};
use uuid::Uuid;
use chrono::Utc;

// Event-specific models for separate collections
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub token: i32,
    pub message: String,
    pub status: String,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfoEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub device_info: serde_json::Value,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionErrorEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub error_code: String,
    pub error_type: String,
    pub field: String,
    pub message: String,
    pub payload: bson::Document,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub email: Option<String>,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginSuccessEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub mobile_no: String,
    pub device_id: String,
    pub session_token: String,
    pub otp: i32,
    pub timestamp: DateTime,
    pub expires_at: DateTime,  // OTP expiration time (30 minutes from creation)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpVerificationEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub mobile_no: String,
    pub session_token: String,
    pub otp: String,
    pub is_success: bool,
    pub user_id: Option<String>,      // UUID v7
    pub user_number: Option<u64>,     // Sequential number
    pub jwt_token: Option<String>,    // JWT token after successful verification
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub user_id: String,              // UUID v7
    pub user_number: u64,             // Sequential number
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub email: Option<String>,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub user_id: String,              // UUID v7
    pub user_number: u64,             // Sequential number
    pub mobile_no: String,
    pub full_name: String,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSettingEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub socket_id: String,
    pub user_id: String,              // UUID v7
    pub user_number: u64,             // Sequential number
    pub mobile_no: String,
    pub language_code: String,
    pub language_name: String,
    pub region_code: Option<String>,
    pub timezone: Option<String>,
    pub user_preferences: serde_json::Value,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,           // UUID v7
    pub user_number: u64,          // Sequential number
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub language_code: Option<String>,
    pub language_name: Option<String>,
    pub region_code: Option<String>,
    pub timezone: Option<String>,
    pub user_preferences: Option<serde_json::Value>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_login_at: Option<DateTime>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSession {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub session_id: String,        // UUID v7
    pub user_id: String,           // UUID v7
    pub user_number: u64,          // Sequential number
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub session_token: String,
    pub otp: String,
    pub is_verified: bool,
    pub jwt_token: Option<String>, // JWT token after OTP verification
    pub created_at: DateTime,
    pub expires_at: DateTime,
    pub verified_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegister {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,           // UUID v7
    pub user_number: u64,          // Sequential number
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub state: Option<String>,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
    pub language_code: Option<String>,
    pub language_name: Option<String>,
    pub region_code: Option<String>,
    pub timezone: Option<String>,
    pub profile_data: Option<serde_json::Value>,
    pub user_preferences: Option<serde_json::Value>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_login_at: Option<DateTime>,
    pub total_logins: i32,         // Total number of logins
    pub is_active: bool,
}

// OTP verification result enum
#[derive(Debug, Clone, PartialEq)]
pub enum OtpVerificationResult {
    Success,    // OTP is valid
    Invalid,    // OTP is invalid
    Expired,    // OTP session has expired
    NotFound,   // No login session found
}

// Helper functions for creating new instances
impl ConnectEvent {
    pub fn new(socket_id: String, token: i32, message: String, status: String) -> Self {
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
    pub fn new(socket_id: String, device_info: serde_json::Value) -> Self {
        Self {
            id: None,
            socket_id,
            device_info,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
        }
    }
}

impl ConnectionErrorEvent {
    pub fn new(socket_id: String, error_code: String, error_type: String, field: String, message: String, payload: bson::Document) -> Self {
        Self {
            id: None,
            socket_id,
            error_code,
            error_type,
            field,
            message,
            payload,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
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
            mobile_no,
            device_id,
            session_token,
            otp,
            expires_at: DateTime::from_millis(Utc::now().timestamp_millis() + (30 * 60 * 1000)), // 30 minutes
        }
    }
}

impl OtpVerificationEvent {
    pub fn new(socket_id: String, mobile_no: String, session_token: String, otp: String, is_success: bool, user_id: Option<String>, user_number: Option<u64>) -> Self {
        Self {
            id: None,
            socket_id,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
            mobile_no,
            session_token,
            otp,
            is_success,
            user_id,
            user_number,
            jwt_token: None,
        }
    }
}

impl UserRegistrationEvent {
    pub fn new(socket_id: String, mobile_no: String, device_id: String, fcm_token: String, email: Option<String>) -> Self {
        let now = DateTime::from_millis(Utc::now().timestamp_millis());
        Self {
            id: None,
            socket_id,
            user_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_number: 1,
            mobile_no,
            device_id,
            fcm_token,
            email,
            timestamp: now,
        }
    }
}

impl UserProfileEvent {
    pub fn new(socket_id: String, mobile_no: String, full_name: String) -> Self {
        Self {
            id: None,
            socket_id,
            user_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_number: 1,
            mobile_no,
            full_name,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
        }
    }
}

impl LanguageSettingEvent {
    pub fn new(socket_id: String, mobile_no: String, language_code: String, language_name: String, region_code: Option<String>, timezone: Option<String>, user_preferences: serde_json::Value) -> Self {
        Self {
            id: None,
            socket_id,
            user_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_number: 1,
            mobile_no,
            language_code,
            language_name,
            region_code,
            timezone,
            user_preferences,
            timestamp: DateTime::from_millis(Utc::now().timestamp_millis()),
        }
    }
}

impl User {
    pub fn new(
        mobile_no: String,
        device_id: String,
        fcm_token: String,
        email: Option<String>,
        user_number: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            user_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_number,
            mobile_no,
            device_id,
            fcm_token,
            email,
            full_name: None,
            language_code: None,
            language_name: None,
            region_code: None,
            timezone: None,
            user_preferences: None,
            created_at: DateTime::from_millis(now.timestamp_millis()),
            updated_at: DateTime::from_millis(now.timestamp_millis()),
            last_login_at: None,
            is_active: true,
        }
    }

    pub fn update_login_info(&mut self, fcm_token: String) {
        self.fcm_token = fcm_token;
        self.last_login_at = Some(DateTime::from_millis(Utc::now().timestamp_millis()));
        self.updated_at = DateTime::from_millis(Utc::now().timestamp_millis());
    }
}

impl LoginSession {
    pub fn new(
        user_id: String,
        user_number: u64,
        mobile_no: String,
        device_id: String,
        fcm_token: String,
        session_token: String,
        otp: String,
    ) -> Self {
        let now = DateTime::from_millis(Utc::now().timestamp_millis());
        let expires_at = DateTime::from_millis(Utc::now().timestamp_millis() + (30 * 60 * 1000)); // 30 minutes
        Self {
            id: None,
            session_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_id,
            user_number,
            mobile_no,
            device_id,
            fcm_token,
            session_token,
            otp,
            is_verified: false,
            jwt_token: None,
            created_at: now,
            expires_at,
            verified_at: None,
        }
    }
    
    pub fn mark_verified(&mut self, jwt_token: String) {
        self.is_verified = true;
        self.jwt_token = Some(jwt_token);
        self.verified_at = Some(DateTime::from_millis(Utc::now().timestamp_millis()));
    }
}

impl UserRegister {
    pub fn new(
        mobile_no: String,
        device_id: String,
        fcm_token: String,
        email: Option<String>,
        user_number: u64,
    ) -> Self {
        let now = DateTime::from_millis(Utc::now().timestamp_millis());
        Self {
            id: None,
            user_id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_number,
            mobile_no,
            device_id,
            fcm_token,
            email,
            full_name: None,
            state: None,
            referral_code: None,
            referred_by: None,
            language_code: None,
            language_name: None,
            region_code: None,
            timezone: None,
            profile_data: None,
            user_preferences: None,
            created_at: now,
            updated_at: now,
            last_login_at: Some(now),
            total_logins: 0,
            is_active: true,
        }
    }
    
    pub fn update_login_info(&mut self, fcm_token: String) {
        self.fcm_token = fcm_token;
        self.last_login_at = Some(DateTime::from_millis(Utc::now().timestamp_millis()));
        self.updated_at = DateTime::from_millis(Utc::now().timestamp_millis());
    }
} 