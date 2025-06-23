use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // User ID (UUID v7)
    pub user_number: u64,      // Sequential user number
    pub mobile_no: String,     // Mobile number
    pub device_id: String,     // Device ID
    pub fcm_token: String,     // FCM token
    pub iat: i64,             // Issued at
    pub exp: i64,             // Expiration time
    pub jti: String,          // JWT ID (unique token identifier)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub user_id: String,
    pub user_number: u64,
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub struct JwtService {
    secret_key: String,
    token_expiry_hours: i64,
}

impl JwtService {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key,
            token_expiry_hours: 24 * 7, // 7 days default
        }
    }

    pub fn new_with_expiry(secret_key: String, expiry_hours: i64) -> Self {
        Self {
            secret_key,
            token_expiry_hours: expiry_hours,
        }
    }

    pub fn generate_token(
        &self,
        user_id: &str,
        user_number: u64,
        mobile_no: &str,
        device_id: &str,
        fcm_token: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.token_expiry_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            user_number,
            mobile_no: mobile_no.to_string(),
            device_id: device_id.to_string(),
            fcm_token: fcm_token.to_string(),
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
            jti: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_ref()),
        )?;

        info!("🔐 Generated JWT token for user: {} (number: {})", user_id, user_number);
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_ref()),
            &Validation::default(),
        )?;

        info!("✅ JWT token verified for user: {} (number: {})", token_data.claims.sub, token_data.claims.user_number);
        Ok(token_data.claims)
    }

    pub fn verify_token_with_device_check(
        &self,
        token: &str,
        expected_device_id: &str,
        expected_mobile_no: &str,
    ) -> Result<Claims, Box<dyn std::error::Error>> {
        let claims = self.verify_token(token)?;
        
        // Verify device ID and mobile number match
        if claims.device_id != expected_device_id {
            return Err("Device ID mismatch".into());
        }
        
        if claims.mobile_no != expected_mobile_no {
            return Err("Mobile number mismatch".into());
        }

        info!("✅ JWT token verified with device check for user: {} (device: {})", claims.sub, claims.device_id);
        Ok(claims)
    }

    pub fn refresh_token(&self, old_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        let claims = self.verify_token(old_token)?;
        
        // Generate new token with same claims but new expiry
        self.generate_token(
            &claims.sub,
            claims.user_number,
            &claims.mobile_no,
            &claims.device_id,
            &claims.fcm_token,
        )
    }

    pub fn get_token_payload(&self, token: &str) -> Result<TokenPayload, Box<dyn std::error::Error>> {
        let claims = self.verify_token(token)?;
        
        Ok(TokenPayload {
            user_id: claims.sub,
            user_number: claims.user_number,
            mobile_no: claims.mobile_no,
            device_id: claims.device_id,
            fcm_token: claims.fcm_token,
            token_type: "Bearer".to_string(),
            expires_in: claims.exp - Utc::now().timestamp(),
        })
    }

    pub fn is_token_expired(&self, token: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let claims = self.verify_token(token)?;
        let now = Utc::now().timestamp();
        Ok(claims.exp < now)
    }
}

// Helper function to create JWT service with default secret
pub fn create_jwt_service() -> JwtService {
    let secret_key = std::env::var("JWT_SECRET_KEY")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
    
    JwtService::new(secret_key)
} 