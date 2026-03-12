# Platform Setup Tutorial

Platform-specific instructions for installing `git-workflow` on Linux, macOS,
Windows, and BSD. For general installation steps (building from source, release
binaries, authentication), see [installation-manual.md](installation-manual.md).

This guide focuses on installing the platform-specific prerequisites (Rust, Git,
GitHub CLI) and downloading the correct release binary for your system.

---

## Linux

### Install from source (all distros)

If you have Rust installed, you can build and install on any distribution:

```bash
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper
cargo install --path .
```

### Release binary (all distros)

```bash
curl -LO https://github.com/nimonht/Simple-Rust-cli-wrapper/releases/latest/download/git-workflow-linux-amd64.tar.gz
tar xzf git-workflow-linux-amd64.tar.gz
sudo mv git-workflow /usr/local/bin/
git-workflow --version
```

### Package manager installation

Community-maintained packaging files are provided in the `packaging/` directory
for several distributions. These are not yet published to official repositories
-- see each section below for instructions on building and installing locally.

---

### Debian / Ubuntu

#### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git
sudo apt update && sudo apt install -y git

# Install GitHub CLI
(type -p wget >/dev/null || (sudo apt update && sudo apt-get install wget -y)) \
  && sudo mkdir -p -m 755 /etc/apt/keyrings \
  && out=$(mktemp) && wget -nv -O$out https://cli.github.com/packages/githubcli-archive-keyring.gpg \
  && cat $out | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
  && sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
  && sudo apt update \
  && sudo apt install gh -y
```

#### Install git-workflow

Build from source or use the release binary (see above).

---

### Fedora / RHEL / CentOS Stream

#### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git
sudo dnf install -y git

# Install GitHub CLI
sudo dnf install -y gh
```

#### Install git-workflow

Build from source or use the release binary (see above).

---

### Arch Linux / Manjaro

#### Prerequisites

```bash
# Install Rust
sudo pacman -S --noconfirm rust

# Install Git
sudo pacman -S --noconfirm git

# Install GitHub CLI
sudo pacman -S --noconfirm github-cli
```

#### Install git-workflow (AUR)

A PKGBUILD is provided in `packaging/aur/`. You can install it with an AUR
helper such as `yay` or `paru`, or build manually:

```bash
# Option 1: Using yay (if published to AUR)
yay -S git-workflow

# Option 2: Using paru (if published to AUR)
paru -S git-workflow

# Option 3: Build from the PKGBUILD manually
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper/packaging/aur
makepkg -si
```

---

### openSUSE

#### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Git
sudo zypper install -y git

# Install GitHub CLI
sudo zypper install -y gh
```

#### Install git-workflow

Build from source or use the release binary (see above).

---

### Void Linux

#### Prerequisites

```bash
# Install Rust
sudo xbps-install -Sy rust cargo

# Install Git
sudo xbps-install -Sy git

# Install GitHub CLI
sudo xbps-install -Sy github-cli
```

#### Install git-workflow

Build from source or use the release binary (see above).

---

### Alpine Linux

#### Prerequisites

```bash
# Install Rust
apk add rust cargo

# Install Git
apk add git

# Install GitHub CLI (may need to build from source or use a community repo)
# Check: https://github.com/cli/cli#installation
```

#### Install git-workflow

Build from source or use the release binary (see above).

> Note: Alpine uses musl libc. If using a release binary, you may need to
> build from source instead, as the pre-built binaries are linked against glibc.

---

### Gentoo

#### Prerequisites

```bash
# Install Rust
emerge --ask dev-lang/rust

# Install Git
emerge --ask dev-vcs/git

# Install GitHub CLI
emerge --ask dev-util/github-cli
```

#### Install git-workflow (ebuild)

An ebuild is provided in `packaging/gentoo/`. To use it with a local overlay:

```bash
# Create a local overlay (if you do not have one)
sudo mkdir -p /var/db/repos/local/dev-vcs/git-workflow

# Copy the ebuild and metadata
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git /tmp/git-workflow-src
sudo cp /tmp/git-workflow-src/packaging/gentoo/dev-vcs/git-workflow/* \
    /var/db/repos/local/dev-vcs/git-workflow/

# Generate the manifest
cd /var/db/repos/local/dev-vcs/git-workflow
sudo ebuild git-workflow-0.2.0.ebuild manifest

# Install
sudo emerge --ask dev-vcs/git-workflow
```

Alternatively, build from source with `cargo install` (see above).

---

### NixOS / Nix

#### Prerequisites

Nix handles all dependencies declaratively. You only need Nix itself:

```bash
# Install Nix (if not on NixOS)
sh <(curl -L https://nixos.org/nix/install) --daemon
```

#### Install git-workflow (Nix flake)

A Nix flake is provided in `packaging/nix/`:

```bash
# Run directly without installing
nix run github:nimonht/Simple-Rust-cli-wrapper

# Install to your profile
nix profile install github:nimonht/Simple-Rust-cli-wrapper

# Or build locally from the packaging directory
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper/packaging/nix
nix build
./result/bin/git-workflow --version
```

For non-flake users, a `default.nix` is also provided:

```bash
cd Simple-Rust-cli-wrapper/packaging/nix
nix-build
./result/bin/git-workflow --version
```

To add to your NixOS configuration or home-manager, import the package from
the flake or use `callPackage` with the `default.nix`.

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
