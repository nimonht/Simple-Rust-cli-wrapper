# Code Rules

Rules and conventions for this project.

## General

- No emoji characters in source code. Use plain text markers such as `[OK]`, `[ERROR]`, or `[WARN]` instead.
- Keep code simple and readable.

## Rust

- Use `edition = "2021"` in `Cargo.toml`.
- Run `cargo clippy -- -D warnings` before submitting changes.
- Run `cargo fmt --all -- --check` to ensure consistent formatting.
- Use `anyhow` for error handling in application code.
- Use `clap` with derive macros for CLI argument parsing.

## Git

- Write clear, concise commit messages.
- Keep pull requests focused on a single change.
