use tracing::{info, warn};
use crate::database::{models::*, repository::*};

pub struct DataService {
    connect_repo: ConnectEventRepository,
    device_info_repo: DeviceInfoEventRepository,
    connection_error_repo: ConnectionErrorEventRepository,
    login_repo: LoginEventRepository,
    login_success_repo: LoginSuccessEventRepository,
    otp_verification_repo: OtpVerificationEventRepository,
}

impl DataService {
    pub fn new() -> Self {
        Self {
            connect_repo: ConnectEventRepository::new(),
            device_info_repo: DeviceInfoEventRepository::new(),
            connection_error_repo: ConnectionErrorEventRepository::new(),
            login_repo: LoginEventRepository::new(),
            login_success_repo: LoginSuccessEventRepository::new(),
            otp_verification_repo: OtpVerificationEventRepository::new(),
        }
    }
    
    // Event-specific storage methods
    pub async fn store_connect_event(&self, socket_id: &str, token: i64, message: &str, status: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event = ConnectEvent::new(socket_id.to_string(), token, message.to_string(), status.to_string());
        let _ = self.connect_repo.store_connect_event(event).await?;
        Ok(())
    }
    
    pub async fn store_device_info_event(&self, socket_id: &str, device_data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let device_id = device_data["device_id"].as_str().unwrap_or("unknown");
        let device_type = device_data["device_type"].as_str().unwrap_or("unknown");
        
        let mut event = DeviceInfoEvent::new(socket_id.to_string(), device_id.to_string(), device_type.to_string());
        
        // Add optional fields if present
        if let Some(manufacturer) = device_data["manufacturer"].as_str() {
            event.manufacturer = Some(manufacturer.to_string());
        }
        if let Some(model) = device_data["model"].as_str() {
            event.model = Some(model.to_string());
        }
        if let Some(firmware) = device_data["firmware_version"].as_str() {
            event.firmware_version = Some(firmware.to_string());
        }
        if let Some(capabilities) = device_data["capabilities"].as_array() {
            event.capabilities = Some(capabilities.iter()
                .filter_map(|c| c.as_str().map(|s| s.to_string()))
                .collect());
        }
        
        let _ = self.device_info_repo.store_device_info_event(event).await?;
        Ok(())
    }
    
    pub async fn store_connection_error_event(&self, socket_id: &str, error_code: &str, error_type: &str, field: &str, message: &str, details: bson::Document) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event = ConnectionErrorEvent::new(
            socket_id.to_string(),
            error_code.to_string(),
            error_type.to_string(),
            field.to_string(),
            message.to_string(),
            details,
        );
        let _ = self.connection_error_repo.store_connection_error_event(event).await?;
        Ok(())
    }
    
    pub async fn store_login_event(&self, socket_id: &str, mobile_no: &str, device_id: &str, fcm_token: &str, email: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut event = LoginEvent::new(socket_id.to_string(), mobile_no.to_string(), device_id.to_string(), fcm_token.to_string());
        event.email = email.map(|e| e.to_string());
        let _ = self.login_repo.store_login_event(event).await?;
        Ok(())
    }
    
    pub async fn store_login_success_event(&self, socket_id: &str, mobile_no: &str, device_id: &str, session_token: &str, otp: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event = LoginSuccessEvent::new(socket_id.to_string(), mobile_no.to_string(), device_id.to_string(), session_token.to_string(), otp);
        let _ = self.login_success_repo.store_login_success_event(event).await?;
        Ok(())
    }
    
    // OTP verification methods
    pub async fn verify_otp(&self, socket_id: &str, mobile_no: &str, session_token: &str, otp: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Get the attempts count for this session
        let attempts_count = self.otp_verification_repo.get_verification_attempts_count(mobile_no, session_token).await?;
        
        // Check if maximum attempts exceeded (5 attempts)
        if attempts_count >= 5 {
            let event = OtpVerificationEvent::new(
                socket_id.to_string(),
                mobile_no.to_string(),
                session_token.to_string(),
                otp.to_string(),
                false,
                attempts_count + 1
            );
            let _ = self.otp_verification_repo.store_otp_verification_event(event).await?;
            warn!("❌ OTP verification failed: Maximum attempts exceeded for mobile: {}", mobile_no);
            return Ok(false);
        }
        
        // Find the login success event to get the original OTP
        let login_success = self.login_success_repo.find_login_success_by_mobile_and_session(mobile_no, session_token).await?;
        
        match login_success {
            Some(login_event) => {
                // Compare the provided OTP with the stored OTP
                let stored_otp = login_event.otp.to_string();
                let is_success = otp == stored_otp;
                
                // Store the verification attempt
                let event = OtpVerificationEvent::new(
                    socket_id.to_string(),
                    mobile_no.to_string(),
                    session_token.to_string(),
                    otp.to_string(),
                    is_success,
                    attempts_count + 1
                );
                let _ = self.otp_verification_repo.store_otp_verification_event(event).await?;
                
                if is_success {
                    info!("✅ OTP verification successful for mobile: {}", mobile_no);
                } else {
                    warn!("❌ OTP verification failed for mobile: {} (attempt {})", mobile_no, attempts_count + 1);
                }
                
                Ok(is_success)
            }
            None => {
                // No login success event found
                let event = OtpVerificationEvent::new(
                    socket_id.to_string(),
                    mobile_no.to_string(),
                    session_token.to_string(),
                    otp.to_string(),
                    false,
                    attempts_count + 1
                );
                let _ = self.otp_verification_repo.store_otp_verification_event(event).await?;
                warn!("❌ OTP verification failed: No login session found for mobile: {}", mobile_no);
                Ok(false)
            }
        }
    }
    
    pub async fn get_otp_attempts_count(&self, mobile_no: &str, session_token: &str) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        self.otp_verification_repo.get_verification_attempts_count(mobile_no, session_token).await
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub total_users: i32,
    pub active_sessions: i32,
    pub server_load: f64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
} 