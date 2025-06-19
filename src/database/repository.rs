use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use tracing::info;
use crate::database::{DatabaseManager, models::*};

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

impl ConnectEventRepository {
    pub fn new() -> Self {
        let database = DatabaseManager::get_database();
        let collection = database.collection::<ConnectEvent>("connect_events");
        Self { collection }
    }
    
    pub async fn store_connect_event(&self, event: ConnectEvent) -> Result<ObjectId, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.collection.insert_one(event, None).await?;
        info!("🔌 Connect event stored with ID: {}", result.inserted_id);
        Ok(result.inserted_id.as_object_id().unwrap())
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
        Ok(result.inserted_id.as_object_id().unwrap())
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
        Ok(result.inserted_id.as_object_id().unwrap())
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
        Ok(result.inserted_id.as_object_id().unwrap())
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
        Ok(result.inserted_id.as_object_id().unwrap())
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
        Ok(result.inserted_id.as_object_id().unwrap())
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