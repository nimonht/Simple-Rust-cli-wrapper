#!/usr/bin/env bash
#
# set-version.sh -- Update the project version in every file that contains it.
#
# Usage:
#   ./scripts/set-version.sh <NEW_VERSION>
#
# Example:
#   ./scripts/set-version.sh 0.3.0
#
# This script treats Cargo.toml as the source of truth. It:
#   1. Reads the current version from Cargo.toml.
#   2. Updates Cargo.toml to the new version.
#   3. Runs `cargo generate-lockfile` to sync Cargo.lock.
#   4. Updates all packaging files (AUR, Gentoo, Nix).
#   5. Updates version references in documentation.
#   6. Renames the Gentoo ebuild file to match the new version.
#
# Requirements: bash, sed, cargo

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

# -- Validate input -----------------------------------------------------------

if [ $# -ne 1 ]; then
    echo "Usage: $0 <NEW_VERSION>"
    echo ""
    echo "Example: $0 0.3.0"
    echo ""
    echo "The version should be a semver string without a leading 'v'."
    exit 1
fi

NEW_VERSION="$1"

# Basic semver validation (MAJOR.MINOR.PATCH with optional pre-release)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
    die "Invalid version format: '$NEW_VERSION'. Expected semver (e.g. 0.3.0 or 1.0.0-rc.1)."
fi

# -- Read current version from Cargo.toml ------------------------------------

CARGO_TOML="$PROJECT_ROOT/Cargo.toml"

if [ ! -f "$CARGO_TOML" ]; then
    die "Cargo.toml not found at $CARGO_TOML"
fi

OLD_VERSION="$(grep -m1 '^version' "$CARGO_TOML" | sed 's/version *= *"\(.*\)"/\1/')"

if [ -z "$OLD_VERSION" ]; then
    die "Could not read current version from Cargo.toml"
fi

if [ "$OLD_VERSION" = "$NEW_VERSION" ]; then
    die "Version is already $NEW_VERSION -- nothing to do."
fi

info "Bumping version: $OLD_VERSION -> $NEW_VERSION"

# -- 1. Cargo.toml ------------------------------------------------------------

info "Updating Cargo.toml ..."
sedi "s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
ok "Cargo.toml"

# -- 2. Cargo.lock (regenerate) -----------------------------------------------

info "Regenerating Cargo.lock ..."
(cd "$PROJECT_ROOT" && cargo generate-lockfile --quiet 2>/dev/null) || {
    info "cargo generate-lockfile failed; trying cargo check instead ..."
    (cd "$PROJECT_ROOT" && cargo check --quiet 2>/dev/null) || true
}
ok "Cargo.lock"

# -- 3. AUR PKGBUILD ----------------------------------------------------------

PKGBUILD="$PROJECT_ROOT/packaging/aur/PKGBUILD"
if [ -f "$PKGBUILD" ]; then
    info "Updating packaging/aur/PKGBUILD ..."
    sedi "s/^pkgver=.*/pkgver=$NEW_VERSION/" "$PKGBUILD"
    ok "PKGBUILD"
else
    info "Skipping PKGBUILD (not found)"
fi

# -- 4. AUR .SRCINFO ----------------------------------------------------------

SRCINFO="$PROJECT_ROOT/packaging/aur/.SRCINFO"
if [ -f "$SRCINFO" ]; then
    info "Updating packaging/aur/.SRCINFO ..."
    sedi "s/pkgver = $OLD_VERSION/pkgver = $NEW_VERSION/g" "$SRCINFO"
    sedi "s/git-workflow-$OLD_VERSION/git-workflow-$NEW_VERSION/g" "$SRCINFO"
    sedi "s/v$OLD_VERSION/v$NEW_VERSION/g" "$SRCINFO"
    ok ".SRCINFO"
else
    info "Skipping .SRCINFO (not found)"
fi

# -- 5. Nix flake.nix ---------------------------------------------------------

NIX_FLAKE="$PROJECT_ROOT/packaging/nix/flake.nix"
if [ -f "$NIX_FLAKE" ]; then
    info "Updating packaging/nix/flake.nix ..."
    sedi "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$NIX_FLAKE"
    ok "flake.nix"
else
    info "Skipping flake.nix (not found)"
fi

# -- 6. Nix default.nix -------------------------------------------------------

NIX_DEFAULT="$PROJECT_ROOT/packaging/nix/default.nix"
if [ -f "$NIX_DEFAULT" ]; then
    info "Updating packaging/nix/default.nix ..."
    sedi "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" "$NIX_DEFAULT"
    ok "default.nix"
else
    info "Skipping default.nix (not found)"
fi

# -- 7. Gentoo ebuild (rename + content) --------------------------------------

GENTOO_DIR="$PROJECT_ROOT/packaging/gentoo/dev-vcs/git-workflow"
OLD_EBUILD="$GENTOO_DIR/git-workflow-$OLD_VERSION.ebuild"
NEW_EBUILD="$GENTOO_DIR/git-workflow-$NEW_VERSION.ebuild"

if [ -f "$OLD_EBUILD" ]; then
    info "Renaming Gentoo ebuild ..."
    mv "$OLD_EBUILD" "$NEW_EBUILD"
    ok "git-workflow-$OLD_VERSION.ebuild -> git-workflow-$NEW_VERSION.ebuild"
elif [ -d "$GENTOO_DIR" ]; then
    # Try to find any ebuild file in the directory
    EXISTING_EBUILD="$(find "$GENTOO_DIR" -name '*.ebuild' -print -quit 2>/dev/null || true)"
    if [ -n "$EXISTING_EBUILD" ]; then
        info "Renaming Gentoo ebuild (found $EXISTING_EBUILD) ..."
        mv "$EXISTING_EBUILD" "$NEW_EBUILD"
        ok "$(basename "$EXISTING_EBUILD") -> git-workflow-$NEW_VERSION.ebuild"
    else
        info "Skipping Gentoo ebuild (no .ebuild file found)"
    fi
else
    info "Skipping Gentoo ebuild (directory not found)"
fi

# -- 8. Documentation references ----------------------------------------------

PLATFORM_SETUP="$PROJECT_ROOT/docs/platform-setup.md"
if [ -f "$PLATFORM_SETUP" ]; then
    info "Updating docs/platform-setup.md ..."
    sedi "s/git-workflow-$OLD_VERSION/git-workflow-$NEW_VERSION/g" "$PLATFORM_SETUP"
    ok "platform-setup.md"
else
    info "Skipping platform-setup.md (not found)"
fi

# -- Summary ------------------------------------------------------------------

echo ""
echo "==========================================="
echo "  Version updated: $OLD_VERSION -> $NEW_VERSION"
echo "==========================================="
echo ""
echo "Files modified:"
echo "  - Cargo.toml"
echo "  - Cargo.lock"
[ -f "$PKGBUILD" ]      && echo "  - packaging/aur/PKGBUILD"
[ -f "$SRCINFO" ]        && echo "  - packaging/aur/.SRCINFO"
[ -f "$NIX_FLAKE" ]      && echo "  - packaging/nix/flake.nix"
[ -f "$NIX_DEFAULT" ]    && echo "  - packaging/nix/default.nix"
[ -f "$NEW_EBUILD" ]     && echo "  - packaging/gentoo/dev-vcs/git-workflow/git-workflow-$NEW_VERSION.ebuild (renamed)"
[ -f "$PLATFORM_SETUP" ] && echo "  - docs/platform-setup.md"
echo ""
echo "Next steps:"
echo "  1. Review the changes:  git diff"
echo "  2. Run the checks:      cargo test && cargo clippy -- -D warnings"
echo "  3. Commit:              git add -A && git commit -m \"Bump version to $NEW_VERSION\""
echo "  4. Tag the release:     git tag v$NEW_VERSION && git push origin v$NEW_VERSION"
