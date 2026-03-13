#!/usr/bin/env bash
#
# generate-checksums.sh -- Generate SHA256 checksums for the release tarball
#                          and update AUR packaging files.
#
# Usage:
#   ./scripts/generate-checksums.sh [OPTIONS] [VERSION]
#
# Arguments:
#   VERSION   The version to generate checksums for (without leading 'v').
#             Defaults to the version in Cargo.toml.
#
# Options:
#   --local   Instead of downloading the tarball from GitHub, create one from
#             the local git repo using `git archive` (HEAD). Useful for
#             pre-release testing before the tag exists on GitHub.
#
# Examples:
#   ./scripts/generate-checksums.sh                # use Cargo.toml version, download from GitHub
#   ./scripts/generate-checksums.sh 0.4.0          # specific version, download from GitHub
#   ./scripts/generate-checksums.sh --local        # use Cargo.toml version, local git archive
#   ./scripts/generate-checksums.sh --local 0.4.0  # specific version, local git archive
#
# This script:
#   1. Downloads (or locally creates) the source tarball for the given version.
#   2. Computes the SHA256 checksum.
#   3. Updates packaging/aur/PKGBUILD with the real checksum.
#   4. Updates packaging/aur/.SRCINFO with the real checksum.
#   5. Prints the checksum to stdout for reference.
#
# Requirements: bash, curl (unless --local), git (for --local), sha256sum, sed

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# -- Helpers ------------------------------------------------------------------

die() {
    echo "[ERROR] $*" >&2
    exit 1
}

info() {
    echo "[INFO] $*"
}

ok() {
    echo "[OK] $*"
}

# Cross-platform sed -i wrapper (GNU vs BSD/macOS)
sedi() {
    if sed --version >/dev/null 2>&1; then
        # GNU sed
        sed -i "$@"
    else
        # BSD/macOS sed
        sed -i '' "$@"
    fi
}

# -- Parse arguments ----------------------------------------------------------

USE_LOCAL=false
VERSION=""

while [ $# -gt 0 ]; do
    case "$1" in
        --local)
            USE_LOCAL=true
            shift
            ;;
        -h|--help)
            head -n 30 "$0" | tail -n +2 | sed 's/^# \?//'
            exit 0
            ;;
        -*)
            die "Unknown option: $1"
            ;;
        *)
            if [ -n "$VERSION" ]; then
                die "Unexpected argument: $1 (version already set to $VERSION)"
            fi
            VERSION="$1"
            shift
            ;;
    esac
done

# -- Resolve version ----------------------------------------------------------

CARGO_TOML="$PROJECT_ROOT/Cargo.toml"

if [ -z "$VERSION" ]; then
    if [ ! -f "$CARGO_TOML" ]; then
        die "Cargo.toml not found at $CARGO_TOML"
    fi
    VERSION="$(grep -m1 '^version' "$CARGO_TOML" | sed 's/version *= *"\(.*\)"/\1/')"
    if [ -z "$VERSION" ]; then
        die "Could not read version from Cargo.toml"
    fi
    info "Using version from Cargo.toml: $VERSION"
else
    info "Using provided version: $VERSION"
fi

# -- Create temp directory ----------------------------------------------------

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

TARBALL="$TMPDIR/v${VERSION}.tar.gz"

# -- Obtain tarball -----------------------------------------------------------

GITHUB_URL="https://github.com/nimonht/Simple-Rust-cli-wrapper/archive/v${VERSION}.tar.gz"

if [ "$USE_LOCAL" = true ]; then
    info "Creating tarball from local git repo (HEAD) ..."
    if ! command -v git >/dev/null 2>&1; then
        die "git is required for --local mode"
    fi
    (cd "$PROJECT_ROOT" && git archive --format=tar.gz --prefix="Simple-Rust-cli-wrapper-${VERSION}/" HEAD) > "$TARBALL"
    ok "Local tarball created"
else
    info "Downloading tarball from $GITHUB_URL ..."
    if ! command -v curl >/dev/null 2>&1; then
        die "curl is required to download the tarball (use --local to skip download)"
    fi
    if ! curl -fSL -o "$TARBALL" "$GITHUB_URL"; then
        die "Failed to download tarball. Is v${VERSION} tagged and pushed?"
    fi
    ok "Tarball downloaded"
fi

# -- Compute checksum ---------------------------------------------------------

if ! command -v sha256sum >/dev/null 2>&1; then
    # macOS fallback
    if command -v shasum >/dev/null 2>&1; then
        CHECKSUM="$(shasum -a 256 "$TARBALL" | awk '{print $1}')"
    else
        die "sha256sum (or shasum) is required"
    fi
else
    CHECKSUM="$(sha256sum "$TARBALL" | awk '{print $1}')"
fi

if [ -z "$CHECKSUM" ]; then
    die "Failed to compute SHA256 checksum"
fi

ok "SHA256: $CHECKSUM"

# -- Update PKGBUILD ----------------------------------------------------------

PKGBUILD="$PROJECT_ROOT/packaging/aur/PKGBUILD"

if [ -f "$PKGBUILD" ]; then
    info "Updating packaging/aur/PKGBUILD ..."
    sedi "s/^sha256sums=(.*/sha256sums=('${CHECKSUM}')/" "$PKGBUILD"
    ok "PKGBUILD"
else
    info "Skipping PKGBUILD (not found)"
fi

# -- Update .SRCINFO -----------------------------------------------------------

SRCINFO="$PROJECT_ROOT/packaging/aur/.SRCINFO"

if [ -f "$SRCINFO" ]; then
    info "Updating packaging/aur/.SRCINFO ..."
    sedi "s/^\tsha256sums = .*/\tsha256sums = ${CHECKSUM}/" "$SRCINFO"
    ok ".SRCINFO"
else
    info "Skipping .SRCINFO (not found)"
fi

# -- Summary ------------------------------------------------------------------

echo ""
echo "==========================================="
echo "  Checksums for v${VERSION}"
echo "==========================================="
echo ""
echo "  SHA256: ${CHECKSUM}"
echo ""
if [ "$USE_LOCAL" = true ]; then
    echo "  Source: local git archive (HEAD)"
else
    echo "  Source: ${GITHUB_URL}"
fi
echo ""
echo "Files updated:"
[ -f "$PKGBUILD" ] && echo "  - packaging/aur/PKGBUILD"
[ -f "$SRCINFO" ]  && echo "  - packaging/aur/.SRCINFO"
echo ""
echo "Next steps:"
echo "  1. Review the changes:  git diff packaging/aur/"
echo "  2. Commit:              git add -A && git commit -m \"Update checksums for v${VERSION}\""
