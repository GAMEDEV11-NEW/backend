# Game Admin Backend (Rust)

A robust Socket.IO server built with Rust for managing game administration, device connections, user authentication, and real-time game actions with MongoDB data persistence.

## Project Structure

```
src/
├── api/                    # API related code
│   ├── middleware/        # Custom middleware components
│   │   └── socket_io_validation.rs   # Socket.IO validation
│   └── mod.rs
├── managers/             # Business logic managers
│   ├── connection.rs     # Connection management
│   ├── events.rs         # Event handlers
│   ├── validation.rs     # Data validation
│   └── mod.rs
├── database/             # Database layer
│   ├── connection.rs     # MongoDB connection management
│   ├── models.rs         # Data models and schemas
│   ├── repository.rs     # Data access layer
│   ├── service.rs        # Business logic services
│   └── mod.rs           # Database module exports
└── main.rs              # Application entry point

test-client/            # Test client implementations
├── test-all.js         # Complete test suite
├── test-login.js       # Login functionality tests
├── test-login-flow.js  # Enhanced login flow tests
├── run-login-tests.js  # Login test runner
├── test-device.js      # Device-specific tests
└── test_connections.py # Connection testing
```

## Features

- 🔐 Secure WebSocket/Socket.IO connections
- 👤 User authentication with login system
- 📧 Email verification and notifications
- 📱 Device information management
- 🚦 Comprehensive event handling
- 📝 Detailed logging and error tracking
- ⚡ Asynchronous operation
- 🛡️ CORS support
- 🔌 Connection validation
- 📊 Structured JSON responses
- 🗄️ MongoDB data persistence
- 📈 Real-time analytics and metrics
- 🎮 Game session tracking
- 📊 Event logging and monitoring

## 🗄️ Database Integration

### MongoDB Setup

The application now includes MongoDB integration for persistent data storage. See [MONGODB_SETUP.md](./MONGODB_SETUP.md) for detailed setup instructions.

#### Quick Setup
1. Install MongoDB Community Edition
2. Create a `.env` file with MongoDB configuration:
   ```env
   MONGODB_URI=mongodb://localhost:27017
   MONGODB_DATABASE=game_admin
   ```
3. Start the server: `cargo run`

#### Database Collections
- `users` - User information and status tracking
- `game_sessions` - Game session management
- `game_events` - Event logging and analytics
- `system_metrics` - System performance monitoring

### Data Models

#### User Model
```rust
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
    pub device_info: Option<DeviceInfo>,
    pub status: UserStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_login: Option<DateTime>,
    pub login_count: i32,
    pub is_active: bool,
}
```

#### Game Session Model
```rust
pub struct GameSession {
    pub session_id: String,
    pub user_id: String,
    pub game_id: Option<String>,
    pub status: SessionStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub ended_at: Option<DateTime>,
    pub duration_seconds: Option<i64>,
    pub metadata: Option<bson::Document>,
}
```

#### Event Model
```rust
pub struct GameEvent {
    pub event_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub event_type: String,
    pub event_data: bson::Document,
    pub timestamp: DateTime,
    pub severity: EventSeverity,
}
```

### Data Service Usage

The application provides a high-level `DataService` for database operations:

```rust
use crate::database::service::DataService;

let data_service = DataService::new();

// Create a user
let user_id = data_service.create_user(
    "user123".to_string(),
    "JohnDoe".to_string(),
    None
).await?;

// Start a game session
let session_id = data_service.start_game_session(
    "user123",
    Some("game456".to_string())
).await?;

// Log events
let event_data = bson::doc! {
    "score": 100,
    "level": 5
};

data_service.log_game_event(
    "user123",
    "game_achievement",
    event_data,
    EventSeverity::Low
).await?;
```

## 🔐 Login System Documentation

### Overview
The login system provides secure authentication using mobile number, device ID, and FCM token with optional email verification. This system is designed for mobile game applications with real-time communication capabilities and persistent data storage.

### Login Flow

#### 1. Initial Connection
```
Client connects to Socket.IO server
↓
Server sends connect_response with token
↓
Client receives connection acknowledgment
```

#### 2. Login Request
```
Client sends login event with credentials
↓
Server validates login data
↓
Server processes authentication
↓
Server stores user data in MongoDB
↓
Server sends email verification (if enabled)
↓
Server responds with success/error
```

#### 3. Email Verification (Optional)
```
Server generates verification code
↓
Server sends email to user
↓
User enters verification code
↓
Server validates code
↓
Server completes authentication
```

#### 4. Authentication Response
```
Success: login:success event with session token
Error: login:error event with detailed error info
```

## 📋 Required Fields

### Mandatory Fields (Compulsory)
| Field | Type | Description | Validation Rules |
|-------|------|-------------|------------------|
| `mobile_no` | string | User's mobile number | 10-15 digits only |
| `device_id` | string | Unique device identifier | 3-50 chars, alphanumeric + _ - |
| `fcm_token` | string | Firebase Cloud Messaging token | 100-500 characters |

### Optional Fields
| Field | Type | Description | Validation Rules |
|-------|------|-------------|------------------|
| `email` | string | User's email address | Valid email format |
| `timestamp` | string | ISO 8601 timestamp | Optional, must be ISO format if provided |

### Email Verification Fields (When Email is Provided)
| Field | Type | Description | Validation Rules |
|-------|------|-------------|------------------|
| `verification_code` | string | Email verification code | 6-digit numeric code |
| `email_verified` | boolean | Email verification status | true/false |

## 📧 Email System

### Email Verification Flow
1. **Registration**: User provides email during login
2. **Code Generation**: Server generates 6-digit verification code
3. **Email Sending**: Server sends verification email
4. **Code Verification**: User enters code for verification
5. **Account Activation**: Account activated upon successful verification

### Email Templates

#### Verification Email
```
Subject: Verify Your Game Account - [Game Name]

Dear [User Name],

Welcome to [Game Name]! Please verify your email address to complete your account setup.

Your verification code is: [VERIFICATION_CODE]

This code will expire in 10 minutes.

If you didn't request this verification, please ignore this email.

Best regards,
[Game Name] Team
```

#### Welcome Email (After Verification)
```
Subject: Welcome to [Game Name] - Account Verified!

Dear [User Name],

Your email has been successfully verified! Your account is now fully activated.

You can now:
- Access all game features
- Receive important notifications
- Participate in multiplayer games
- Earn rewards and achievements

Thank you for joining [Game Name]!

Best regards,
[Game Name] Team
```

#### Login Notification Email
```
Subject: New Login Detected - [Game Name]

Dear [User Name],

We detected a new login to your [Game Name] account.

Login Details:
- Device: [Device ID]
- Time: [Timestamp]
- Location: [IP Address/Location]

If this was you, no action is needed. If not, please contact support immediately.

Best regards,
[Game Name] Security Team
```

## 🔍 Validation Rules

### Mobile Number Validation
```javascript
// Must be 10-15 digits only
mobile_no: "9876543210"  // ✅ Valid
mobile_no: "12345"       // ❌ Too short
mobile_no: "98A6543210"  // ❌ Contains letters
mobile_no: "1234567890123456" // ❌ Too long
```

### Device ID Validation
```javascript
// Must be 3-50 characters, alphanumeric + underscore + hyphen
device_id: "device_001"     // ✅ Valid
device_id: "ab"             // ❌ Too short
device_id: "device@001"     // ❌ Special characters not allowed
device_id: "very_long_device_id_that_exceeds_fifty_characters_limit" // ❌ Too long
```

### FCM Token Validation
```javascript
// Must be 100-500 characters
fcm_token: "fcm_token_example_abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890" // ✅ Valid (150 chars)
fcm_token: "shorttoken"     // ❌ Too short
fcm_token: "x".repeat(600)  // ❌ Too long
```

### Email Validation (Optional)
```javascript
// Must be valid email format
email: "user@example.com"   // ✅ Valid
email: "invalid-email"      // ❌ Invalid format
email: "@example.com"       // ❌ Missing username
email: "user@"              // ❌ Missing domain
```

### Timestamp Validation (Optional)
```javascript
// Must be ISO 8601 format if provided
timestamp: "2024-01-15T10:30:00Z"  // ✅ Valid
timestamp: "2024-01-15 10:30:00"   // ❌ Wrong format
```

## 📤 Request Format

### Basic Login Request (Without Email)
```json
{
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "fcm_token": "fcm_token_example_abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

### Login Request with Email
```json
{
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "fcm_token": "fcm_token_example_abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890",
    "email": "user@example.com",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

### Email Verification Request
```json
{
    "mobile_no": "9876543210",
    "session_token": "123456789",
    "otp": "123456",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

## 📥 Response Format

### Successful Login Response (Without Email)
```json
{
    "status": "success",
    "message": "Login successful",
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "session_token": "123456789",
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "login:success"
}
```

### Successful Login Response (With Email - Pending Verification)
```json
{
    "status": "success",
    "message": "Login successful. Email verification required.",
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "email": "user@example.com",
    "email_verified": false,
    "verification_sent": true,
    "session_token": "123456789",
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "login:success"
}
```

### Successful Login Response (With Email - Verified)
```json
{
    "status": "success",
    "message": "Login successful",
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "email": "user@example.com",
    "email_verified": true,
    "session_token": "123456789",
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "login:success"
}
```

### Email Verification Success Response
```json
{
    "status": "success",
    "message": "Email verified successfully",
    "mobile_no": "9876543210",
    "email": "user@example.com",
    "email_verified": true,
    "verification_code": "123456",
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "email:verified"
}
```

### Error Response Format (Unified)
```json
{
    "status": "error",
    "error_code": "MISSING_FIELD",
    "error_type": "FIELD_ERROR",
    "field": "mobile_no",
    "message": "mobile_no is required and must be a string",
    "details": {
        "field_type": "string",
        "required": true
    },
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "connection_error"
}
```

### Email Verification Error Response
```json
{
    "status": "error",
    "error_code": "INVALID_VERIFICATION_CODE",
    "error_type": "VERIFICATION_ERROR",
    "field": "verification_code",
    "message": "Invalid or expired verification code",
    "details": {
        "attempts_remaining": 2,
        "code_expires_in": "5 minutes"
    },
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "connection_error"
}
```

## Event Types

### Authentication Events
- `login` - User authentication
- `login:success` - Successful login response
- `email:verification` - Email verification request
- `email:verified` - Email verification success
- `email:verification_error` - Email verification error
- `email:resend` - Resend verification email

### Device Events
- `device:info` - Device information submission
- `device:info:ack` - Device info acknowledgment

### Connection Events
- `connect_response` - Connection establishment response
- `connection_error` - Unified error response for all validation and authentication errors
- `disconnect` - Connection termination

## Error Codes

### Unified Error Handling
All errors (login, device info, validation, etc.) are sent via the `connection_error` event with the following structure:

### Login Errors
- `MISSING_FIELD` - Required field is missing
- `EMPTY_FIELD` - Field is empty
- `INVALID_FORMAT` - Field format is invalid
- `INVALID_LENGTH` - Field length is outside allowed range
- `INVALID_TYPE` - Field type is incorrect

### Email Verification Errors
- `INVALID_VERIFICATION_CODE` - Verification code is invalid or expired
- `VERIFICATION_CODE_EXPIRED` - Verification code has expired
- `MAX_ATTEMPTS_EXCEEDED` - Maximum verification attempts exceeded
- `EMAIL_ALREADY_VERIFIED` - Email is already verified
- `EMAIL_NOT_FOUND` - Email not found in user account

### Device Info Errors
- `MISSING_FIELD` - Required field is missing
- `EMPTY_FIELD` - Field is empty
- `INVALID_FORMAT` - Field format is invalid
- `INVALID_TYPE` - Field type is incorrect

## Getting Started

1. Install Rust and Cargo
2. Clone the repository
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Configure email settings in `.env`:
   ```env
   PORT=3002
   HOST=0.0.0.0
   SMTP_HOST=smtp.gmail.com
   SMTP_PORT=587
   SMTP_USERNAME=your-email@gmail.com
   SMTP_PASSWORD=your-app-password
   EMAIL_FROM=noreply@yourgame.com
   ```
5. Run the server:
   ```bash
   cargo run
   ```

## Development

The project uses:
- Axum for the web framework
- Socket.IO for real-time communication
- Tower for middleware
- Tokio for async runtime
- Serde for JSON serialization
- Chrono for timestamp handling
- Lettre for email functionality (to be implemented)

## Testing

Run the test suite:
```bash
# Run Rust tests
cargo test

# Run client tests
cd test-client
npm install

# Run all tests
node test-all.js

# Run login tests specifically
node test-login-flow.js

# Run interactive login tests
node test-login-flow.js --interactive

# Run session management tests
node test-login-flow.js --session
```

## Environment Variables

Create a `.env` file in the root directory:
```env
PORT=3002
HOST=0.0.0.0

# Email Configuration (Required for email features)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
EMAIL_FROM=noreply@yourgame.com

# Email Templates
GAME_NAME=Your Game Name
SUPPORT_EMAIL=support@yourgame.com
```

## Testing Examples

### Valid Login (Without Email)
```javascript
socket.emit('login', {
    mobile_no: '9876543210',
    device_id: 'device_001',
    fcm_token: 'fcm_token_example_' + 'x'.repeat(100),
    timestamp: '2024-01-15T10:30:00Z'
});
```

### Valid Login (With Email)
```javascript
socket.emit('login', {
    mobile_no: '9876543210',
    device_id: 'device_001',
    fcm_token: 'fcm_token_example_' + 'x'.repeat(100),
    email: 'user@example.com',
    timestamp: '2024-01-15T10:30:00Z'
});
```

### Email Verification
```javascript
socket.emit('email:verification', {
    mobile_no: '9876543210',
    verification_code: '123456',
    timestamp: '2024-01-15T10:30:00Z'
});
```

### Invalid Login (Missing Mobile Number)
```javascript
socket.emit('login', {
    device_id: 'device_001',
    fcm_token: 'fcm_token_example_' + 'x'.repeat(100)
});
```

### Device Info
```javascript
socket.emit('device:info', {
    device_id: 'test-device-001',
    device_type: 'game-console',
    timestamp: '2024-01-15T10:30:00Z',
    manufacturer: 'TestCorp',
    model: 'GameStation Pro',
    firmware_version: '1.2.3',
    capabilities: ['multiplayer', 'streaming', 'vr']
});
```

## Security Considerations

### Email Security
- Verification codes expire after 10 minutes
- Maximum 3 verification attempts per session
- Rate limiting on email sending (max 5 emails per hour per user)
- Secure SMTP configuration with TLS

### Session Security
- Session tokens are randomly generated
- Sessions expire after 24 hours of inactivity
- Multiple device logins are tracked and logged
- Suspicious login attempts trigger security notifications

### Data Protection
- All sensitive data is encrypted in transit
- Mobile numbers and emails are validated before processing
- FCM tokens are validated for proper format
- Timestamp validation prevents replay attacks

## Implementation Notes

### Email System Implementation Required
The email functionality needs to be implemented in the Rust backend:

1. **Add email dependencies** to `Cargo.toml`:
   ```toml
   [dependencies]
   lettre = { version = "0.10", features = ["tokio1", "tokio1-native-tls"] }
   lettre_email = "0.10"
   ```

2. **Create email manager** in `src/managers/email.rs`:
   - Email template management
   - SMTP configuration
   - Email sending functionality
   - Verification code generation and storage

3. **Update validation** in `src/managers/validation.rs`:
   - Add email format validation
   - Add verification code validation

4. **Update events** in `src/managers/events.rs`:
   - Add email verification event handlers
   - Add email sending logic
   - Add verification code management

5. **Add database storage** for:
   - User email addresses
   - Verification codes
   - Email verification status
   - Login history

### Testing Email Features
Use the enhanced test script `test-login-flow.js` to test:
- Email validation
- Verification code generation
- Email sending simulation
- Verification code validation
- Email verification flow 