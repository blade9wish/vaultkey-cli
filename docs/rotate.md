# Key Rotation

The `rotate` command re-encrypts all secrets in a vault bundle from one GPG recipient to another. This is useful when rotating team members, revoking access, or updating key pairs.

## Usage

```
vaultkey rotate \
  --vault path/to/secrets.vault \
  --old-recipient old@example.com \
  --new-recipient new@example.com \
  [--dry-run]
```

## Options

| Flag | Description |
|------|-------------|
| `--vault` | Path to the vault bundle file |
| `--old-recipient` | GPG key ID or email of the current recipient |
| `--new-recipient` | GPG key ID or email of the new recipient |
| `--dry-run` | Preview rotation without writing changes |
| `--audit-path` | Custom path for the audit log (default: `audit.log`) |

## How It Works

1. Loads the vault from disk.
2. For each secret, decrypts the ciphertext using the old recipient's key.
3. Re-encrypts the plaintext using the new recipient's key.
4. Saves the updated vault back to disk.
5. Records each rotation event in the audit log.

## Dry Run

Using `--dry-run` will report how many secrets would be rotated without making any changes. This is recommended before performing a live rotation.

## Audit Trail

All rotation events are recorded in the audit log with timestamps and secret key names. The completion event includes a summary count.

## Security Notes

- Ensure the new recipient's public key is available in your GPG keyring before rotating.
- The old recipient's private key must be accessible to decrypt existing secrets.
- After rotation, the old recipient will no longer be able to decrypt the secrets.
