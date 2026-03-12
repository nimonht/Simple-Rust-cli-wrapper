# Agent Guide

This document describes the project for AI coding agents. See [README.md](README.md)
for user-facing documentation, commands, and quick start instructions.

## Project

`git-workflow` is a Rust CLI tool that automates Git workflows. It provides
four subcommands:

- `start <branch-name>` -- sync the default branch and create a new feature branch.
- `finish <pr-title>` -- stage all changes, commit, push, and open a Pull Request via GitHub CLI (`gh`).
- `dump [OPTIONS]` -- dump commits from a branch to patch or diff files (for kernel dev workflows). Options: `--branch`, `--commit`, `--all`, `--format` (patch/diff), `--output`, `--email`.
- `tui` -- launch interactive TUI mode (lazydocker-style, respects terminal theme).

## Build and Test

```bash
cargo build            # compile
cargo test             # run unit tests
cargo clippy           # lint
cargo fmt --all        # format
```

## Repository Layout

```
Cargo.toml             -- crate manifest
src/main.rs            -- CLI entry point and top-level wiring
src/git.rs             -- Git helper functions (shell-outs to git/gh)
src/commands/*.rs       -- subcommand implementations (start, finish, dump)
src/tui/*.rs            -- TUI module (interactive terminal UI)
docs/                  -- additional documentation (installation, use cases)
.github/workflows/     -- CI and release pipelines
.github/code-rules.md  -- coding conventions
```

## Code Conventions

- No emoji in source code. Use plain text markers like `[OK]`, `[ERROR]`.
- Edition 2021.
- See `.github/code-rules.md` for the full list.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.5 | CLI argument parsing (derive) |
| anyhow | 1.0 | Error handling |
| colored | 3.1 | Terminal color output |
| ratatui | 0.28 | Terminal UI framework |
| crossterm | 0.28 | Cross-platform terminal manipulation |

## External Dependencies

The CLI shells out to `git` and `gh` (GitHub CLI). Both must be installed and
configured on the user's machine.
