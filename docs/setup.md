# Setup Guide

## Prerequisites

Before installing `git-workflow`, make sure you have the following tools
installed:

| Tool | Purpose | Install guide |
|------|---------|---------------|
| **Rust** (cargo) | Build the CLI from source | https://rustup.rs |
| **Git** | Version control | https://git-scm.com/downloads |
| **GitHub CLI** (gh) | Create Pull Requests from the terminal | https://cli.github.com |

## Install from source

```bash
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .
```

This places the `git-workflow` binary in `~/.cargo/bin/`, which should already
be in your `PATH` if you installed Rust via `rustup`.

## Install from a release binary

Download the archive for your platform from the
[Releases](https://github.com/nimonht/Simple-Rust-cli-wrapper/releases) page,
extract it, and move the binary to a directory in your `PATH`.

### Linux / macOS

```bash
tar xzf git-workflow-linux-amd64.tar.gz   # or the macOS variant
sudo mv git-workflow /usr/local/bin/
```

### Windows

Extract `git-workflow-windows-amd64.zip` and move `git-workflow.exe` to a
directory listed in your `PATH` environment variable (for example
`C:\Users\<you>\.cargo\bin\`).

## Verify the installation

```bash
git-workflow --version
git-workflow --help
```

## Authenticate GitHub CLI

The `finish` command uses `gh` to open Pull Requests. Authenticate once:

```bash
gh auth login
```

Follow the prompts to log in with your GitHub account.

## Updating

To update to the latest version from source:

```bash
cd Simple-Rust-cli-wrapper
git pull
cargo install --path .
```

Or download the newest release binary from the Releases page.
