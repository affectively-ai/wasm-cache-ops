use wasm_bindgen::prelude::*;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod hashing;
mod serialization;
mod compression;
mod batch_ops;
mod eviction;

use hashing::hash_query;
use serialization::{serialize_document, deserialize_document};
use compression::{compress_data, decompress_data};
use batch_ops::batch_operations;
use eviction::lru_eviction;

/// Hash a query JSON string for fingerprinting
/// 
/// # Arguments
/// * `query_json` - JSON string representing the query
/// 
/// # Returns
/// Hex-encoded hash string
#[wasm_bindgen]
pub fn hash_query_wasm(query_json: &str) -> String {
    hash_query(query_json)
}

/// Serialize a document JSON string (optimized)
/// 
/// # Arguments
/// * `doc_json` - JSON string representing the document
/// 
/// # Returns
/// Serialized JSON string
#[wasm_bindgen]
pub fn serialize_document_wasm(doc_json: &str) -> String {
    serialize_document(doc_json)
}

/// Deserialize a document JSON string (optimized)
/// 
/// # Arguments
/// * `serialized` - Serialized JSON string
/// 
/// # Returns
/// Deserialized JSON string
#[wasm_bindgen]
pub fn deserialize_document_wasm(serialized: &str) -> String {
    deserialize_document(serialized)
}

/// Compress data using gzip
/// 
/// # Arguments
/// * `data` - String data to compress
/// 
/// # Returns
/// Base64-encoded compressed data
#[wasm_bindgen]
pub fn compress_data_wasm(data: &str) -> Vec<u8> {
    compress_data(data)
}

/// Decompress data from gzip
/// 
/// # Arguments
/// * `compressed` - Compressed byte array
/// 
/// # Returns
/// Decompressed string
#[wasm_bindgen]
pub fn decompress_data_wasm(compressed: &[u8]) -> String {
    decompress_data(compressed)
}

/// Optimize batch operations
/// 
/// # Arguments
/// * `ops_json` - JSON string with batch operations
/// 
/// # Returns
/// Optimized operations JSON string
#[wasm_bindgen]
pub fn batch_operations_wasm(ops_json: &str) -> String {
    batch_operations(ops_json)
}

/// LRU eviction algorithm
/// 
/// # Arguments
/// * `entries_json` - JSON string with cache entries
/// * `max_size` - Maximum cache size in bytes
/// 
/// # Returns
/// JSON string with entries to evict
#[wasm_bindgen]
pub fn lru_eviction_wasm(entries_json: &str, max_size: u64) -> String {
    lru_eviction(entries_json, max_size)
}
