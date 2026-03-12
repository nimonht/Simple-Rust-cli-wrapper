# Installation Manual

This is the authoritative installation guide for `git-workflow`. For
platform-specific details (distro packages, Homebrew, winget, BSD), see
[platform-setup.md](platform-setup.md).

## Prerequisites

Before installing `git-workflow`, make sure you have the following tools
installed:

| Tool | Purpose | Install guide |
|------|---------|---------------|
| **Rust** (cargo) | Build the CLI from source | https://rustup.rs |
| **Git** | Version control | https://git-scm.com/downloads |
| **GitHub CLI** (gh) | Create Pull Requests from the terminal | https://cli.github.com |

Rust is only required when building from source. If you install from a
pre-built release binary, you only need Git and GitHub CLI.

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

For platform-specific download and extraction steps, see
[platform-setup.md](platform-setup.md).

## Install via package manager (Linux)

Community-maintained packaging files are provided in the `packaging/` directory
for several Linux distributions. These are not yet published to official
repositories -- you can build and install locally from the packaging files.

### Arch Linux (AUR)

A PKGBUILD is provided in `packaging/aur/`. Install with an AUR helper or
build manually:

```bash
# Using yay (if published to AUR)
yay -S git-workflow

# Using paru (if published to AUR)
paru -S git-workflow

# Build from the PKGBUILD manually
git clone https://github.com/nimonht/Simple-Rust-cli-wrapper.git
cd Simple-Rust-cli-wrapper/packaging/aur
makepkg -si
```

### Gentoo (ebuild)

An ebuild is provided in `packaging/gentoo/`. See
[platform-setup.md](platform-setup.md#gentoo) for instructions on setting up a
local overlay and installing via `emerge`.

### NixOS / Nix (flake)

A Nix flake is provided in `packaging/nix/`:

```bash
# Run directly without installing
nix run github:nimonht/Simple-Rust-cli-wrapper

# Install to your profile
nix profile install github:nimonht/Simple-Rust-cli-wrapper
```

For non-flake users, a `default.nix` is also provided:

```bash
cd Simple-Rust-cli-wrapper/packaging/nix
nix-build
./result/bin/git-workflow --version
```

For full per-distro prerequisites and setup instructions (Debian, Fedora, Arch,
openSUSE, Void, Alpine, Gentoo, NixOS, and more), see
[platform-setup.md](platform-setup.md).

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

Or download the newest release binary from the
[Releases](https://github.com/nimonht/Simple-Rust-cli-wrapper/releases) page.
