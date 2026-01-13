use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::prelude::*;

/// Compress data using gzip
/// Returns compressed bytes
pub fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap_or_else(|_| ());
    encoder.finish().unwrap_or_else(|_| Vec::new())
}

/// Decompress data from gzip
/// Returns decompressed string
pub fn decompress_data(compressed: &[u8]) -> String {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap_or(0);
    decompressed
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
}
