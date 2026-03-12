# Platform Setup Tutorial

Platform-specific instructions for installing `git-workflow` on Linux, macOS,
Windows, and BSD. For general installation steps (building from source, release
binaries, authentication), see [installation-manual.md](installation-manual.md).

This guide focuses on installing the platform-specific prerequisites (Rust, Git,
GitHub CLI) and downloading the correct release binary for your system.

---

## Linux

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git (Debian/Ubuntu)
sudo apt update && sudo apt install -y git

# Install GitHub CLI
# Debian/Ubuntu:
(type -p wget >/dev/null || (sudo apt update && sudo apt-get install wget -y)) \
  && sudo mkdir -p -m 755 /etc/apt/keyrings \
  && out=$(mktemp) && wget -nv -O$out https://cli.github.com/packages/githubcli-archive-keyring.gpg \
  && cat $out | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
  && sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
  && sudo apt update \
  && sudo apt install gh -y
# Fedora:
sudo dnf install gh
```

### Release binary

```bash
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-linux-amd64.tar.gz
tar xzf git-workflow-linux-amd64.tar.gz
sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

---

## macOS

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git and GitHub CLI via Homebrew
brew install git gh
```

### Release binary (Intel)

```bash
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-macos-amd64.tar.gz
tar xzf git-workflow-macos-amd64.tar.gz
sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

### Release binary (Apple Silicon)

```bash
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-macos-arm64.tar.gz
tar xzf git-workflow-macos-arm64.tar.gz
sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

---

## Windows

### Prerequisites

```powershell
# Install Rust (download from https://rustup.rs or use winget)
winget install Rustlang.Rustup

# Install Git
winget install Git.Git

# Install GitHub CLI
winget install GitHub.cli
```

### Release binary

1. Download `git-workflow-windows-amd64.zip` from the
   [Releases](https://github.com/nimonht/Simple-Rust-cli-wrapper/releases) page.
2. Extract the ZIP file.
3. Move `git-workflow.exe` to a directory in your `PATH` (for example
   `C:\Users\<you>\.cargo\bin\`).
4. Open a new terminal and run `git-workflow --version`.

---

## BSD (FreeBSD / OpenBSD)

### Prerequisites

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git
# FreeBSD:
sudo pkg install git
# OpenBSD:
doas pkg_add git

# Install GitHub CLI
# FreeBSD:
sudo pkg install gh
# OpenBSD: build from source or use a port
#   See https://github.com/cli/cli#installation
```

> Note: Pre-built release binaries are provided for Linux, macOS, and Windows.
> BSD users should build from source using the instructions in
> [installation-manual.md](installation-manual.md).

---

## Verify

On any platform, confirm the installation:

```bash
git-workflow --version
git-workflow --help
```
