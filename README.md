# git-workflow

A Rust CLI tool that automates common Git workflows for teams.

## Commands

| Command | Description |
|---------|-------------|
| `git-workflow start <branch-name>` | Sync the default branch and create a new feature branch |
| `git-workflow finish <pr-title>` | Stage, commit, push, and open a Pull Request |

## Quick Start

```bash
# Install from source
cargo install --path .

# Start a feature branch
git-workflow start feature/my-feature

# ... make changes ...

# Commit, push, and open a PR
git-workflow finish "Add my feature"
```

## Prerequisites

- [Rust](https://rustup.rs) (cargo)
- [Git](https://git-scm.com/downloads)
- [GitHub CLI](https://cli.github.com) (`gh`) -- installed and authenticated

## Installation

### From source

```bash
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .
```

### From release binaries

Download a pre-built binary from the
[Releases](https://github.com/nimonht/Simple-Rust-cli-wrapper/releases) page
and place it somewhere in your `PATH`.

See [docs/platform-setup.md](docs/platform-setup.md) for detailed instructions
for Linux, macOS, Windows, and BSD.

## Documentation

- [Setup guide](docs/setup.md) -- prerequisites, installation, and
  authentication
- [Platform setup tutorial](docs/platform-setup.md) -- step-by-step for Linux,
  macOS, Windows, and BSD
- [Use case examples](docs/use-cases.md) -- common workflows illustrated
- [Code rules](.github/code-rules.md) -- coding conventions for contributors

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

To cut a release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## License

BSD 2-Clause. See [LICENSE](LICENSE).