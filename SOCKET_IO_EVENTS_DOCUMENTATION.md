# Socket.IO Events Documentation
## Game Admin Backend - Complete Event Reference

This document provides a comprehensive list of all Socket.IO events used in the Game Admin Backend, including both **client-to-server** (received) and **server-to-client** (emitted) events with their complete data structures.

---

## 📋 Table of Contents
1. [Connection Events](#connection-events)
2. [Device Management Events](#device-management-events)
3. [Authentication Events](#authentication-events)
4. [User Profile Events](#user-profile-events)
5. [Language Setting Events](#language-setting-events)
6. [Error Events](#error-events)
7. [Event Flow Diagrams](#event-flow-diagrams)

---

## 🔌 Connection Events

### 1. Client Connection (Automatic)
**Event**: `connect` (Socket.IO built-in)
**Direction**: Server → Client
**Trigger**: Automatic when client connects

**Response Data**:
```json
{
  "token": 123456,
  "message": "Welcome to the Game Admin Server!",
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "status": "connected",
  "event": "connect"
}
```

**Data Fields**:
- `token` (number): 6-digit random token for session identification
- `message` (string): Welcome message
- `timestamp` (string): ISO 8601 timestamp
- `socket_id` (string): Unique socket identifier
- `status` (string): Connection status ("connected")
- `event` (string): Event type ("connect")

### 2. Client Disconnection
**Event**: `disconnect` (Socket.IO built-in)
**Direction**: Client → Server
**Trigger**: When client disconnects

**Data**: No specific data structure (automatic Socket.IO event)

---

## 📱 Device Management Events

### 3. Device Information
**Event**: `device:info`
**Direction**: Client → Server
**Purpose**: Send device information for validation and tracking

**Request Data**:
```json
{
  "device_id": "device_123456789",
  "device_type": "mobile",
  "timestamp": "2024-01-15T10:30:00Z",
  "manufacturer": "Samsung",
  "model": "Galaxy S21",
  "firmware_version": "Android 12",
  "capabilities": ["camera", "gps", "bluetooth", "wifi"]
}
```

**Required Fields**:
- `device_id` (string): Unique device identifier
- `device_type` (string): Device type (mobile, tablet, desktop)
- `timestamp` (string): ISO 8601 timestamp

**Optional Fields**:
- `manufacturer` (string): Device manufacturer
- `model` (string): Device model
- `firmware_version` (string): Operating system version
- `capabilities` (array): Array of device capabilities

**Response Event**: `device:info:ack`
**Response Data**:
```json
{
  "status": "success",
  "message": "Device info received and validated",
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "device:info:ack"
}
```

---

## 🔐 Authentication Events

### 4. User Login
**Event**: `login`
**Direction**: Client → Server
**Purpose**: Authenticate user with mobile number and device

**Request Data**:
```json
{
  "mobile_no": "+1234567890",
  "device_id": "device_123456789",
  "fcm_token": "fcm_token_123456",
  "email": "user@example.com"
}
```

**Required Fields**:
- `mobile_no` (string): Mobile number with country code
- `device_id` (string): Device identifier
- `fcm_token` (string): Firebase Cloud Messaging token

**Optional Fields**:
- `email` (string): User email address

**Response Event**: `login:success`
**Response Data**:
```json
{
  "status": "success",
  "message": "Login successful",
  "mobile_no": "+1234567890",
  "device_id": "device_123456789",
  "session_token": "session_123456789",
  "otp": 123456,
  "is_new_user": true,
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "login:success"
}
```

**Response Fields**:
- `status` (string): "success"
- `message` (string): Success message
- `mobile_no` (string): User's mobile number
- `device_id` (string): Device identifier
- `session_token` (string): Session token for subsequent requests
- `otp` (number): 6-digit OTP for verification
- `is_new_user` (boolean): Whether this is a new user registration
- `timestamp` (string): ISO 8601 timestamp
- `socket_id` (string): Socket identifier
- `event` (string): Event type ("login:success")

### 5. OTP Verification
**Event**: `verify:otp`
**Direction**: Client → Server
**Purpose**: Verify OTP sent during login

**Request Data**:
```json
{
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "otp": "123456"
}
```

**Required Fields**:
- `mobile_no` (string): Mobile number
- `session_token` (string): Session token from login response
- `otp` (string): 6-digit OTP code

**Response Event**: `otp:verified`
**Response Data**:
```json
{
  "status": "success",
  "message": "OTP verification successful. Authentication completed.",
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "user_status": "new_user",
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "otp:verified"
}
```

**Response Fields**:
- `user_status` (string): Indicates if the user is new or existing (`new_user`, `existing_user`)

---

## 👤 User Profile Events

### 6. Set User Profile
**Event**: `set:profile`
**Direction**: Client → Server
**Purpose**: Set or update user profile information

**Request Data**:
```json
{
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "full_name": "John Doe",
  "state": "California",
  "referral_code": "JOHN123",
  "referred_by": "FRIEND456",
  "profile_data": {
    "avatar": "avatar_url",
    "bio": "Gaming enthusiast",
    "preferences": {
      "notifications": true,
      "privacy": "public"
    }
  }
}
```

**Required Fields**:
- `mobile_no` (string): Mobile number
- `session_token` (string): Session token
- `full_name` (string): User's full name
- `state` (string): User's state/location

**Optional Fields**:
- `referral_code` (string): User's referral code
- `referred_by` (string): Referral code of user who referred this user
- `profile_data` (object): Additional profile information

**Response Event**: `profile:set`
**Response Data**:
```json
{
  "status": "success",
  "message": "User profile updated successfully! 🎉",
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "full_name": "John Doe",
  "state": "California",
  "referral_code": "JOHN123",
  "referred_by": "FRIEND456",
  "profile_data": {
    "avatar": "avatar_url",
    "bio": "Gaming enthusiast",
    "preferences": {
      "notifications": true,
      "privacy": "public"
    }
  },
  "welcome_message": "Welcome John Doe! Your profile has been set up successfully.",
  "next_steps": "You can now proceed to set your language preferences.",
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "profile:set"
}
```

---

## 🌐 Language Setting Events

### 7. Set Language Preferences
**Event**: `set:language`
**Direction**: Client → Server
**Purpose**: Set user's language and regional preferences

**Request Data**:
```json
{
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "language_code": "en",
  "language_name": "English",
  "region_code": "US",
  "timezone": "America/Los_Angeles",
  "user_preferences": {
    "date_format": "MM/DD/YYYY",
    "time_format": "12h",
    "currency": "USD"
  }
}
```

**Required Fields**:
- `mobile_no` (string): Mobile number
- `session_token` (string): Session token
- `language_code` (string): Language code (en, es, fr, de, hi, zh, ja, ko, ar, pt, ru)
- `language_name` (string): Language name

**Optional Fields**:
- `region_code` (string): Region/country code
- `timezone` (string): Timezone identifier
- `user_preferences` (object): Additional user preferences

**Response Event**: `language:set`
**Response Data**:
```json
{
  "status": "success",
  "message": "Welcome to Game Admin! 🎮",
  "mobile_no": "+1234567890",
  "session_token": "session_123456789",
  "language_code": "en",
  "language_name": "English",
  "region_code": "US",
  "timezone": "America/Los_Angeles",
  "user_preferences": {
    "date_format": "MM/DD/YYYY",
    "time_format": "12h",
    "currency": "USD"
  },
  "localized_messages": {
    "welcome": "Welcome to Game Admin! 🎮",
    "setup_complete": "Setup completed successfully! ✅",
    "ready_to_play": "You're all set to start gaming! 🚀",
    "next_steps": "Explore the dashboard and start managing your game experience."
  },
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "language:set"
}
```

---

## ❌ Error Events

### 8. Connection Error
**Event**: `connection_error`
**Direction**: Server → Client
**Purpose**: Send error responses for any failed operations

**Error Data Structure**:
```json
{
  "status": "error",
  "error_code": "ERROR_CODE",
  "error_type": "ERROR_TYPE",
  "field": "field_name",
  "message": "Human readable error message",
  "details": {
    "additional_info": "Additional error details",
    "suggestions": "How to fix the error"
  },
  "timestamp": "2024-01-15T10:30:00Z",
  "socket_id": "socket_123456",
  "event": "connection_error"
}
```

**Common Error Codes**:
- `MISSING_FIELD`: Required field is missing
- `INVALID_FORMAT`: Data format is invalid
- `EMPTY_FIELD`: Field cannot be empty
- `INVALID_TYPE`: Field has wrong data type
- `INVALID_SESSION`: Session token is invalid
- `INVALID_OTP`: OTP verification failed
- `MAX_ATTEMPTS_EXCEEDED`: Too many OTP attempts
- `REFERRAL_CODE_EXISTS`: Referral code already exists
- `VERIFICATION_ERROR`: System verification error
- `SESSION_VERIFICATION_ERROR`: Session verification failed

**Error Types**:
- `FIELD_ERROR`: Field validation error
- `FORMAT_ERROR`: Data format error
- `VALUE_ERROR`: Invalid value error
- `TYPE_ERROR`: Data type error
- `AUTHENTICATION_ERROR`: Authentication failure
- `OTP_ERROR`: OTP-related error
- `VALIDATION_ERROR`: General validation error
- `SYSTEM_ERROR`: System-level error

---

## 🔄 Event Flow Diagrams

### Complete User Registration Flow
```
1. Client connects → Server sends connect_response
2. Client sends device:info → Server responds with device:info:ack
3. Client sends login → Server responds with login:success (includes OTP)
4. Client sends verify:otp → Server responds with otp:verified
5. Client sends set:profile → Server responds with profile:set
6. Client sends set:language → Server responds with language:set
```

### Error Handling Flow
```
1. Client sends invalid data → Server validates
2. If validation fails → Server sends connection_error
3. Client receives error → Client can retry with correct data
4. Server logs error → Error stored in MongoDB for analytics
```

---

## 📊 Data Validation Rules

### Mobile Number
- Must be a string
- Must include country code
- Cannot be empty
- Format: `+[country_code][number]`

### Device ID
- Must be a string
- Cannot be empty
- Minimum length: 1 character
- Should be unique per device

### Session Token
- Must be a string
- Generated by server during login
- Required for authenticated operations
- Validated on each request

### OTP
- Must be a string
- Must be exactly 6 digits
- Maximum 5 attempts allowed
- Expires after verification

### Language Code
- Must be a supported language code
- Supported codes: en, es, fr, de, hi, zh, ja, ko, ar, pt, ru
- Default: "en" (English)

---

## 🗄️ Database Storage

All events are automatically stored in MongoDB collections:
- `connect_events`: Connection responses
- `device_info_events`: Device information
- `login_events`: Login attempts
- `login_success_events`: Successful logins
- `otp_verification_events`: OTP verifications
- `user_profile_events`: Profile updates
- `language_setting_events`: Language preferences
- `connection_error_events`: Error logs
- `userregister`: User registration data

---

## 🔧 Testing

Use the provided test files in `test-client/` directory:
- `test-all.js`: Complete test suite
- `test-login.js`: Login flow testing
- `test-otp.js`: OTP verification testing
- `test-device.js`: Device info testing
- `test-user-profile.js`: Profile management testing
- `test-language-setting.js`: Language setting testing

---

## 📝 Notes

1. **Timestamps**: All timestamps are in ISO 8601 format (UTC)
2. **Socket IDs**: Automatically generated by Socket.IO
3. **Session Tokens**: Valid for the duration of the socket connection
4. **Error Handling**: All errors include detailed information for debugging
5. **Localization**: Success messages are localized based on language preference
6. **Security**: Session tokens are validated on each authenticated request
7. **Logging**: All events are logged for analytics and debugging
8. **Validation**: Comprehensive validation for all input data

---

*Last Updated: January 2024*
*Version: 1.0*
*Backend: Rust with Socket.IO*
*Database: MongoDB* 