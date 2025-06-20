use mongodb::{Collection, bson::{doc, oid::ObjectId, DateTime, to_bson}};
use tracing::info;
use futures_util::TryStreamExt;
use crate::database::{DatabaseManager, models::*};

// Helper function to safely convert inserted_id to ObjectId
fn safe_object_id_conversion(inserted_id: mongodb::bson::Bson) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
    inserted_id.as_object_id()
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to get ObjectId from inserted document")) as Box<dyn std::error::Error + Send + Sync>)
}

// Separate repositories for each event type
pub struct ConnectEventRepository {
    collection: Collection<ConnectEvent>,
}

pub struct DeviceInfoEventRepository {
    collection: Collection<DeviceInfoEvent>,
}

pub struct ConnectionErrorEventRepository {
    collection: Collection<ConnectionErrorEvent>,
}

pub struct LoginEventRepository {
    collection: Collection<LoginEvent>,
}

pub struct LoginSuccessEventRepository {
    collection: Collection<LoginSuccessEvent>,
}

pub struct OtpVerificationEventRepository {
    collection: Collection<OtpVerificationEvent>,
}

pub struct LanguageSettingEventRepository {
    collection: Collection<LanguageSettingEvent>,
}

pub struct UserProfileEventRepository {
    collection: Collection<UserProfileEvent>,
}

pub struct UserRegisterRepository {
    collection: Collection<UserRegister>,
}

impl ConnectEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<ConnectEvent>("connect_events");
        Self { collection }
    }
    
    pub async fn store_connect_event(&self, event: ConnectEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("🔌 Connect event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
}

impl DeviceInfoEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<DeviceInfoEvent>("device_info_events");
        Self { collection }
    }
    
    pub async fn store_device_info_event(&self, event: DeviceInfoEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("📱 Device info event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
}

impl ConnectionErrorEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<ConnectionErrorEvent>("connection_error_events");
        Self { collection }
    }
    
    pub async fn store_connection_error_event(&self, event: ConnectionErrorEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("❌ Connection error event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
}

impl LoginEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<LoginEvent>("login_events");
        Self { collection }
    }
    
    pub async fn store_login_event(&self, event: LoginEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("🔐 Login event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
}

impl LoginSuccessEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<LoginSuccessEvent>("login_success_events");
        Self { collection }
    }
    
    pub async fn store_login_success_event(&self, event: LoginSuccessEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("✅ Login success event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
    
    // Find login success event by mobile number and session token
    pub async fn find_login_success_by_mobile_and_session(&self, mobile_no: &str, session_token: &str) -> Result<Option<LoginSuccessEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no,
            "session_token": session_token
        };
        let event = self.collection.find_one(filter, None).await?;
        Ok(event)
    }
}

impl OtpVerificationEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<OtpVerificationEvent>("otp_verification_events");
        Self { collection }
    }
    
    pub async fn store_otp_verification_event(&self, event: OtpVerificationEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("🔢 OTP verification event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
    
    // Get OTP verification attempts count for a mobile number and session token
    pub async fn get_verification_attempts_count(&self, mobile_no: &str, session_token: &str) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no,
            "session_token": session_token
        };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count as i32)
    }
}

impl LanguageSettingEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<LanguageSettingEvent>("language_setting_events");
        Self { collection }
    }
    
    pub async fn store_language_setting_event(&self, event: LanguageSettingEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("🌐 Language setting event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
    
    // Find language setting by mobile number and session token
    pub async fn find_language_setting_by_mobile_and_session(&self, mobile_no: &str, session_token: &str) -> Result<Option<LanguageSettingEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no,
            "session_token": session_token
        };
        let event = self.collection.find_one(filter, None).await?;
        Ok(event)
    }
}

impl UserProfileEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<UserProfileEvent>("user_profile_events");
        Self { collection }
    }
    
    pub async fn store_user_profile_event(&self, event: UserProfileEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("👤 User profile event stored with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
    
    // Find user profile by mobile number and session token
    pub async fn find_user_profile_by_mobile_and_session(&self, mobile_no: &str, session_token: &str) -> Result<Option<UserProfileEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no,
            "session_token": session_token
        };
        let event = self.collection.find_one(filter, None).await?;
        Ok(event)
    }
    
    // Check if referral code already exists
    pub async fn check_referral_code_exists(&self, referral_code: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "referral_code": referral_code
        };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count > 0)
    }
}

impl UserRegisterRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<UserRegister>("userregister");
        Self { collection }
    }
    
    pub async fn store_user_register_event(&self, event: UserRegister) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("👤 User registered with ID: {}", result.inserted_id);
        safe_object_id_conversion(result.inserted_id)
    }
    
    // Create a new user in the userregister collection
    pub async fn create_user_register(&self, user: &UserRegister) -> Result<ObjectId, mongodb::error::Error> {
        let result = self.collection.insert_one(user, None).await?;
        result.inserted_id.as_object_id()
            .ok_or_else(|| mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to get ObjectId from inserted document")))
    }
    
    // Find user by mobile number
    pub async fn find_user_by_mobile(&self, mobile_no: &str) -> Result<Option<UserRegister>, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "mobile_no": mobile_no };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }
    
    // Update user login information
    pub async fn update_user_login_info(&self, mobile_no: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no
        };
        let update = doc! {
            "$set": {
                "last_login_date": DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
                "is_active": true
            },
            "$inc": {
                "total_logins": 1
            }
        };
        let result = self.collection.update_one(filter, update, None).await?;
        if result.modified_count > 0 {
            info!("Updated login info for mobile: {}", mobile_no);
        }
        Ok(())
    }
    
    // Update user profile information
    pub async fn update_user_profile(&self, mobile_no: &str, full_name: Option<String>, state: Option<String>, referral_code: Option<String>, referred_by: Option<String>, profile_data: Option<serde_json::Value>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no
        };
        
        let mut set_doc = doc! {
            "updated_at": DateTime::from_millis(chrono::Utc::now().timestamp_millis())
        };
        
        if let Some(name) = full_name {
            set_doc.insert("full_name", name);
        }
        if let Some(state_val) = state {
            set_doc.insert("state", state_val);
        }
        if let Some(ref_code) = referral_code {
            set_doc.insert("referral_code", ref_code);
        }
        if let Some(ref_by) = referred_by {
            set_doc.insert("referred_by", ref_by);
        }
        if let Some(profile) = profile_data {
            set_doc.insert("profile_data", to_bson(&profile)?);
        }
        
        let update_doc = doc! { "$set": set_doc };
        let result = self.collection.update_one(filter, update_doc, None).await?;
        
        if result.modified_count > 0 {
            info!("✅ Updated profile for mobile: {} (modified: {})", mobile_no, result.modified_count);
        } else {
            info!("⚠️ No changes made to profile for mobile: {} (matched: {})", mobile_no, result.matched_count);
        }
        
        Ok(())
    }
    
    // Update user language settings
    pub async fn update_user_language_settings(&self, mobile_no: &str, language_code: Option<String>, language_name: Option<String>, region_code: Option<String>, timezone: Option<String>, user_preferences: Option<serde_json::Value>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "mobile_no": mobile_no
        };
        
        let mut set_doc = doc! {
            "updated_at": DateTime::from_millis(chrono::Utc::now().timestamp_millis())
        };
        
        if let Some(lang_code) = language_code {
            set_doc.insert("language_code", lang_code);
        }
        if let Some(lang_name) = language_name {
            set_doc.insert("language_name", lang_name);
        }
        if let Some(region) = region_code {
            set_doc.insert("region_code", region);
        }
        if let Some(tz) = timezone {
            set_doc.insert("timezone", tz);
        }
        if let Some(prefs) = user_preferences {
            set_doc.insert("user_preferences", to_bson(&prefs)?);
        }
        
        let update_doc = doc! { "$set": set_doc };
        let result = self.collection.update_one(filter, update_doc, None).await?;
        
        if result.modified_count > 0 {
            info!("✅ Updated language settings for mobile: {} (modified: {})", mobile_no, result.modified_count);
        } else {
            info!("⚠️ No changes made to language settings for mobile: {} (matched: {})", mobile_no, result.matched_count);
        }
        
        Ok(())
    }
    
    // Check if user exists
    pub async fn user_exists(&self, mobile_no: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "mobile_no": mobile_no };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count > 0)
    }
    
    // Check if referral code already exists
    pub async fn check_referral_code_exists(&self, referral_code: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { 
            "referral_code": referral_code
        };
        let count = self.collection.count_documents(filter, None).await?;
        Ok(count > 0)
    }
    
    // Get user by mobile number (returns mongodb::error::Error for compatibility)
    pub async fn get_user_by_mobile(&self, mobile_no: &str) -> Result<Option<UserRegister>, mongodb::error::Error> {
        let filter = doc! { "mobile_no": mobile_no };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }
    
    // Get all users
    pub async fn get_all_users(&self) -> Result<Vec<UserRegister>, Box<dyn std::error::Error + Send + Sync>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users = Vec::new();
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }
    
    // Get user statistics
    pub async fn get_user_statistics(&self) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let total_users = self.collection.count_documents(None, None).await?;
        let today = chrono::Utc::now().date_naive();
        let today_start = DateTime::from_millis(today.and_hms_opt(0, 0, 0)
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid time")) as Box<dyn std::error::Error + Send + Sync>)?
            .and_utc().timestamp_millis());
        let today_filter = doc! { "created_at": { "$gte": today_start } };
        let new_users_today = self.collection.count_documents(today_filter, None).await?;
        
        let active_filter = doc! { "is_active": true };
        let active_users = self.collection.count_documents(active_filter, None).await?;
        
        Ok(serde_json::json!({
            "total_users": total_users,
            "new_users_today": new_users_today,
            "active_users": active_users,
            "last_updated": chrono::Utc::now().to_rfc3339()
        }))
    }
} 