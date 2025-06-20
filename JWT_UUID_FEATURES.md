# JWT Token and UUID v7 Features Documentation

## Overview

This document describes the implementation of two major features added to the Game Admin Backend:

1. **UUID v7 User IDs with Sequential Numbering**
2. **JWT Token Authentication System**

## 1. UUID v7 User IDs with Sequential Numbering

### Features
- **UUID v7 Generation**: Each user gets a unique UUID v7 identifier
- **Sequential Numbering**: Users also get a sequential number (1, 2, 3, etc.)
- **Database Integration**: Both UUID and number are stored in MongoDB
- **Event Tracking**: All user events include both identifiers

### Implementation Details

#### User Model Structure
```rust
pub struct User {
    pub id: Option<ObjectId>,           // MongoDB ObjectId
    pub user_id: String,                // UUID v7
    pub user_number: u64,               // Sequential number
    pub mobile_no: String,
    pub device_id: String,
    pub fcm_token: String,
    // ... other fields
}
```

#### UUID v7 Generation
```rust
use uuid::Uuid;

// Generate UUID v7
let user_id = Uuid::new_v7().to_string();
```

#### Sequential Numbering
```rust
// Get next user number
async fn get_next_user_number(&self) -> u64 {
    let mut counter = self.user_counter.lock().await;
    *counter += 1;
    *counter
}
```

### Usage Example
```rust
// Register new user
let (user_id, user_number) = data_service.register_new_user(
    mobile_no,
    device_id,
    fcm_token,
    email
).await?;

// user_id = "018f1234-5678-9abc-def0-123456789abc" (UUID v7)
// user_number = 1 (sequential)
```

## 2. JWT Token Authentication System

### Features
- **JWT Token Generation**: After successful OTP verification
- **Device-Specific Tokens**: Tokens include FCM + device_id + mobile_number
- **Secret Key Configuration**: Configurable secret key for token signing
- **Token Validation**: Comprehensive token verification
- **Token Refresh**: Support for token refresh functionality
- **Expiry Management**: 7-day default expiry with configurable duration

### Implementation Details

#### JWT Claims Structure
```rust
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
```

#### JWT Service Configuration
```rust
pub struct JwtService {
    secret_key: String,
    token_expiry_hours: i64,
}

// Create JWT service with environment variable
pub fn create_jwt_service() -> JwtService {
    let secret_key = std::env::var("JWT_SECRET_KEY")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
    
    JwtService::new(secret_key)
}
```

#### Token Generation
```rust
// Generate JWT token after OTP verification
let jwt_token = jwt_service.generate_token(
    &user_id,           // UUID v7
    user_number,        // Sequential number
    mobile_no,          // Mobile number
    device_id,          // Device ID
    fcm_token,          // FCM token
)?;
```

#### Token Validation
```rust
// Verify token with device check
let claims = jwt_service.verify_token_with_device_check(
    token,
    expected_device_id,
    expected_mobile_no,
)?;
```

### JWT Token Flow

1. **User Login**: User provides mobile, device_id, fcm_token
2. **OTP Generation**: Server generates OTP and session token
3. **OTP Verification**: User verifies OTP
4. **JWT Generation**: Server generates JWT token with:
   - User ID (UUID v7)
   - User number (sequential)
   - Mobile number
   - Device ID
   - FCM token
   - Expiry (7 days)
5. **Token Response**: Client receives JWT token for future requests

### Environment Configuration

Set the JWT secret key in your environment:
```bash
export JWT_SECRET_KEY="your-super-secret-jwt-key-change-in-production"
```

Or use the default key (not recommended for production).

## 3. Database Schema Updates

### New Collections
- `users`: Main user collection with UUID v7 and sequential numbering
- `login_sessions`: Session tracking with JWT tokens
- `otp_verification_events`: OTP events with JWT token storage

### Updated Collections
- All event collections now include `user_id` and `user_number` fields

## 4. API Response Examples

### Successful OTP Verification Response
```json
{
  "status": "success",
  "message": "OTP verification successful. Authentication completed.",
  "mobile_no": "8888855555",
  "session_token": "123456789",
  "user_id": "018f1234-5678-9abc-def0-123456789abc",
  "user_number": 1,
  "user_status": "new_user",
  "jwt_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 604800,
  "timestamp": "2024-01-01T12:00:00Z",
  "socket_id": "socket123",
  "event": "otp:verified"
}
```

### JWT Token Payload (Decoded)
```json
{
  "sub": "018f1234-5678-9abc-def0-123456789abc",
  "user_number": 1,
  "mobile_no": "8888855555",
  "device_id": "test-device-001",
  "fcm_token": "test-fcm-token-123",
  "iat": 1704110400,
  "exp": 1704715200,
  "jti": "018f1234-5678-9abc-def0-123456789def"
}
```

## 5. Testing

### Run JWT Tests
```bash
cd test-client
node run-jwt-test.js
```

### Test Features
- ✅ UUID v7 User ID generation
- ✅ Sequential user numbering
- ✅ JWT token generation after OTP verification
- ✅ JWT token validation with device check
- ✅ JWT token payload verification
- ✅ JWT token expiry handling
- ✅ FCM + Device ID + Mobile Number integration
- ✅ Secret key configuration

## 6. Security Considerations

### JWT Security
- **Secret Key**: Use strong, unique secret keys in production
- **Token Expiry**: 7-day default expiry (configurable)
- **Device Binding**: Tokens are bound to specific device_id and mobile_number
- **Token Rotation**: Support for token refresh functionality

### UUID v7 Benefits
- **Time-ordered**: UUID v7 includes timestamp for chronological ordering
- **Unique**: Extremely low collision probability
- **Secure**: Random component prevents enumeration attacks
- **Efficient**: Optimized for database indexing

## 7. Migration Notes

### For Existing Users
- New users will get UUID v7 and sequential numbers
- Existing users will be assigned UUID v7 and numbers on next login
- All new events will include both identifiers

### Database Migration
- No automatic migration required
- Users get new identifiers on next interaction
- Backward compatibility maintained

## 8. Future Enhancements

### Planned Features
- **Token Blacklisting**: Support for token revocation
- **Multi-device Support**: Multiple devices per user
- **Token Analytics**: Token usage tracking
- **Rate Limiting**: JWT-based rate limiting
- **Audit Logging**: Comprehensive token audit trail

### Performance Optimizations
- **Token Caching**: Redis-based token caching
- **Batch Operations**: Bulk token operations
- **Indexing**: Optimized database indexes for UUID v7

## 9. Troubleshooting

### Common Issues

#### JWT Token Generation Fails
- Check JWT_SECRET_KEY environment variable
- Verify all required fields are present
- Check database connectivity

#### UUID v7 Generation Issues
- Ensure uuid crate is properly configured
- Check for system clock issues
- Verify MongoDB connection

#### Sequential Numbering Problems
- Check user counter initialization
- Verify database transaction handling
- Monitor for concurrent access issues

### Debug Commands
```bash
# Check JWT secret key
echo $JWT_SECRET_KEY

# Test UUID generation
cargo test uuid_generation

# Verify database connectivity
cargo test database_connection
```

## 10. Support

For issues or questions regarding these features:
1. Check the logs for detailed error messages
2. Run the test suite to verify functionality
3. Review the database schema and data integrity
4. Contact the development team for assistance 