use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct CacheEntry {
    key: String,
    size: u64,
    last_access: u64, // Timestamp
    access_count: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntries {
    entries: Vec<CacheEntry>,
}

/// LRU eviction algorithm
/// Returns keys to evict to stay under max_size
pub fn lru_eviction(entries_json: &str, max_size: u64) -> String {
    let entries: CacheEntries = match serde_json::from_str(entries_json) {
        Ok(e) => e,
        Err(_) => return json!({"keys_to_evict": [], "error": "Invalid JSON"}).to_string(),
    };

    // Calculate total size
    let total_size: u64 = entries.entries.iter().map(|e| e.size).sum();

    if total_size <= max_size {
        return json!({"keys_to_evict": []}).to_string();
    }

    // Sort by LRU score: lower access_count and older last_access = higher priority to evict
    let mut sorted_entries = entries.entries.clone();
    sorted_entries.sort_by(|a, b| {
        // Primary: access count (lower = evict first)
        match a.access_count.cmp(&b.access_count) {
            std::cmp::Ordering::Equal => {
                // Secondary: last access time (older = evict first)
                a.last_access.cmp(&b.last_access)
            }
            other => other,
        }
    });

    // Evict entries until we're under max_size
    let mut keys_to_evict: Vec<String> = Vec::new();
    let mut current_size = total_size;

    for entry in sorted_entries {
        if current_size <= max_size {
            break;
        }
        keys_to_evict.push(entry.key.clone());
        current_size -= entry.size;
    }

    json!({
        "keys_to_evict": keys_to_evict,
        "freed_size": total_size - current_size,
        "remaining_size": current_size
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_lru_eviction() {
        let entries_json = json!({
            "entries": [
                {"key": "key1", "size": 1000, "last_access": 1000, "access_count": 5},
                {"key": "key2", "size": 2000, "last_access": 2000, "access_count": 3},
                {"key": "key3", "size": 3000, "last_access": 3000, "access_count": 1},
            ]
        }).to_string();

        // Max size is 4000, total is 6000, need to evict 2000
        let result = lru_eviction(&entries_json, 4000);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        let keys: Vec<String> = parsed["keys_to_evict"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

        // Should evict key3 (lowest access_count) and key2 (next lowest)
        assert!(keys.contains(&"key3".to_string()));
    }
}
