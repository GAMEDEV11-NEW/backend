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
} 