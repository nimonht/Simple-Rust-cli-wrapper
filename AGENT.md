# Agent Guide

This document describes the project for AI coding agents. See [README.md](README.md)
for user-facing documentation, commands, and quick start instructions.

## Project

`git-workflow` is a Rust CLI tool that automates Git workflows. It provides
four subcommands:

- `start <branch-name>` -- sync the default branch and create a new feature branch.
- `finish <pr-title>` -- stage all changes, commit, push, and open a Pull Request via GitHub CLI (`gh`).
- `dump [OPTIONS]` -- dump commits from a branch to patch or diff files (for kernel dev workflows). Options: `--branch`, `--commit`, `--all`, `--format` (patch/diff), `--output`, `--email`.
- `tui` -- launch interactive TUI mode (respects terminal theme).

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
packaging/aur/         -- Arch Linux AUR PKGBUILD and .SRCINFO
packaging/deb/         -- Debian/Ubuntu .deb build script and control template
packaging/gentoo/      -- Gentoo ebuild and metadata
packaging/nix/         -- Nix flake and default.nix derivation
packaging/rpm/         -- Fedora/RHEL RPM spec and build script
scripts/set-version.sh         -- version bump helper (updates all files)
scripts/generate-checksums.sh  -- SHA256 checksum generator for release tarballs
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

## Packaging

The project provides native packaging for several Linux distributions so users
can build from source and install through their package manager for proper
package tracking (install, upgrade, remove).

| Directory | Format | Build command |
|-----------|--------|---------------|
| `packaging/aur/` | Arch Linux PKGBUILD | `cd packaging/aur && makepkg -si` |
| `packaging/deb/` | Debian/Ubuntu .deb | `./packaging/deb/build-deb.sh --install` |
| `packaging/rpm/` | Fedora/RHEL RPM | `./packaging/rpm/build-rpm.sh --local --install` |
| `packaging/gentoo/` | Gentoo ebuild | See docs/platform-setup.md |
| `packaging/nix/` | Nix flake | `nix build` or `nix profile install .` |

### SHA256 Checksums

The AUR PKGBUILD uses `sha256sums` to verify the source tarball. During
development, this is set to `SKIP`. After tagging a release and pushing to
GitHub, run:

```bash
./scripts/generate-checksums.sh          # download tarball from GitHub, compute hash
./scripts/generate-checksums.sh --local  # use local git archive (pre-release testing)
```

This updates `packaging/aur/PKGBUILD` and `packaging/aur/.SRCINFO` with the
real checksum. The release CI workflow also generates a `SHA256SUMS` file
attached to each GitHub Release.

### Nix Hashes

The Nix files (`flake.nix`, `default.nix`) use `pkgs.lib.fakeHash` as
placeholders. After tagging a release, compute the real hashes with:

```bash
nix-prefetch-url --unpack \
  https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v<VERSION>.tar.gz
nix hash to-sri --type sha256 <hex-hash>
```

For local builds, the flake provides a `local` package variant that uses
`cargoLock.lockFile` instead of `cargoHash`, so no hash computation is needed.

## Releasing

The version is defined once in `Cargo.toml`. A helper script propagates it to
every file that contains a hardcoded version string (packaging files, docs,
lockfile, Gentoo ebuild filename):

```bash
./scripts/set-version.sh x.x.x   # bump to x.x.x everywhere
cargo test && cargo clippy -- -D warnings
git add -A && git commit -m "Bump version to x.x.x"
git tag vx.x.x && git push origin vx.x.x
./scripts/generate-checksums.sh   # fill in real SHA256 after tag is pushed
git add -A && git commit -m "Update checksums for vx.x.x"
git push
```

The `set-version.sh` script updates:

- `Cargo.toml` and regenerates `Cargo.lock`
- `packaging/aur/PKGBUILD` (pkgver, resets sha256sums to SKIP)
- `packaging/aur/.SRCINFO` (version strings, resets sha256sums to SKIP)
- `packaging/nix/flake.nix` and `packaging/nix/default.nix` (version string)
- `packaging/gentoo/` ebuild (renames file to new version)
- `packaging/rpm/git-workflow.spec` (Version and upstream_version macros)
- `docs/platform-setup.md` and `docs/installation-manual.md` (version refs)

The deb `control.template` uses a `@@VERSION@@` placeholder substituted at
build time by `build-deb.sh`, so it does not need version updates.

Never edit version strings by hand -- always use the script.

## CI/CD

- **CI** (`.github/workflows/ci.yml`): Runs on push/PR to main/master. Checks
  formatting, linting, tests, and cross-platform builds (Linux, macOS, Windows).
- **Release** (`.github/workflows/release.yml`): Triggered on `v*` tags. Builds
  release binaries for 4 targets, generates `SHA256SUMS` (for both release
  archives and the source tarball), and creates a GitHub Release with all
  artifacts attached.