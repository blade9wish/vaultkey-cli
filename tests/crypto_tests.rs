/// Integration tests for the crypto module.
/// These tests require a working GPG installation with a test key available.
/// They are gated behind the `gpg_integration` feature flag to avoid failures
/// in CI environments without GPG configured.

#[cfg(feature = "gpg_integration")]
mod gpg_integration_tests {
    use vaultkey_cli::crypto::{gpg_decrypt, gpg_encrypt};

    const TEST_RECIPIENT: &str = "test@vaultkey.local";

    #[test]
    fn test_encrypt_produces_armored_output() {
        let plaintext = b"super secret value";
        let result = gpg_encrypt(plaintext, TEST_RECIPIENT);
        assert!(result.is_ok(), "Encryption should succeed: {:?}", result);
        let ciphertext = result.unwrap();
        let armored = String::from_utf8_lossy(&ciphertext);
        assert!(
            armored.contains("BEGIN PGP MESSAGE"),
            "Output should be GPG armored"
        );
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = b"roundtrip test payload";
        let ciphertext = gpg_encrypt(plaintext, TEST_RECIPIENT)
            .expect("Encryption should succeed");
        let decrypted = gpg_decrypt(&ciphertext).expect("Decryption should succeed");
        assert_eq!(decrypted, plaintext, "Decrypted output must match original");
    }

    #[test]
    fn test_encrypt_empty_payload() {
        let result = gpg_encrypt(b"", TEST_RECIPIENT);
        assert!(result.is_ok(), "Encrypting empty payload should succeed");
    }
}

/// Unit-level tests that do NOT require GPG — validate error path behaviour
/// by calling decrypt with clearly invalid input.
#[cfg(test)]
mod unit_tests {
    use vaultkey_cli::crypto::gpg_decrypt;

    #[test]
    fn test_decrypt_invalid_data_returns_error() {
        let garbage = b"this is not valid gpg ciphertext";
        let result = gpg_decrypt(garbage);
        // gpg will exit non-zero; we expect a Crypto error variant.
        assert!(
            result.is_err(),
            "Decrypting garbage data should return an error"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("gpg"),
            "Error message should mention gpg, got: {}",
            err_msg
        );
    }
}
