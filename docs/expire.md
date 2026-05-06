# Secret Expiry Management

The `expire` module provides time-based expiry tracking for secrets stored in vaultkey bundles.

## Overview

Each secret key can be assigned an expiry deadline. The expiry registry tracks these deadlines and provides status queries, warnings for soon-to-expire secrets, and automated purging of expired entries.

## Commands

### Set Expiry

```bash
vaultkey expire set <key> --ttl <seconds> [--notify-before <seconds>]
```

Registers an expiry deadline for the given key. Optionally configure an early-warning threshold.

**Example:**
```bash
vaultkey expire set db_password --ttl 86400 --notify-before 3600
```

### Check Expiry

```bash
vaultkey expire check <key>
```

Reports the current expiry status for a key: `OK`, `SOON`, or `EXPIRED`.

### List All Expiries

```bash
vaultkey expire list
```

Lists all registered expiry entries with their status and deadline.

### Purge Expired

```bash
vaultkey expire purge
```

Removes all expired entries from the registry. Does not delete the underlying secrets — only clears expiry metadata.

## Status Values

| Status  | Meaning                                              |
|---------|------------------------------------------------------|
| OK      | Secret is valid and not close to expiry              |
| SOON    | Secret will expire within the notify-before window   |
| EXPIRED | Secret has passed its expiry deadline                |

## Notes

- Expiry tracking is metadata only. Enforcement (e.g., blocking access) is handled by the `lock` and `ttl` modules.
- Use `vaultkey audit` to log expiry-related events for compliance.
