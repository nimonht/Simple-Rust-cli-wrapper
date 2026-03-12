# Contributing to git-workflow

Welcome! Thank you for considering a contribution to **git-workflow**. Whether
you are fixing a bug, adding a feature, improving documentation, or reporting
an issue, your help is appreciated.

This guide explains how to get started, what conventions to follow, and how to
submit your changes.

## Table of Contents

- [Getting Started](#getting-started)
- [Repository Layout](#repository-layout)
- [Code Style](#code-style)
- [How to Submit Changes](#how-to-submit-changes)
- [Commit Messages](#commit-messages)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Reporting Bugs](#reporting-bugs)
- [Requesting Features](#requesting-features)
- [CI/CD](#cicd)
- [Code of Conduct](#code-of-conduct)
- [License](#license)

## Getting Started

### Prerequisites

Make sure the following tools are installed and available on your system:

- **Rust** (stable toolchain) -- install via [rustup](https://rustup.rs)
- **Git** -- [https://git-scm.com/downloads](https://git-scm.com/downloads)
- **GitHub CLI** (`gh`) -- [https://cli.github.com](https://cli.github.com),
  installed and authenticated

### Clone and Build

1. Fork the repository on GitHub.
2. Clone your fork locally:

```
git clone https://github.com/<your-username>/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
```

3. Build and run the test suite to confirm everything works:

```
cargo build          # compile
cargo test           # run unit tests
cargo clippy         # lint (must pass with -D warnings)
cargo fmt --all      # format
```

All four commands must succeed before you submit changes.

## Repository Layout

```
Cargo.toml             -- crate manifest
src/main.rs            -- CLI entry point and top-level wiring
src/git.rs             -- Git helper functions (shell-outs to git/gh)
src/commands/*.rs      -- subcommand implementations (start, finish, dump)
src/tui/*.rs           -- TUI module (interactive terminal UI)
docs/                  -- additional documentation
packaging/aur/         -- Arch Linux AUR PKGBUILD
packaging/gentoo/      -- Gentoo ebuild and metadata
packaging/nix/         -- Nix flake and default.nix derivation
scripts/set-version.sh -- version bump helper (updates all files)
.github/workflows/     -- CI and release pipelines
.github/code-rules.md  -- coding conventions
```

## Code Style

The full set of coding conventions is documented in
[.github/code-rules.md](.github/code-rules.md). Below are the key rules:

- **No emoji in source code.** Use plain text markers such as `[OK]`,
  `[ERROR]`, or `[WARN]` instead.
- **Edition 2021.** The `Cargo.toml` must specify `edition = "2021"`.
- **Error handling.** Use the `anyhow` crate for error handling in application
  code.
- **CLI parsing.** Use `clap` with derive macros for argument parsing.
- **Linting.** Code must pass `cargo clippy -- -D warnings` with zero
  warnings.
- **Formatting.** Code must pass `cargo fmt --all -- --check`. Run
  `cargo fmt --all` to auto-format before committing.
- **Simplicity.** Keep code simple and readable.

## How to Submit Changes

1. **Fork** the repository and create a new branch from `main`:

```
git checkout -b feature/my-change
```

2. **Make your changes.** Follow the code style guidelines above.

3. **Run all checks locally** before pushing:

```
cargo fmt --all
cargo clippy -- -D warnings
cargo test
cargo build
```

4. **Push** your branch to your fork:

```
git push origin feature/my-change
```

5. **Open a Pull Request** against the `main` branch of the upstream
   repository. Fill in the PR description with a clear explanation of what
   your change does and why.

## Commit Messages

- Write clear, concise commit messages.
- Use the imperative mood in the subject line (e.g., "Add dump format option"
  rather than "Added dump format option").
- Keep the subject line under 72 characters.
- If more detail is needed, add a blank line after the subject and write a
  longer description in the body.

Good examples:

```
Fix branch detection when remote HEAD is unset

The previous logic assumed that `git remote show origin` would always
print a HEAD line. Handle the case where it is missing by falling back
to "main".
```

```
Add --email flag to dump subcommand
```

## Pull Request Guidelines

- **Focus on a single change.** Do not bundle unrelated fixes or features into
  one PR.
- **Provide a clear description.** Explain what the PR does, why it is needed,
  and how it was tested.
- **Add or update tests** when your change affects behavior.
- **Ensure CI passes.** All automated checks must be green before a PR will be
  reviewed.
- **Respond to review feedback** promptly and respectfully.

## Reporting Bugs

If you find a bug, please open an issue on the
[GitHub issue tracker](https://github.com/nimonht/Simple-Rust-cli-wrapper/issues).
If a bug report template is available, please use it. Include:

- A clear and descriptive title.
- Steps to reproduce the problem.
- Expected behavior vs. actual behavior.
- Your environment (OS, Rust version, git version, gh version).
- Any relevant log output or error messages.

## Requesting Features

Feature requests are welcome. Please open an issue on the
[GitHub issue tracker](https://github.com/nimonht/Simple-Rust-cli-wrapper/issues)
and, if a feature request template is available, use it. Describe:

- The problem or use case your feature would address.
- How you envision the feature working.
- Any alternatives you have considered.

## CI/CD

The project uses GitHub Actions for continuous integration:

- **CI** (`.github/workflows/ci.yml`) runs automatically on every push and
  pull request. It executes the following checks:
  - `cargo check` -- verify the project compiles
  - `cargo test` -- run the test suite
  - `cargo clippy -- -D warnings` -- lint with warnings treated as errors
  - `cargo fmt --all -- --check` -- verify code formatting

  All checks must pass before a PR can be merged.

- **Release** (`.github/workflows/release.yml`) triggers when a tag matching
  `v*` is pushed. It builds release binaries for Linux, macOS, and Windows,
  then creates a GitHub Release with the artifacts attached.

You do not need to do anything special to trigger CI -- it runs automatically
when you open or update a pull request.

## Code of Conduct

This project has a [Code of Conduct](CODE_OF_CONDUCT.md). By participating,
you agree to abide by its terms. Please read it before contributing.

## License

By contributing to git-workflow, you agree that your contributions will be
licensed under the **BSD 2-Clause License**, the same license that covers the
project. See [LICENSE](LICENSE) for details.