# Development

## Rust toolchain

This project tracks stable Rust via `rust-toolchain.toml`.

Policy:

- Use stable Rust.
- Keep `rustfmt` and `clippy` installed.
- `Cargo.toml` currently uses edition 2024, so supported Rust must understand Rust 2024.
- CI is source of truth for accepted toolchain behavior.

Before commit:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
