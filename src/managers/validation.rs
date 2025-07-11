use serde_json::{json, Value};
use tracing::info;

// Error details structure
#[derive(Debug)]
pub struct ValidationError {
    pub code: String,
    pub error_type: String,
    pub field: String,
    pub message: String,
    pub details: Value,
}

pub struct ValidationManager;

impl ValidationManager {
    // Validate device info data
    pub fn validate_device_info(data: &Value) -> Result<(), ValidationError> {
        // Check if data is an object
        let obj = data.as_object().ok_or(ValidationError {
            code: "INVALID_FORMAT".to_string(),
            error_type: "FORMAT_ERROR".to_string(),
            field: "root".to_string(),
            message: "Device info must be a JSON object".to_string(),
            details: json!({"received_type": if data.is_object() { "object" } else if data.is_array() { "array" } else if data.is_string() { "string" } else if data.is_number() { "number" } else if data.is_boolean() { "boolean" } else { "null" }}),
        })?;
        
        // Required fields (mandatory)
        let device_id = obj
            .get("device_id")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let device_type =
            obj.get("device_type")
                .and_then(|v| v.as_str())
                .ok_or(ValidationError {
                    code: "MISSING_FIELD".to_string(),
                    error_type: "FIELD_ERROR".to_string(),
                    field: "device_type".to_string(),
                    message: "device_type is required and must be a string".to_string(),
                    details: json!({"field_type": "string", "required": true}),
                })?;
        
        let timestamp = obj
            .get("timestamp")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "timestamp".to_string(),
                message: "timestamp is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        // Optional fields (not mandatory)
        let manufacturer = obj.get("manufacturer").and_then(|v| v.as_str());
        let model = obj.get("model").and_then(|v| v.as_str());
        let firmware_version = obj.get("firmware_version").and_then(|v| v.as_str());
        let capabilities = obj.get("capabilities").and_then(|v| v.as_array());
        
        // Validate required field values
        if device_id.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if device_type.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "device_type".to_string(),
                message: "device_type cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        // Validate optional fields if they are present
        if let Some(manufacturer_val) = manufacturer {
            if manufacturer_val.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "manufacturer".to_string(),
                    message: "manufacturer cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
        }
        
        if let Some(model_val) = model {
            if model_val.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "model".to_string(),
                    message: "model cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
        }
        
        if let Some(firmware_val) = firmware_version {
            if firmware_val.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "firmware_version".to_string(),
                    message: "firmware_version cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
        }
        
        if let Some(capabilities_val) = capabilities {
            if capabilities_val.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "capabilities".to_string(),
                    message: "capabilities cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
            
            // Validate capabilities array contains only strings
            for (index, capability) in capabilities_val.iter().enumerate() {
                if !capability.is_string() {
                    return Err(ValidationError {
                        code: "INVALID_TYPE".to_string(),
                        error_type: "TYPE_ERROR".to_string(),
                        field: format!("capabilities[{}]", index),
                        message: "all capabilities must be strings".to_string(),
                        details: json!({
                            "expected_type": "string",
                            "received_type": if capability.is_string() { "string" } else if capability.is_number() { "number" } else if capability.is_boolean() { "boolean" } else if capability.is_array() { "array" } else if capability.is_object() { "object" } else { "null" },
                            "received_value": capability,
                            "array_index": index,
                            "required": false
                        }),
                    });
                }
            }
        }
        
        // Validate timestamp format (basic ISO format check)
        if !timestamp.contains('T') || !timestamp.contains('Z') {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "timestamp".to_string(),
                message: "timestamp must be in ISO format (e.g., 2024-01-15T10:30:00Z)".to_string(),
                details: json!({
                    "expected_format": "ISO 8601",
                    "example": "2024-01-15T10:30:00Z",
                    "received_value": timestamp,
                    "required": true
                }),
            });
        }
        
        info!("✅ Device info validation passed for device: {}", device_id);
        Ok(())
    }

    // Validate login data
    pub fn validate_login_data(data: &Value) -> Result<(), ValidationError> {
        // Check if data is an object
        let obj = data.as_object().ok_or(ValidationError {
            code: "INVALID_FORMAT".to_string(),
            error_type: "FORMAT_ERROR".to_string(),
            field: "root".to_string(),
            message: "Login data must be a JSON object".to_string(),
            details: json!({"received_type": if data.is_object() { "object" } else if data.is_array() { "array" } else if data.is_string() { "string" } else if data.is_number() { "number" } else if data.is_boolean() { "boolean" } else { "null" }}),
        })?;
        
        // Required fields (mandatory)
        let mobile_no = obj
            .get("mobile_no")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let device_id = obj
            .get("device_id")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let fcm_token = obj
            .get("fcm_token")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "fcm_token".to_string(),
                message: "fcm_token is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        // Optional fields
        let timestamp = obj.get("timestamp").and_then(|v| v.as_str());
        
        // Validate required field values
        if mobile_no.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if device_id.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if fcm_token.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "fcm_token".to_string(),
                message: "fcm_token cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        // Validate mobile number format (basic validation for 10-15 digits)
        if !mobile_no.chars().all(|c| c.is_digit(10)) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must contain only digits".to_string(),
                details: json!({
                    "allowed_characters": "digits only",
                    "received_value": mobile_no,
                    "required": true
                }),
            });
        }
        
        if mobile_no.len() < 10 || mobile_no.len() > 15 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must be between 10 and 15 digits".to_string(),
                details: json!({
                    "min_length": 10,
                    "max_length": 15,
                    "received_length": mobile_no.len(),
                    "required": true
                }),
            });
        }
        
        // Validate device_id format (alphanumeric and underscore only, 3-50 characters)
        if !device_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id must contain only alphanumeric characters, underscores, and hyphens".to_string(),
                details: json!({
                    "allowed_characters": "alphanumeric, underscore, hyphen",
                    "received_value": device_id,
                    "required": true
                }),
            });
        }
        
        if device_id.len() < 3 || device_id.len() > 50 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "device_id".to_string(),
                message: "device_id must be between 3 and 50 characters".to_string(),
                details: json!({
                    "min_length": 3,
                    "max_length": 50,
                    "received_length": device_id.len(),
                    "required": true
                }),
            });
        }
        
        // Validate FCM token format (basic validation for Firebase token)
        if fcm_token.len() < 100 || fcm_token.len() > 500 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "fcm_token".to_string(),
                message: "fcm_token must be between 100 and 500 characters".to_string(),
                details: json!({
                    "min_length": 100,
                    "max_length": 500,
                    "received_length": fcm_token.len(),
                    "required": true
                }),
            });
        }
        
        // Validate optional timestamp if provided
        if let Some(timestamp_val) = timestamp {
            if !timestamp_val.contains('T') || !timestamp_val.contains('Z') {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "timestamp".to_string(),
                    message: "timestamp must be in ISO format (e.g., 2024-01-15T10:30:00Z)".to_string(),
                    details: json!({
                        "expected_format": "ISO 8601",
                        "example": "2024-01-15T10:30:00Z",
                        "received_value": timestamp_val,
                        "required": false
                    }),
                });
            }
        }
        
        info!("✅ Login data validation passed for mobile: {}", mobile_no);
        Ok(())
    }

    // Validate OTP verification data
    pub fn validate_otp_data(data: &Value) -> Result<(), ValidationError> {
        // Check if data is an object
        let obj = data.as_object().ok_or(ValidationError {
            code: "INVALID_FORMAT".to_string(),
            error_type: "FORMAT_ERROR".to_string(),
            field: "root".to_string(),
            message: "OTP data must be a JSON object".to_string(),
            details: json!({"received_type": if data.is_object() { "object" } else if data.is_array() { "array" } else if data.is_string() { "string" } else if data.is_number() { "number" } else if data.is_boolean() { "boolean" } else { "null" }}),
        })?;
        
        // Required fields (mandatory)
        let mobile_no = obj
            .get("mobile_no")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let otp = obj
            .get("otp")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "otp".to_string(),
                message: "otp is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let session_token = obj
            .get("session_token")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        // Optional fields
        let timestamp = obj.get("timestamp").and_then(|v| v.as_str());
        
        // Validate required field values
        if mobile_no.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if otp.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "otp".to_string(),
                message: "otp cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        // Validate mobile number format (basic validation for 10-15 digits)
        if !mobile_no.chars().all(|c| c.is_digit(10)) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must contain only digits".to_string(),
                details: json!({
                    "allowed_characters": "digits only",
                    "received_value": mobile_no,
                    "required": true
                }),
            });
        }
        
        if mobile_no.len() < 10 || mobile_no.len() > 15 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must be between 10 and 15 digits".to_string(),
                details: json!({
                    "min_length": 10,
                    "max_length": 15,
                    "received_length": mobile_no.len(),
                    "required": true
                }),
            });
        }
        
        // Validate OTP format (6 digits only)
        if !otp.chars().all(|c| c.is_digit(10)) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "otp".to_string(),
                message: "otp must contain only digits".to_string(),
                details: json!({
                    "allowed_characters": "digits only",
                    "received_value": otp,
                    "required": true
                }),
            });
        }
        
        if otp.len() != 6 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "otp".to_string(),
                message: "otp must be exactly 6 digits".to_string(),
                details: json!({
                    "expected_length": 6,
                    "received_length": otp.len(),
                    "required": true
                }),
            });
        }
        
        // Validate session token (should not be empty)
        if session_token.is_empty() {
            return Err(ValidationError {
                code: "INVALID_VALUE".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token cannot be empty".to_string(),
                details: json!({
                    "min_length": 1,
                    "received_length": session_token.len(),
                    "required": true
                }),
            });
        }
        
        // Validate optional timestamp if provided
        if let Some(timestamp_val) = timestamp {
            if !timestamp_val.contains('T') || !timestamp_val.contains('Z') {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "timestamp".to_string(),
                    message: "timestamp must be in ISO format (e.g., 2024-01-15T10:30:00Z)".to_string(),
                    details: json!({
                        "expected_format": "ISO 8601",
                        "example": "2024-01-15T10:30:00Z",
                        "received_value": timestamp_val,
                        "required": false
                    }),
                });
            }
        }
        
        info!("✅ OTP data validation passed for mobile: {}", mobile_no);
        Ok(())
    }

    // Validate language setting data
    pub fn validate_language_setting_data(data: &Value) -> Result<(), ValidationError> {
        // Check if data is an object
        let obj = data.as_object().ok_or(ValidationError {
            code: "INVALID_FORMAT".to_string(),
            error_type: "FORMAT_ERROR".to_string(),
            field: "root".to_string(),
            message: "Language setting data must be a JSON object".to_string(),
            details: json!({"received_type": if data.is_object() { "object" } else if data.is_array() { "array" } else if data.is_string() { "string" } else if data.is_number() { "number" } else if data.is_boolean() { "boolean" } else { "null" }}),
        })?;
        
        // Required fields (mandatory)
        let mobile_no = obj
            .get("mobile_no")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let session_token = obj
            .get("session_token")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let language_code = obj
            .get("language_code")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "language_code".to_string(),
                message: "language_code is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let language_name = obj
            .get("language_name")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "language_name".to_string(),
                message: "language_name is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        // Optional fields
        let region_code = obj.get("region_code").and_then(|v| v.as_str());
        let timezone = obj.get("timezone").and_then(|v| v.as_str());
        let _user_preferences = obj.get("user_preferences");
        let timestamp = obj.get("timestamp").and_then(|v| v.as_str());
        
        // Validate required field values
        if mobile_no.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if session_token.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if language_code.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "language_code".to_string(),
                message: "language_code cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if language_name.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "language_name".to_string(),
                message: "language_name cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        // Validate mobile number format (basic validation for 10-15 digits)
        if !mobile_no.chars().all(|c| c.is_digit(10)) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must contain only digits".to_string(),
                details: json!({
                    "allowed_characters": "digits only",
                    "received_value": mobile_no,
                    "required": true
                }),
            });
        }
        
        if mobile_no.len() < 10 || mobile_no.len() > 15 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must be between 10 and 15 digits".to_string(),
                details: json!({
                    "min_length": 10,
                    "max_length": 15,
                    "received_length": mobile_no.len(),
                    "required": true
                }),
            });
        }
        
        // Validate language code format (ISO 639-1: 2 letters)
        if !language_code.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "language_code".to_string(),
                message: "language_code must contain only lowercase letters".to_string(),
                details: json!({
                    "allowed_characters": "lowercase letters only",
                    "expected_format": "ISO 639-1 (2 letters)",
                    "received_value": language_code,
                    "required": true
                }),
            });
        }
        
        if language_code.len() != 2 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "language_code".to_string(),
                message: "language_code must be exactly 2 characters".to_string(),
                details: json!({
                    "expected_length": 2,
                    "received_length": language_code.len(),
                    "required": true
                }),
            });
        }
        
        // Validate language name (should be reasonable length)
        if language_name.len() < 2 || language_name.len() > 50 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "language_name".to_string(),
                message: "language_name must be between 2 and 50 characters".to_string(),
                details: json!({
                    "min_length": 2,
                    "max_length": 50,
                    "received_length": language_name.len(),
                    "required": true
                }),
            });
        }
        
        // Validate optional region code if provided (ISO 3166-1 alpha-2: 2 uppercase letters)
        if let Some(region_val) = region_code {
            if !region_val.chars().all(|c| c.is_ascii_uppercase()) {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "region_code".to_string(),
                    message: "region_code must contain only uppercase letters".to_string(),
                    details: json!({
                        "allowed_characters": "uppercase letters only",
                        "expected_format": "ISO 3166-1 alpha-2 (2 letters)",
                        "received_value": region_val,
                        "required": false
                    }),
                });
            }
            
            if region_val.len() != 2 {
                return Err(ValidationError {
                    code: "INVALID_LENGTH".to_string(),
                    error_type: "LENGTH_ERROR".to_string(),
                    field: "region_code".to_string(),
                    message: "region_code must be exactly 2 characters".to_string(),
                    details: json!({
                        "expected_length": 2,
                        "received_length": region_val.len(),
                        "required": false
                    }),
                });
            }
        }
        
        // Validate optional timezone if provided (basic format check)
        if let Some(timezone_val) = timezone {
            if timezone_val.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "timezone".to_string(),
                    message: "timezone cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
            
            if timezone_val.len() < 3 || timezone_val.len() > 50 {
                return Err(ValidationError {
                    code: "INVALID_LENGTH".to_string(),
                    error_type: "LENGTH_ERROR".to_string(),
                    field: "timezone".to_string(),
                    message: "timezone must be between 3 and 50 characters".to_string(),
                    details: json!({
                        "min_length": 3,
                        "max_length": 50,
                        "received_length": timezone_val.len(),
                        "required": false
                    }),
                });
            }
        }
        
        // Validate optional timestamp if provided
        if let Some(timestamp_val) = timestamp {
            if !timestamp_val.contains('T') || !timestamp_val.contains('Z') {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "timestamp".to_string(),
                    message: "timestamp must be in ISO format (e.g., 2024-01-15T10:30:00Z)".to_string(),
                    details: json!({
                        "expected_format": "ISO 8601",
                        "example": "2024-01-15T10:30:00Z",
                        "received_value": timestamp_val,
                        "required": false
                    }),
                });
            }
        }
        
        info!("✅ Language setting data validation passed for mobile: {} (language: {})", mobile_no, language_code);
        Ok(())
    }

    // Validate user profile data
    pub fn validate_user_profile_data(data: &Value) -> Result<(), ValidationError> {
        // Check if data is an object
        let obj = data.as_object().ok_or(ValidationError {
            code: "INVALID_FORMAT".to_string(),
            error_type: "FORMAT_ERROR".to_string(),
            field: "root".to_string(),
            message: "User profile data must be a JSON object".to_string(),
            details: json!({"received_type": if data.is_object() { "object" } else if data.is_array() { "array" } else if data.is_string() { "string" } else if data.is_number() { "number" } else if data.is_boolean() { "boolean" } else { "null" }}),
        })?;
        
        // Required fields (mandatory)
        let mobile_no = obj
            .get("mobile_no")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let session_token = obj
            .get("session_token")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let full_name = obj
            .get("full_name")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "full_name".to_string(),
                message: "full_name is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        let state = obj
            .get("state")
            .and_then(|v| v.as_str())
            .ok_or(ValidationError {
                code: "MISSING_FIELD".to_string(),
                error_type: "FIELD_ERROR".to_string(),
                field: "state".to_string(),
                message: "state is required and must be a string".to_string(),
                details: json!({"field_type": "string", "required": true}),
            })?;
        
        // Optional fields
        let referral_code = obj.get("referral_code").and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty());
        let referred_by = obj.get("referred_by").and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty());
        let _profile_data = obj.get("profile_data");
        let timestamp = obj.get("timestamp").and_then(|v| v.as_str());
        
        // Validate required field values
        if mobile_no.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if session_token.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "session_token".to_string(),
                message: "session_token cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if full_name.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "full_name".to_string(),
                message: "full_name cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        if state.is_empty() {
            return Err(ValidationError {
                code: "EMPTY_FIELD".to_string(),
                error_type: "VALUE_ERROR".to_string(),
                field: "state".to_string(),
                message: "state cannot be empty".to_string(),
                details: json!({"min_length": 1, "received_length": 0, "required": true}),
            });
        }
        
        // Validate mobile number format (basic validation for 10-15 digits)
        if !mobile_no.chars().all(|c| c.is_digit(10)) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must contain only digits".to_string(),
                details: json!({
                    "allowed_characters": "digits only",
                    "received_value": mobile_no,
                    "required": true
                }),
            });
        }
        
        if mobile_no.len() < 10 || mobile_no.len() > 15 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "mobile_no".to_string(),
                message: "mobile_no must be between 10 and 15 digits".to_string(),
                details: json!({
                    "min_length": 10,
                    "max_length": 15,
                    "received_length": mobile_no.len(),
                    "required": true
                }),
            });
        }
        
        // Validate full name (should be reasonable length and contain letters)
        if full_name.len() < 2 || full_name.len() > 100 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "full_name".to_string(),
                message: "full_name must be between 2 and 100 characters".to_string(),
                details: json!({
                    "min_length": 2,
                    "max_length": 100,
                    "received_length": full_name.len(),
                    "required": true
                }),
            });
        }
        
        // Check if full name contains at least some letters
        if !full_name.chars().any(|c| c.is_alphabetic()) {
            return Err(ValidationError {
                code: "INVALID_FORMAT".to_string(),
                error_type: "FORMAT_ERROR".to_string(),
                field: "full_name".to_string(),
                message: "full_name must contain at least some letters".to_string(),
                details: json!({
                    "required_characters": "at least one letter",
                    "received_value": full_name,
                    "required": true
                }),
            });
        }
        
        // Validate state (should be reasonable length)
        if state.len() < 2 || state.len() > 50 {
            return Err(ValidationError {
                code: "INVALID_LENGTH".to_string(),
                error_type: "LENGTH_ERROR".to_string(),
                field: "state".to_string(),
                message: "state must be between 2 and 50 characters".to_string(),
                details: json!({
                    "min_length": 2,
                    "max_length": 50,
                    "received_length": state.len(),
                    "required": true
                }),
            });
        }
        
        // Validate optional referral code if provided
        if let Some(ref_code) = referral_code {
            if ref_code.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "referral_code".to_string(),
                    message: "referral_code cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
            
            if ref_code.len() < 4 || ref_code.len() > 20 {
                return Err(ValidationError {
                    code: "INVALID_LENGTH".to_string(),
                    error_type: "LENGTH_ERROR".to_string(),
                    field: "referral_code".to_string(),
                    message: "referral_code must be between 4 and 20 characters".to_string(),
                    details: json!({
                        "min_length": 4,
                        "max_length": 20,
                        "received_length": ref_code.len(),
                        "required": false
                    }),
                });
            }
            
            // Check if referral code contains only alphanumeric characters
            if !ref_code.chars().all(|c| c.is_alphanumeric()) {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "referral_code".to_string(),
                    message: "referral_code must contain only alphanumeric characters".to_string(),
                    details: json!({
                        "allowed_characters": "alphanumeric only",
                        "received_value": ref_code,
                        "required": false
                    }),
                });
            }
        }
        
        // Validate optional referred_by if provided
        if let Some(ref_by) = referred_by {
            if ref_by.is_empty() {
                return Err(ValidationError {
                    code: "EMPTY_FIELD".to_string(),
                    error_type: "VALUE_ERROR".to_string(),
                    field: "referred_by".to_string(),
                    message: "referred_by cannot be empty if provided".to_string(),
                    details: json!({"min_length": 1, "received_length": 0, "required": false}),
                });
            }
            
            if ref_by.len() < 4 || ref_by.len() > 20 {
                return Err(ValidationError {
                    code: "INVALID_LENGTH".to_string(),
                    error_type: "LENGTH_ERROR".to_string(),
                    field: "referred_by".to_string(),
                    message: "referred_by must be between 4 and 20 characters".to_string(),
                    details: json!({
                        "min_length": 4,
                        "max_length": 20,
                        "received_length": ref_by.len(),
                        "required": false
                    }),
                });
            }
            
            // Check if referred_by contains only alphanumeric characters
            if !ref_by.chars().all(|c| c.is_alphanumeric()) {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "referred_by".to_string(),
                    message: "referred_by must contain only alphanumeric characters".to_string(),
                    details: json!({
                        "allowed_characters": "alphanumeric only",
                        "received_value": ref_by,
                        "required": false
                    }),
                });
            }
        }
        
        // Validate optional timestamp if provided
        if let Some(timestamp_val) = timestamp {
            if !timestamp_val.contains('T') || !timestamp_val.contains('Z') {
                return Err(ValidationError {
                    code: "INVALID_FORMAT".to_string(),
                    error_type: "FORMAT_ERROR".to_string(),
                    field: "timestamp".to_string(),
                    message: "timestamp must be in ISO format (e.g., 2024-01-15T10:30:00Z)".to_string(),
                    details: json!({
                        "expected_format": "ISO 8601",
                        "example": "2024-01-15T10:30:00Z",
                        "received_value": timestamp_val,
                        "required": false
                    }),
                });
            }
        }
        
        info!("✅ User profile data validation passed for mobile: {} (name: {})", mobile_no, full_name);
        Ok(())
    }
} 