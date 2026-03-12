# git-workflow

A Rust CLI tool that automates Git workflows.

## Commands

| Command | Description |
|---------|-------------|
| `git-workflow start <branch-name>` | Sync the default branch and create a new feature branch |
| `git-workflow finish <pr-title>` | Stage, commit, push, and open a Pull Request |
| `git-workflow dump [OPTIONS]` | Dump commits from a branch to patch or diff files |
| `git-workflow tui` | Launch interactive TUI mode |


> [!WARNING]
> **EXPERIMENTAL SOFTWARE**
> This scheduler is experimental and intended for use with no guarantees. It may contain bugs and is not recommended for production use. Use at your own risk.

> [!NOTE]
> **AI TRANSPARENCY**
> Large Language Models were used for optimization pattern matching and design exploration. All implementation details have been human-verified and tested. See [AGENT.md](AGENT.md) for more information.

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

## License

BSD 2-Clause. See [LICENSE](LICENSE).
