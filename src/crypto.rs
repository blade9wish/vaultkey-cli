use std::process::{Command, Stdio};
use std::io::Write;

use crate::error::{VaultKeyError, Result};

/// Encrypt plaintext data using GPG with the given recipient key ID or email.
pub fn gpg_encrypt(plaintext: &[u8], recipient: &str) -> Result<Vec<u8>> {
    let mut child = Command::new("gpg")
        .args([
            "--batch",
            "--yes",
            "--trust-model",
            "always",
            "--encrypt",
            "--armor",
            "-r",
            recipient,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| VaultKeyError::Crypto(format!("Failed to spawn gpg: {}", e)))?;

    if let Some(stdin) = child.stdin.take() {
        let mut stdin = stdin;
        stdin
            .write_all(plaintext)
            .map_err(|e| VaultKeyError::Crypto(format!("Failed to write to gpg stdin: {}", e)))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| VaultKeyError::Crypto(format!("gpg encrypt failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VaultKeyError::Crypto(format!(
            "gpg encrypt error: {}",
            stderr.trim()
        )));
    }

    Ok(output.stdout)
}

/// Decrypt GPG-armored ciphertext, returning the plaintext bytes.
pub fn gpg_decrypt(ciphertext: &[u8]) -> Result<Vec<u8>> {
    let mut child = Command::new("gpg")
        .args(["--batch", "--yes", "--decrypt"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| VaultKeyError::Crypto(format!("Failed to spawn gpg: {}", e)))?;

    if let Some(stdin) = child.stdin.take() {
        let mut stdin = stdin;
        stdin
            .write_all(ciphertext)
            .map_err(|e| VaultKeyError::Crypto(format!("Failed to write to gpg stdin: {}", e)))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| VaultKeyError::Crypto(format!("gpg decrypt failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(VaultKeyError::Crypto(format!(
            "gpg decrypt error: {}",
            stderr.trim()
        )));
    }

    Ok(output.stdout)
}
