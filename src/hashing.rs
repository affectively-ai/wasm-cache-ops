use sha2::{Sha256, Digest};
use hex;

/// Hash a query JSON string for fingerprinting
/// Uses SHA-256 for deterministic hashing
pub fn hash_query(query_json: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(query_json.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_query() {
        let query1 = r#"{"collection":"users","where":{"age":">30"}}"#;
        let query2 = r#"{"collection":"users","where":{"age":">30"}}"#;
        let query3 = r#"{"collection":"users","where":{"age":">31"}}"#;
        
        let hash1 = hash_query(query1);
        let hash2 = hash_query(query2);
        let hash3 = hash_query(query3);
        
        // Same query should produce same hash
        assert_eq!(hash1, hash2);
        // Different query should produce different hash
        assert_ne!(hash1, hash3);
        // Hash should be 64 characters (SHA-256 hex)
        assert_eq!(hash1.len(), 64);
    }
}
