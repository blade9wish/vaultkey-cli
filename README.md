# vaultkey-cli

> A lightweight CLI for managing encrypted secret bundles with TOML-based config and GPG backend support.

---

## Installation

**From source (requires Rust toolchain):**

```bash
cargo install --path .
```

Or via crates.io:

```bash
cargo install vaultkey-cli
```

---

## Usage

Initialize a new secret bundle:

```bash
vaultkey init --config vault.toml
```

Add and encrypt a secret:

```bash
vaultkey set API_KEY "supersecret" --recipient you@example.com
```

Retrieve a decrypted secret:

```bash
vaultkey get API_KEY
```

List all keys in the current bundle:

```bash
vaultkey list
```

Export the encrypted bundle to a file:

```bash
vaultkey export --out secrets.bundle.gpg
```

---

## Configuration

`vaultkey-cli` uses a TOML file (`vault.toml` by default) to define bundle metadata, GPG recipient keys, and storage paths. A minimal example:

```toml
[vault]
name = "my-project"
recipient = "you@example.com"
store = "./secrets"
```

---

## Requirements

- Rust 1.70+
- GPG (`gpg2`) installed and configured on your system

---

## License

This project is licensed under the [MIT License](LICENSE).