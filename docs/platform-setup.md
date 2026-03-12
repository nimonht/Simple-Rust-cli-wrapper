# Platform Setup Tutorial

Step-by-step instructions for installing `git-workflow` on Linux, macOS,
Windows, and BSD.

---

## Linux

### From source

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2. Install Git (Debian/Ubuntu)
sudo apt update && sudo apt install -y git

# 3. Install GitHub CLI
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

# 4. Build and install git-workflow
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .

# 5. Authenticate gh
gh auth login
```

### From release binary

```bash
# Download the latest release
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-linux-amd64.tar.gz
tar xzf git-workflow-linux-amd64.tar.gz
sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

---

## macOS

### From source

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2. Install Git and GitHub CLI via Homebrew
brew install git gh

# 3. Build and install
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .

# 4. Authenticate gh
gh auth login
```

### From release binary

```bash
# Intel Mac
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-macos-amd64.tar.gz
tar xzf git-workflow-macos-amd64.tar.gz

# Apple Silicon
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-macos-arm64.tar.gz
tar xzf git-workflow-macos-arm64.tar.gz

sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

---

## Windows

### From source

```powershell
# 1. Install Rust
#    Download and run the installer from https://rustup.rs
#    Or use winget:
winget install Rustlang.Rustup

# 2. Install Git
winget install Git.Git

# 3. Install GitHub CLI
winget install GitHub.cli

# 4. Build and install (from a terminal with cargo in PATH)
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .

# 5. Authenticate gh
gh auth login
```

### From release binary

1. Download `git-workflow-windows-amd64.zip` from the
   [Releases](https://github.com/nimonht/Simple-Rust-cli-wrapper/releases) page.
2. Extract the ZIP file.
3. Move `git-workflow.exe` to a directory in your `PATH` (for example
   `C:\Users\<you>\.cargo\bin\`).
4. Open a new terminal and run `git-workflow --version`.

---

## BSD (FreeBSD / OpenBSD)

### From source

```sh
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2. Install Git
# FreeBSD:
sudo pkg install git
# OpenBSD:
doas pkg_add git

# 3. Install GitHub CLI
# FreeBSD:
sudo pkg install gh
# OpenBSD: build from source or use a port
#   See https://github.com/cli/cli#installation

# 4. Build and install
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .

# 5. Authenticate gh
gh auth login
```

> Note: Pre-built release binaries are provided for Linux, macOS, and Windows.
> BSD users should build from source.

---

## Verify

On any platform, confirm the installation:

```bash
git-workflow --version
git-workflow --help
```
