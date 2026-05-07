use vaultkey_cli::compress::{
    compress, compress_string, compression_ratio, decompress, decompress_string,
};

#[test]
fn test_compress_decompress_roundtrip() {
    let original = b"hello, vaultkey! this is a test payload for compression.";
    let compressed = compress(original).expect("compress failed");
    let restored = decompress(&compressed).expect("decompress failed");
    assert_eq!(original.to_vec(), restored);
}

#[test]
fn test_compress_reduces_size_for_repetitive_data() {
    let original = "aaaa".repeat(200);
    let compressed = compress(original.as_bytes()).expect("compress failed");
    assert!(
        compressed.len() < original.len(),
        "expected compressed size to be smaller for repetitive data"
    );
}

#[test]
fn test_decompress_invalid_data_returns_error() {
    let garbage = b"this is not gzip data at all!!!";
    let result = decompress(garbage);
    assert!(result.is_err(), "expected error on invalid gzip data");
}

#[test]
fn test_compression_ratio_empty_input() {
    let ratio = compression_ratio(b"", b"anything");
    assert!(ratio.is_none());
}

#[test]
fn test_compression_ratio_non_empty() {
    let original = b"hello world hello world hello world";
    let compressed = compress(original).expect("compress failed");
    let ratio = compression_ratio(original, &compressed);
    assert!(ratio.is_some());
    let r = ratio.unwrap();
    assert!(r > 0.0, "ratio should be positive");
}

#[test]
fn test_compress_string_roundtrip() {
    let input = "secret_value=super_secret_123!";
    let encoded = compress_string(input).expect("compress_string failed");
    let decoded = decompress_string(&encoded).expect("decompress_string failed");
    assert_eq!(input, decoded);
}

#[test]
fn test_decompress_string_invalid_base64() {
    let result = decompress_string("!!!not-valid-base64!!!");
    assert!(result.is_err());
}

#[test]
fn test_compress_empty_bytes() {
    let compressed = compress(b"").expect("compress of empty should succeed");
    let restored = decompress(&compressed).expect("decompress of empty compressed should succeed");
    assert_eq!(restored, b"");
}
