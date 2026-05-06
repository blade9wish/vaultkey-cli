# Notification Rules

VaultKey supports configurable event notifications to alert on key vault operations.

## Supported Events

| Event     | Trigger                          |
|-----------|----------------------------------|
| `access`  | A secret was accessed            |
| `rotate`  | A secret was rotated             |
| `expire`  | A secret has expired             |
| `unlock`  | A bundle was unlocked            |
| `policy`  | A policy violation occurred      |
| `quota`   | A quota limit was exceeded       |
| `*`       | Wildcard — matches all events    |

## Channels

- **stdout** — Print to standard output
- **file** — Append to a log file
- **webhook** — POST to a URL (future support)

## Commands

### Add a rule

```bash
vaultkey notify add --event rotate --channel file --target /var/log/vaultkey.log
vaultkey notify add --event access --channel stdout
vaultkey notify add --event "*" --channel webhook --target https://hooks.example.com/vault
```

### Remove rules for an event

```bash
vaultkey notify remove --event rotate
```

### List all rules

```bash
vaultkey notify list
```

### Test a notification

```bash
vaultkey notify test --event access --key my_secret
```

## Storage

Rules are stored in `.vaultkey/notify.toml` as TOML.

## Example `notify.toml`

```toml
[[rules]]
event = "rotate"
channel = { File = "/var/log/vaultkey-rotate.log" }

[[rules]]
event = "*"
channel = "Stdout"
```
