use serde_json::{self, json};

/// Serialize a document JSON string (optimized)
/// Validates and minifies JSON
/// Returns an error JSON object if the input is not valid JSON
pub fn serialize_document(doc_json: &str) -> String {
    // Parse and re-stringify to ensure valid JSON and remove whitespace
    match serde_json::from_str::<serde_json::Value>(doc_json) {
        Ok(value) => {
            // Use compact format (no whitespace)
            serde_json::to_string(&value).unwrap_or_else(|_| doc_json.to_string())
        }
        Err(e) => {
            // Return error JSON so callers can detect the failure
            json!({"error": format!("Invalid JSON: {}", e)}).to_string()
        }
    }
}

/// Deserialize a document JSON string (optimized)
/// Validates JSON structure
/// Returns an error JSON object if the input is not valid JSON
pub fn deserialize_document(serialized: &str) -> String {
    // Validate JSON structure
    match serde_json::from_str::<serde_json::Value>(serialized) {
        Ok(_) => serialized.to_string(),
        Err(e) => {
            // Return error JSON so callers can detect the failure
            json!({"error": format!("Invalid JSON: {}", e)}).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_document() {
        let json_str = r#"{"name":"John","age":30}"#;
        let serialized = serialize_document(json_str);
        assert_eq!(serialized, r#"{"age":30,"name":"John"}"#);
    }

    #[test]
    fn test_deserialize_document() {
        let json_str = r#"{"name":"John","age":30}"#;
        let deserialized = deserialize_document(json_str);
        assert_eq!(deserialized, json_str);
    }

    #[test]
    fn test_serialize_removes_whitespace() {
        let json_str = r#"{"name": "John", "age": 30}"#;
        let serialized = serialize_document(json_str);
        assert!(!serialized.contains(' '));
    }

    #[test]
    fn test_serialize_invalid_json_returns_error() {
        let invalid = "not valid json {{{";
        let result = serialize_document(invalid);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("error").is_some());
    }

    #[test]
    fn test_deserialize_invalid_json_returns_error() {
        let invalid = "not valid json";
        let result = deserialize_document(invalid);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("error").is_some());
    }
}
