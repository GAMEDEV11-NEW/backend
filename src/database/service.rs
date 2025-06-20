use tracing::{info, error};
use crate::database::{models::*, repository::*, DatabaseManager};
use chrono;
use mongodb::{Database, Collection};
use bson::doc;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DataService {
    db: &'static Database,
    user_counter: Arc<Mutex<u64>>,
    connect_repo: ConnectEventRepository,
    device_info_repo: DeviceInfoEventRepository,
    connection_error_repo: ConnectionErrorEventRepository,
    login_repo: LoginEventRepository,
    login_success_repo: LoginSuccessEventRepository,
    otp_verification_repo: OtpVerificationEventRepository,
    language_setting_repo: LanguageSettingEventRepository,
    user_profile_repo: UserProfileEventRepository,
    user_register_repo: UserRegisterRepository,
}

impl DataService {
    pub fn new() -> Self {
        // Get the shared database instance
        let db = DatabaseManager::get_database();
        
        // Initialize user counter
        let user_counter = Arc::new(Mutex::new(0));
        
        Self {
            db,
            user_counter,
            connect_repo: ConnectEventRepository::new(),
            device_info_repo: DeviceInfoEventRepository::new(),
            connection_error_repo: ConnectionErrorEventRepository::new(),
            login_repo: LoginEventRepository::new(),
            login_success_repo: LoginSuccessEventRepository::new(),
            otp_verification_repo: OtpVerificationEventRepository::new(),
            language_setting_repo: LanguageSettingEventRepository::new(),
            user_profile_repo: UserProfileEventRepository::new(),
            user_register_repo: UserRegisterRepository::new(),
        }
    }
    
    // Get next user number
    async fn get_next_user_number(&self) -> u64 {
        let mut counter = self.user_counter.lock().await;
        *counter += 1;
        *counter
    }
    
    // Store connect event
    pub async fn store_connect_event(&self, socket_id: &str, token: i32, message: &str, status: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<ConnectEvent> = self.db.collection("connect_events");
        let event = ConnectEvent::new(socket_id.to_string(), token, message.to_string(), status.to_string());
        collection.insert_one(event, None).await?;
        info!("📝 Stored connect event for socket: {}", socket_id);
        Ok(())
    }
    
    // Store device info event
    pub async fn store_device_info_event(&self, socket_id: &str, device_info: &serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<DeviceInfoEvent> = self.db.collection("device_info_events");
        let event = DeviceInfoEvent::new(socket_id.to_string(), device_info.clone());
        collection.insert_one(event, None).await?;
        info!("📝 Stored device info event for socket: {}", socket_id);
        Ok(())
    }
    
    // Store login event
    pub async fn store_login_event(&self, socket_id: &str, mobile_no: &str, device_id: &str, fcm_token: &str, email: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<LoginEvent> = self.db.collection("login_events");
        let event = LoginEvent {
            id: None,
            socket_id: socket_id.to_string(),
            mobile_no: mobile_no.to_string(),
            device_id: device_id.to_string(),
            fcm_token: fcm_token.to_string(),
            email: email.map(|e| e.to_string()),
            timestamp: bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        };
        match collection.insert_one(event, None).await {
            Ok(_) => {
                info!("📝 Stored login event for mobile: {}", mobile_no);
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to store login event for mobile {}: {}", mobile_no, e);
                Err(Box::new(e))
            }
        }
    }
    
    // Store login success event
    pub async fn store_login_success_event(&self, socket_id: &str, mobile_no: &str, device_id: &str, session_token: &str, otp: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<LoginSuccessEvent> = self.db.collection("login_success_events");
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::minutes(30); // OTP expires in 30 minutes
        
        let event = LoginSuccessEvent {
            id: None,
            socket_id: socket_id.to_string(),
            mobile_no: mobile_no.to_string(),
            device_id: device_id.to_string(),
            session_token: session_token.to_string(),
            otp,
            timestamp: bson::DateTime::from_millis(now.timestamp_millis()),
            expires_at: bson::DateTime::from_millis(expires_at.timestamp_millis()),
        };
        match collection.insert_one(event, None).await {
            Ok(_) => {
                info!("📝 Stored login success event for mobile: {} (OTP expires at: {})", mobile_no, expires_at);
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to store login success event for mobile {}: {}", mobile_no, e);
                Err(Box::new(e))
            }
        }
    }
    
    // Store OTP verification event
    pub async fn store_otp_verification_event(
        &self,
        socket_id: &str,
        mobile_no: &str,
        session_token: &str,
        otp: &str,
        is_success: bool,
        user_id: Option<&str>,
        user_number: Option<u64>,
        jwt_token: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<OtpVerificationEvent> = self.db.collection("otp_verification_events");
        let event = OtpVerificationEvent {
            id: None,
            socket_id: socket_id.to_string(),
            mobile_no: mobile_no.to_string(),
            session_token: session_token.to_string(),
            otp: otp.to_string(),
            is_success,
            user_id: user_id.map(|id| id.to_string()),
            user_number,
            jwt_token: jwt_token.map(|token| token.to_string()),
            timestamp: bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        };
        collection.insert_one(event, None).await?;
        info!("📝 Stored OTP verification event for mobile: {} (success: {})", mobile_no, is_success);
        Ok(())
    }
    
    // Store user registration event
    pub async fn store_user_registration_event(
        &self,
        socket_id: &str,
        user_id: &str,
        user_number: u64,
        mobile_no: &str,
        device_id: &str,
        fcm_token: &str,
        email: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<UserRegistrationEvent> = self.db.collection("user_registration_events");
        let event = UserRegistrationEvent {
            id: None,
            socket_id: socket_id.to_string(),
            user_id: user_id.to_string(),
            user_number,
            mobile_no: mobile_no.to_string(),
            device_id: device_id.to_string(),
            fcm_token: fcm_token.to_string(),
            email: email.map(|e| e.to_string()),
            timestamp: bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        };
        collection.insert_one(event, None).await?;
        info!("📝 Stored user registration event for user: {} (number: {})", user_id, user_number);
        Ok(())
    }
    
    // Store user profile event
    pub async fn store_user_profile_event(
        &self,
        socket_id: &str,
        user_id: &str,
        user_number: u64,
        mobile_no: &str,
        full_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<UserProfileEvent> = self.db.collection("user_profile_events");
        let event = UserProfileEvent {
            id: None,
            socket_id: socket_id.to_string(),
            user_id: user_id.to_string(),
            user_number,
            mobile_no: mobile_no.to_string(),
            full_name: full_name.to_string(),
            timestamp: bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        };
        collection.insert_one(event, None).await?;
        info!("📝 Stored user profile event for user: {} (number: {})", user_id, user_number);
        Ok(())
    }
    
    // Store language setting event
    pub async fn store_language_setting_event(
        &self,
        socket_id: &str,
        user_id: &str,
        user_number: u64,
        mobile_no: &str,
        language_code: &str,
        language_name: &str,
        region_code: Option<&str>,
        timezone: Option<&str>,
        user_preferences: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<LanguageSettingEvent> = self.db.collection("language_setting_events");
        let event = LanguageSettingEvent {
            id: None,
            socket_id: socket_id.to_string(),
            user_id: user_id.to_string(),
            user_number,
            mobile_no: mobile_no.to_string(),
            language_code: language_code.to_string(),
            language_name: language_name.to_string(),
            region_code: region_code.map(|r| r.to_string()),
            timezone: timezone.map(|t| t.to_string()),
            user_preferences: user_preferences.clone(),
            timestamp: bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        };
        collection.insert_one(event, None).await?;
        info!("📝 Stored language setting event for user: {} (number: {})", user_id, user_number);
        Ok(())
    }
    
    // Store connection error event
    pub async fn store_connection_error_event(
        &self,
        socket_id: &str,
        error_code: &str,
        error_type: &str,
        field: &str,
        message: &str,
        payload: bson::Document,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<ConnectionErrorEvent> = self.db.collection("connection_error_events");
        let event = ConnectionErrorEvent::new(
            socket_id.to_string(),
            error_code.to_string(),
            error_type.to_string(),
            field.to_string(),
            message.to_string(),
            payload,
        );
        match collection.insert_one(event, None).await {
            Ok(_) => {
                info!("📝 Stored connection error event for socket: {} (error: {})", socket_id, error_code);
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to store connection error event for socket {}: {}", socket_id, e);
                Err(Box::new(e))
            }
        }
    }
    
    // Check if user exists
    pub async fn user_exists(&self, mobile_no: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.user_exists(mobile_no).await
    }
    
    // Get user by mobile number
    pub async fn get_user_by_mobile(&self, mobile_no: &str) -> Result<Option<UserRegister>, Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.find_user_by_mobile(mobile_no).await
    }
    
    // Register new user with UUID v7 and sequential numbering
    pub async fn register_new_user(
        &self,
        mobile_no: &str,
        device_id: &str,
        fcm_token: &str,
        email: Option<&str>,
    ) -> Result<(String, u64), Box<dyn std::error::Error + Send + Sync>> {
        // Get next user number
        let user_number = self.get_next_user_number().await;
        
        // Create new user with UUID v7
        let user = UserRegister::new(
            mobile_no.to_string(),
            device_id.to_string(),
            fcm_token.to_string(),
            email.map(|e| e.to_string()),
            user_number,
        );
        
        let user_id = user.user_id.clone();
        
        // Insert user using the repository
        self.user_register_repo.create_user_register(&user).await?;
        
        info!("🆕 Registered new user: {} (number: {})", user_id, user_number);
        Ok((user_id, user_number))
    }
    
    // Update user login info
    pub async fn update_user_login_info(&self, mobile_no: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.update_user_login_info(mobile_no).await
    }
    
    // Update user FCM token
    pub async fn update_user_fcm_token(&self, mobile_no: &str, fcm_token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<UserRegister> = self.db.collection("userregister");
        let filter = doc! { "mobile_no": mobile_no };
        let update = doc! {
            "$set": {
                "fcm_token": fcm_token,
                "updated_at": bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis())
            }
        };
        collection.update_one(filter, update, None).await?;
        info!("🔄 Updated FCM token for mobile: {}", mobile_no);
        Ok(())
    }
    
    // Update user profile
    pub async fn update_user_profile(&self, mobile_no: &str, full_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.update_user_profile(
            mobile_no, 
            Some(full_name.to_string()), 
            None, 
            None, 
            None, 
            None
        ).await
    }
    
    // Update user language settings
    pub async fn update_user_language_in_register(
        &self,
        mobile_no: &str,
        language_code: Option<String>,
        language_name: Option<String>,
        region_code: Option<String>,
        timezone: Option<String>,
        user_preferences: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.update_user_language_settings(
            mobile_no,
            language_code,
            language_name,
            region_code,
            timezone,
            Some(user_preferences)
        ).await
    }
    
    // Verify OTP and return user info
    pub async fn verify_otp(&self, _socket_id: &str, mobile_no: &str, session_token: &str, otp: &str) -> Result<OtpVerificationResult, Box<dyn std::error::Error + Send + Sync>> {
        // Find the login success event for this mobile number and session token
        let login_success_event = self.login_success_repo.find_login_success_by_mobile_and_session(mobile_no, session_token).await?;
        
        match login_success_event {
            Some(event) => {
                // Check if the OTP session has expired
                let now = chrono::Utc::now();
                let expires_at = chrono::DateTime::from_timestamp_millis(event.expires_at.timestamp_millis())
                    .unwrap_or(chrono::Utc::now());
                
                if now > expires_at {
                    info!("⏰ OTP session expired for mobile: {} (expired at: {}, current time: {})", 
                          mobile_no, expires_at, now);
                    return Ok(OtpVerificationResult::Expired);
                }
                
                // Compare the provided OTP with the stored OTP
                let stored_otp = event.otp.to_string();
                let provided_otp = otp.to_string();
                
                let is_valid = provided_otp == stored_otp;
                
                info!("🔢 OTP verification for mobile: {} (provided: {}, stored: {}, valid: {}, expires: {})", 
                      mobile_no, provided_otp, stored_otp, is_valid, expires_at);
                
                if is_valid {
                    Ok(OtpVerificationResult::Success)
                } else {
                    Ok(OtpVerificationResult::Invalid)
                }
            }
            None => {
                // No login success event found for this mobile number and session token
                info!("❌ No login success event found for mobile: {} with session token: {}", mobile_no, session_token);
                Ok(OtpVerificationResult::NotFound)
            }
        }
    }
    
    // Get user by session token (for session verification)
    pub async fn get_user_by_session_token(&self, session_token: &str) -> Result<Option<UserRegister>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, you would store and verify session tokens
        // For demo purposes, we'll extract mobile number from session token
        let mobile_no = session_token.chars().take(10).collect::<String>();
        self.get_user_by_mobile(&mobile_no).await
    }

    // Verify session and mobile number
    pub async fn verify_session_and_mobile(&self, mobile_no: &str, session_token: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let login_success = self.login_success_repo.find_login_success_by_mobile_and_session(mobile_no, session_token).await?;
        Ok(login_success.is_some())
    }

    // Check if referral code exists
    pub async fn check_referral_code_exists(&self, referral_code: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.check_referral_code_exists(referral_code).await
    }

    // Generate unique referral code
    pub async fn generate_unique_referral_code(&self, _mobile_no: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 10;
        
        while attempts < MAX_ATTEMPTS {
            // Generate a 6-character alphanumeric code using a thread-safe approach
            let code: String = (0..6)
                .map(|_| {
                    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                    let idx = rand::random::<usize>() % chars.len();
                    chars.chars().nth(idx).unwrap()
                })
                .collect();
            
            // Check if code already exists
            let exists = self.check_referral_code_exists(&code).await?;
            if !exists {
                return Ok(code);
            }
            
            attempts += 1;
        }
        
        Err("Failed to generate unique referral code after maximum attempts".into())
    }

    // Update user profile in register
    pub async fn update_user_profile_in_register(
        &self,
        mobile_no: &str,
        full_name: Option<String>,
        state: Option<String>,
        referral_code: Option<String>,
        referred_by: Option<String>,
        profile_data: Option<serde_json::Value>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_register_repo.update_user_profile(mobile_no, full_name, state, referral_code, referred_by, profile_data).await
    }

    // Check OTP verification attempts and implement rate limiting
    pub async fn check_otp_attempts(&self, mobile_no: &str, session_token: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Get the count of verification attempts for this mobile number and session token
        let attempts_count = self.otp_verification_repo.get_verification_attempts_count(mobile_no, session_token).await?;
        
        // Allow maximum 5 attempts per session
        const MAX_ATTEMPTS: i32 = 5;
        let is_allowed = attempts_count < MAX_ATTEMPTS;
        
        if !is_allowed {
            info!("🚫 OTP verification attempts exceeded for mobile: {} (attempts: {}, max: {})", 
                  mobile_no, attempts_count, MAX_ATTEMPTS);
        } else {
            info!("✅ OTP verification attempt allowed for mobile: {} (attempts: {}/{})", 
                  mobile_no, attempts_count + 1, MAX_ATTEMPTS);
        }
        
        Ok(is_allowed)
    }

    // Clean up expired OTP sessions
    pub async fn cleanup_expired_otp_sessions(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let collection: Collection<LoginSuccessEvent> = self.db.collection("login_success_events");
        let now = chrono::Utc::now();
        let filter = doc! {
            "expires_at": {
                "$lt": bson::DateTime::from_millis(now.timestamp_millis())
            }
        };
        
        let result = collection.delete_many(filter, None).await?;
        let deleted_count = result.deleted_count;
        
        if deleted_count > 0 {
            info!("🧹 Cleaned up {} expired OTP sessions", deleted_count);
        }
        
        Ok(deleted_count)
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