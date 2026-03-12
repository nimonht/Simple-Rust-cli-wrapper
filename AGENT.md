# Agent Guide

This document describes the project for AI coding agents.

## Project

`git-workflow` is a Rust CLI tool that wraps common Git operations. It provides
two subcommands:

- `start <branch-name>` -- sync the default branch and create a new feature branch.
- `finish <pr-title>` -- stage all changes, commit, push, and open a Pull Request via GitHub CLI (`gh`).

## Build and Test

```bash
cargo build            # compile
cargo test             # run unit tests
cargo clippy           # lint
cargo fmt --all        # format
```

## Repository Layout

```
Cargo.toml          -- crate manifest
src/main.rs         -- CLI entry point, command implementations, and tests
docs/               -- additional documentation (setup, use cases)
.github/workflows/  -- CI and release pipelines
.github/code-rules.md -- coding conventions
```

## Code Conventions

- No emoji in source code. Use plain text markers like `[OK]`, `[ERROR]`.
- Edition 2021.
- `anyhow` for errors, `clap` derive for CLI parsing, `colored` for terminal output.
- See `.github/code-rules.md` for the full list.

## External Dependencies

The CLI shells out to `git` and `gh` (GitHub CLI). Both must be installed and
configured on the user's machine.
