# Secret Search

The `search` command allows you to find secrets within a vault bundle by searching key names and/or values.

## Usage

```bash
vaultkey search <vault-path> <term> [OPTIONS]
```

## Options

| Flag | Description |
|------|-------------|
| `--case-sensitive` | Enable case-sensitive matching (default: false) |
| `--keys-only` | Only search key names, skip values |

## Examples

### Search by partial key name
```bash
vaultkey search ./my_vault.toml db
```
Output:
```
Found 2 result(s) for 'db':
----------------------------------------
  db_host = localhost [matched in: key]
  db_password = secret123 [matched in: key]
```

### Search values case-insensitively
```bash
vaultkey search ./my_vault.toml admin
```

### Keys-only search
```bash
vaultkey search ./my_vault.toml api --keys-only
```

## Notes

- Results are sorted alphabetically by key name.
- When `--keys-only` is set, values are not displayed in output.
- Matching is substring-based; regex is not supported.
