use serde_json;

/// Serialize a document JSON string (optimized)
/// Validates and minifies JSON
pub fn serialize_document(doc_json: &str) -> String {
    // Parse and re-stringify to ensure valid JSON and remove whitespace
    match serde_json::from_str::<serde_json::Value>(doc_json) {
        Ok(value) => {
            // Use compact format (no whitespace)
            serde_json::to_string(&value).unwrap_or_else(|_| doc_json.to_string())
        }
        Err(_) => {
            // If invalid JSON, return as-is (caller should handle)
            doc_json.to_string()
        }
    }
}

/// Deserialize a document JSON string (optimized)
/// Validates JSON structure
pub fn deserialize_document(serialized: &str) -> String {
    // Validate JSON structure
    match serde_json::from_str::<serde_json::Value>(serialized) {
        Ok(_) => serialized.to_string(),
        Err(_) => {
            // Return empty object if invalid
            "{}".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_document() {
        let json = r#"{"name":"John","age":30}"#;
        let serialized = serialize_document(json);
        assert_eq!(serialized, r#"{"age":30,"name":"John"}"#);
    }

    #[test]
    fn test_deserialize_document() {
        let json = r#"{"name":"John","age":30}"#;
        let deserialized = deserialize_document(json);
        assert_eq!(deserialized, json);
    }

    #[test]
    fn test_serialize_removes_whitespace() {
        let json = r#"{"name": "John", "age": 30}"#;
        let serialized = serialize_document(json);
        assert!(!serialized.contains(' '));
    }
}
