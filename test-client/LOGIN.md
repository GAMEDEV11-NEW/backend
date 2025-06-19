# Login System Documentation

## Overview

The login system is designed for mobile device authentication using mobile number, device ID, and FCM (Firebase Cloud Messaging) token. This document outlines the complete login flow, validation rules, and testing procedures.

## 🔐 Login Flow

### 1. Client Connection
```
Client connects to Socket.IO server
↓
Server sends connect_response with token
↓
Client receives connection acknowledgment
```

### 2. Login Request
```
Client sends login event with credentials
↓
Server validates login data
↓
Server responds with success/error
```

### 3. Authentication Response
```
Success: login:success event with session token
Error: connection_error event with detailed error info (unified error handling)
```

## 📋 Required Fields

### Mandatory Fields
| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `mobile_no` | string | User's mobile number | 10-15 digits only |
| `device_id` | string | Unique device identifier | 3-50 chars, alphanumeric + _ - |
| `fcm_token` | string | Firebase Cloud Messaging token | 100-500 characters |

### Optional Fields
| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `timestamp` | string | ISO 8601 timestamp | Optional, must be ISO format if provided |

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

### Timestamp Validation (Optional)
```javascript
// Must be ISO 8601 format if provided
timestamp: "2024-01-15T10:30:00Z"  // ✅ Valid
timestamp: "2024-01-15 10:30:00"   // ❌ Wrong format
```

## 📤 Request Format

### Valid Login Request
```json
{
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "fcm_token": "fcm_token_example_abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

### Minimal Login Request (without timestamp)
```json
{
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "fcm_token": "fcm_token_example_abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890"
}
```

## 📥 Response Format

### Successful Login Response
```json
{
    "status": "success",
    "message": "Login successful",
    "mobile_no": "9876543210",
    "device_id": "device_001",
    "session_token": 123456789,
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "login:success"
}
```

### Error Response Format (Unified)
```json
{
    "status": "error",
    "error_code": "INVALID_FORMAT",
    "error_type": "FORMAT_ERROR",
    "field": "mobile_no",
    "message": "mobile_no must contain only digits",
    "details": {
        "allowed_characters": "digits only",
        "received_value": "98A6543210",
        "required": true
    },
    "timestamp": "2024-01-15T10:30:00Z",
    "socket_id": "socket_connection_id",
    "event": "connection_error"
}
```

## 🚨 Error Codes

| Error Code | Error Type | Description | Common Causes |
|------------|------------|-------------|---------------|
| `MISSING_FIELD` | `FIELD_ERROR` | Required field is missing | Field not provided in request |
| `EMPTY_FIELD` | `VALUE_ERROR` | Field is empty | Field provided but with empty value |
| `INVALID_FORMAT` | `FORMAT_ERROR` | Field format is invalid | Wrong character types or format |
| `INVALID_LENGTH` | `LENGTH_ERROR` | Field length is outside allowed range | Too short or too long |
| `INVALID_TYPE` | `TYPE_ERROR` | Field has wrong data type | String expected but number provided |

## 🧪 Testing Procedures

### Running Tests
```bash
# Navigate to test-client directory
cd test-client

# Install dependencies
npm install

# Run login tests
node test-login.js
```

### Test Cases Covered
1. **Valid Login** - All fields correct
2. **Missing Fields** - mobile_no, device_id, fcm_token
3. **Empty Fields** - Empty values for required fields
4. **Invalid Mobile** - Non-digits, too short, too long
5. **Invalid Device ID** - Special characters, wrong length
6. **Invalid FCM Token** - Too short, too long
7. **Invalid Timestamp** - Wrong format

### Test Output Example
```
🚀 Starting Login Tests...

📋 Test: Valid login with all required fields
📤 Sending data: {
  "mobile_no": "9876543210",
  "device_id": "device_001",
  "fcm_token": "fcm_token_example_...",
  "timestamp": "2024-01-15T10:30:00Z"
}
   🔌 Connected to server (socket ID: abc123)
   📥 Received login:success: {...}
✅ PASSED - Expected: success, Got: success
──────────────────────────────────────────────────

📋 Test: Missing mobile_no
📤 Sending data: {
  "device_id": "device_001",
  "fcm_token": "fcm_token_example_..."
}
   🔌 Connected to server (socket ID: abc124)
   📥 Received connection_error: {...}
✅ PASSED - Expected: error, Got: error
──────────────────────────────────────────────────

📊 Test Results:
✅ Passed: 8
❌ Failed: 2
📈 Success Rate: 80.0%
```

## 🔧 Implementation Notes

### Backend Validation
- All validation is performed server-side using Rust
- Validation errors include detailed information for debugging
- Session tokens are generated randomly (9-digit numbers)
- Timestamps are in ISO 8601 format (UTC)

### Client Requirements
- Must connect via Socket.IO
- Must handle both success and error responses
- Should implement proper error handling for network issues
- Should validate data client-side before sending (for UX)

### Security Considerations
- FCM tokens should be obtained from Firebase SDK
- Session tokens are temporary and should be refreshed
- All communication should be over HTTPS/WSS
- Implement rate limiting for login attempts

## 📚 Additional Resources

- [Socket.IO Documentation](https://socket.io/docs/)
- [Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging)
- [ISO 8601 Date Format](https://en.wikipedia.org/wiki/ISO_8601)
- [Rust Backend Source Code](../src/managers/validation.rs)