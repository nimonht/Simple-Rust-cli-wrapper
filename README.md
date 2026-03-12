# git-workflow

A Rust CLI tool that automates Git workflows.

## Commands

| Command | Description |
|---------|-------------|
| `git-workflow start <branch-name>` | Sync the default branch and create a new feature branch |
| `git-workflow finish <pr-title>` | Stage, commit, push, and open a Pull Request |
| `git-workflow dump [OPTIONS]` | Dump commits from a branch to patch or diff files |
| `git-workflow tui` | Launch interactive TUI mode |

## Quick Start

See [docs/installation-manual.md](docs/installation-manual.md) for full
installation instructions.

```bash
# Start a feature branch
git-workflow start feature/my-feature

# ... make changes ...

# Commit, push, and open a PR
git-workflow finish "Add my feature"

# Dump patches from a branch
git-workflow dump --branch feature/my-feature --format patch --output ./patches

# Send patches via email
git-workflow dump --branch feature/my-feature --format patch --email maintainer@example.com

# Launch the interactive TUI
git-workflow tui
```

## Prerequisites

- [Rust](https://rustup.rs) (cargo) -- only needed when building from source
- [Git](https://git-scm.com/downloads)
- [GitHub CLI](https://cli.github.com) (`gh`) -- installed and authenticated

## Documentation

- [Installation manual](docs/installation-manual.md) -- prerequisites,
  installation, and authentication
- [Platform setup tutorial](docs/platform-setup.md) -- step-by-step for Linux,
  macOS, Windows, and BSD
- [Use case examples](docs/use-cases.md) -- common workflows illustrated
- [Code rules](.github/code-rules.md) -- coding conventions for contributors
- [Contributing guide](CONTRIBUTING.md) -- how to contribute to the project
- [Code of Conduct](CODE_OF_CONDUCT.md) -- community standards and expectations

## Development

```bash
cargo build          # compile
cargo test           # run tests
cargo clippy         # lint
cargo fmt --all      # format
```

## CI/CD

- **CI** (`.github/workflows/ci.yml`) -- runs check, test, clippy, and fmt on
  every push and pull request. Also builds release binaries for Linux, macOS,
  and Windows.
- **Release** (`.github/workflows/release.yml`) -- when a tag matching `v*` is
  pushed, builds release archives for all platforms and creates a GitHub Release
  with the artifacts attached.

To cut a release, use the version bump script to update all version strings
across the project in one step:

```bash
./scripts/set-version.sh 0.3.0
cargo test && cargo clippy -- -D warnings
git add -A && git commit -m "Bump version to 0.3.0"
git tag v0.3.0
git push origin v0.3.0
```

Never edit version strings by hand -- the script updates `Cargo.toml`,
`Cargo.lock`, all packaging files, and documentation automatically.

## License

BSD 2-Clause. See [LICENSE](LICENSE).