use crate::error::VaultError;
use std::io::{Read, Write};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

/// Compress bytes using gzip
pub fn compress(data: &[u8]) -> Result<Vec<u8>, VaultError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|e| VaultError::Io(e))?;
    encoder
        .finish()
        .map_err(|e| VaultError::Io(e))
}

/// Decompress gzip-compressed bytes
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, VaultError> {
    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    decoder
        .read_to_end(&mut out)
        .map_err(|e| VaultError::Io(e))?;
    Ok(out)
}

/// Returns the compression ratio as a float (compressed / original)
/// Returns None if input is empty
pub fn compression_ratio(original: &[u8], compressed: &[u8]) -> Option<f64> {
    if original.is_empty() {
        return None;
    }
    Some(compressed.len() as f64 / original.len() as f64)
}

/// Compress a UTF-8 string and return base64-encoded result
pub fn compress_string(input: &str) -> Result<String, VaultError> {
    let compressed = compress(input.as_bytes())?;
    Ok(base64::encode(&compressed))
}

/// Decompress a base64-encoded gzip-compressed string back to UTF-8
pub fn decompress_string(input: &str) -> Result<String, VaultError> {
    let compressed = base64::decode(input)
        .map_err(|e| VaultError::Generic(format!("base64 decode error: {}", e)))?;
    let raw = decompress(&compressed)?;
    String::from_utf8(raw)
        .map_err(|e| VaultError::Generic(format!("utf8 decode error: {}", e)))
}
