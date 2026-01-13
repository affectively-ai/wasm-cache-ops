use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
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

    // Group operations by type
    let mut grouped: HashMap<String, Vec<BatchOperation>> = HashMap::new();
    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    for op in batch.operations {
        // Deduplicate: keep only the last operation for each key
        if seen_keys.contains(&op.key) {
            // Remove previous operation with same key
            if let Some(ops) = grouped.get_mut(&op.op_type) {
                ops.retain(|o| o.key != op.key);
            }
        }
        seen_keys.insert(op.key.clone());
        grouped.entry(op.op_type.clone()).or_insert_with(Vec::new).push(op);
    }

    // Flatten back to operations array
    let mut optimized: Vec<BatchOperation> = Vec::new();
    for (_op_type, ops) in grouped {
        optimized.extend(ops);
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
}
