use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct BatchOperation {
    #[serde(rename = "type")]
    op_type: String,
    key: String,
    value: Option<Value>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BatchOperations {
    operations: Vec<BatchOperation>,
}

/// Optimize batch operations
/// Groups operations by type and deduplicates
pub fn batch_operations(ops_json: &str) -> String {
    let batch: BatchOperations = match serde_json::from_str(ops_json) {
        Ok(b) => b,
        Err(_) => return json!({"error": "Invalid JSON"}).to_string(),
    };

    // Deduplicate: keep only the last operation for each key, preserving order
    let mut seen_keys: HashMap<String, usize> = HashMap::new();
    let mut optimized: Vec<BatchOperation> = Vec::new();

    for op in batch.operations {
        if let Some(&prev_idx) = seen_keys.get(&op.key) {
            // Replace the previous operation for this key
            optimized[prev_idx] = op.clone();
            // Update with new op_type in case it changed
            let key = op.key.clone();
            seen_keys.insert(key, prev_idx);
        } else {
            let idx = optimized.len();
            seen_keys.insert(op.key.clone(), idx);
            optimized.push(op);
        }
    }

    json!({
        "operations": optimized,
        "count": optimized.len()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_operations_deduplication() {
        let ops_json = r#"{
            "operations": [
                {"type": "set", "key": "key1", "value": "value1"},
                {"type": "set", "key": "key2", "value": "value2"},
                {"type": "set", "key": "key1", "value": "value1_updated"}
            ]
        }"#;

        let result = batch_operations(ops_json);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        let count = parsed["count"].as_u64().unwrap();

        // Should have 2 operations after deduplication
        assert_eq!(count, 2);
    }

    #[test]
    fn test_batch_operations_cross_type_deduplication() {
        let ops_json = r#"{
            "operations": [
                {"type": "set", "key": "key1", "value": "value1"},
                {"type": "delete", "key": "key1", "value": null}
            ]
        }"#;

        let result = batch_operations(ops_json);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        let count = parsed["count"].as_u64().unwrap();

        // Should have 1 operation: the delete replaces the set
        assert_eq!(count, 1);
        let ops = parsed["operations"].as_array().unwrap();
        assert_eq!(ops[0]["type"].as_str().unwrap(), "delete");
    }

    #[test]
    fn test_batch_operations_preserves_order() {
        let ops_json = r#"{
            "operations": [
                {"type": "set", "key": "a", "value": 1},
                {"type": "set", "key": "b", "value": 2},
                {"type": "set", "key": "c", "value": 3}
            ]
        }"#;

        let result = batch_operations(ops_json);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        let ops = parsed["operations"].as_array().unwrap();

        assert_eq!(ops[0]["key"].as_str().unwrap(), "a");
        assert_eq!(ops[1]["key"].as_str().unwrap(), "b");
        assert_eq!(ops[2]["key"].as_str().unwrap(), "c");
    }

    #[test]
    fn test_batch_operations_invalid_json() {
        let result = batch_operations("not json");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("error").is_some());
    }
}
