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

    write_to_stdin(&mut child, plaintext, "encrypt")?;

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

    write_to_stdin(&mut child, ciphertext, "decrypt")?;

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

/// Write data to the stdin of a spawned child process, then close the handle.
/// The `op` parameter is used only for error messages (e.g. "encrypt" or "decrypt").
fn write_to_stdin(
    child: &mut std::process::Child,
    data: &[u8],
    op: &str,
) -> Result<()> {
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(data)
            .map_err(|e| VaultKeyError::Crypto(format!("Failed to write to gpg stdin ({}): {}", op, e)))?;
    }
    Ok(())
}
