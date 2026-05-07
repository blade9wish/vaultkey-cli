# Alias Management

The `alias` module provides a way to define short, memorable names that map to
longer vault bundle paths or key identifiers.

## Commands

### Add an alias

```bash
vaultkey alias add <alias> <target>
```

Creates a new alias pointing to the given target path. Fails if the alias
already exists.

### Remove an alias

```bash
vaultkey alias remove <alias>
```

Deletes an existing alias. Fails if the alias is not found.

### List all aliases

```bash
vaultkey alias list
```

Prints all defined aliases in alphabetical order.

### Resolve an alias

```bash
vaultkey alias resolve <alias>
```

Prints the target path for the given alias. Useful in scripts.

### Rename an alias

```bash
vaultkey alias rename <old> <new>
```

Renames an existing alias without changing its target. Fails if the old alias
does not exist or the new name is already taken.

## Storage

Aliases are persisted in `aliases.toml` inside the vault directory:

```toml
[entries]
prod = "production/secrets"
dev = "dev/secrets"
```

## Notes

- Alias names and targets must be non-empty strings.
- Aliases are resolved locally and do not affect the underlying bundle paths.
