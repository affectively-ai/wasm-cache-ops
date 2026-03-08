use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::prelude::*;

/// Compress data using gzip
/// Returns compressed bytes, or an empty Vec if compression fails
pub fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if encoder.write_all(data.as_bytes()).is_err() {
        return Vec::new();
    }
    match encoder.finish() {
        Ok(compressed) => compressed,
        Err(_) => Vec::new(),
    }
}

/// Decompress data from gzip
/// Returns decompressed string, or an empty string if decompression fails
pub fn decompress_data(compressed: &[u8]) -> String {
    if compressed.is_empty() {
        return String::new();
    }
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = String::new();
    match decoder.read_to_string(&mut decompressed) {
        Ok(_) => decompressed,
        Err(_) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let original = r#"{"name":"John","age":30,"data":"This is a test string with some content"}"#;
        let compressed = compress_data(original);
        let decompressed = decompress_data(&compressed);
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_reduces_size() {
        let large_data = "x".repeat(1000);
        let compressed = compress_data(&large_data);
        // Compressed should be smaller for repetitive data
        assert!(compressed.len() < large_data.len());
    }

    #[test]
    fn test_compress_empty_string() {
        let compressed = compress_data("");
        assert!(!compressed.is_empty()); // gzip header is still produced
        let decompressed = decompress_data(&compressed);
        assert_eq!(decompressed, "");
    }

    #[test]
    fn test_decompress_empty_input() {
        let result = decompress_data(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_decompress_invalid_data() {
        let result = decompress_data(&[0, 1, 2, 3]);
        assert_eq!(result, "");
    }
}
