#!/usr/bin/env bash
#
# build-rpm.sh -- Build an RPM package for git-workflow.
#
# Usage:
#   ./build-rpm.sh [--local] [--install]
#
# Options:
#   --local     Build from local source tree instead of downloading a tarball.
#   --install   Install the resulting RPM with sudo rpm -i after building.
#
# Requirements: rpmbuild, cargo, rust, git

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SPEC_FILE="$SCRIPT_DIR/git-workflow.spec"

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

# -- Parse arguments ----------------------------------------------------------

USE_LOCAL=false
DO_INSTALL=false

for arg in "$@"; do
    case "$arg" in
        --local)
            USE_LOCAL=true
            ;;
        --install)
            DO_INSTALL=true
            ;;
        -h|--help)
            echo "Usage: $0 [--local] [--install]"
            echo ""
            echo "Options:"
            echo "  --local     Build from local source tree instead of downloading a tarball."
            echo "  --install   Install the resulting RPM with sudo rpm -i after building."
            exit 0
            ;;
        *)
            die "Unknown option: $arg (use --help for usage)"
            ;;
    esac
done

# -- Read version from Cargo.toml --------------------------------------------

CARGO_TOML="$PROJECT_ROOT/Cargo.toml"

if [ ! -f "$CARGO_TOML" ]; then
    die "Cargo.toml not found at $CARGO_TOML"
fi

UPSTREAM_VERSION="$(grep -m1 '^version' "$CARGO_TOML" | sed 's/version *= *"\(.*\)"/\1/')"

if [ -z "$UPSTREAM_VERSION" ]; then
    die "Could not read version from Cargo.toml"
fi

# RPM does not allow hyphens in Version; Fedora convention uses tilde for
# pre-release (e.g. 1.0.0-rc1 -> 1.0.0~rc1).
RPM_VERSION="$(echo "$UPSTREAM_VERSION" | sed 's/-/~/')"

info "Upstream version: $UPSTREAM_VERSION"
info "RPM version:      $RPM_VERSION"

# -- Validate prerequisites ---------------------------------------------------

if ! command -v rpmbuild >/dev/null 2>&1; then
    die "rpmbuild not found. Install rpm-build (e.g. sudo dnf install rpm-build)."
fi

if ! command -v cargo >/dev/null 2>&1; then
    die "cargo not found. Install the Rust toolchain first."
fi

if [ ! -f "$SPEC_FILE" ]; then
    die "Spec file not found at $SPEC_FILE"
fi

# -- Set up rpmbuild directory tree -------------------------------------------

RPMBUILD_DIR="$PROJECT_ROOT/target/rpmbuild"

info "Setting up rpmbuild tree at $RPMBUILD_DIR ..."

mkdir -p "$RPMBUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

ok "rpmbuild tree created"

# -- Copy spec file -----------------------------------------------------------

cp "$SPEC_FILE" "$RPMBUILD_DIR/SPECS/git-workflow.spec"

ok "Spec file copied to SPECS/"

# -- Prepare source tarball ---------------------------------------------------

REPO_NAME="Simple-Rust-cli-wrapper"
TARBALL_NAME="${REPO_NAME}-${UPSTREAM_VERSION}.tar.gz"
TARBALL_PATH="$RPMBUILD_DIR/SOURCES/$TARBALL_NAME"

if [ "$USE_LOCAL" = true ]; then
    info "Building source tarball from local tree ..."

    # Create a temporary directory with the expected top-level name
    STAGING_DIR="$(mktemp -d)"
    STAGING_SUBDIR="$STAGING_DIR/${REPO_NAME}-${UPSTREAM_VERSION}"
    trap 'rm -rf "$STAGING_DIR"' EXIT

    # Use git archive if inside a git repo, otherwise fall back to cp
    if (cd "$PROJECT_ROOT" && git rev-parse --is-inside-work-tree >/dev/null 2>&1); then
        info "Using git archive to create tarball ..."
        mkdir -p "$STAGING_SUBDIR"
        (cd "$PROJECT_ROOT" && git archive --format=tar HEAD) | tar -x -C "$STAGING_SUBDIR"
    else
        info "Not a git repository; copying files directly ..."
        mkdir -p "$STAGING_SUBDIR"
        # Copy project contents, excluding build artifacts
        rsync -a \
            --exclude='target/' \
            --exclude='.git/' \
            --exclude='target/rpmbuild/' \
            "$PROJECT_ROOT/" "$STAGING_SUBDIR/"
    fi

    # Create the tarball
    (cd "$STAGING_DIR" && tar czf "$TARBALL_PATH" "${REPO_NAME}-${UPSTREAM_VERSION}")

    ok "Local source tarball created: $TARBALL_NAME"
else
    DOWNLOAD_URL="https://github.com/nimonht/${REPO_NAME}/archive/v${UPSTREAM_VERSION}/${TARBALL_NAME}"

    if [ -f "$TARBALL_PATH" ]; then
        info "Source tarball already exists at $TARBALL_PATH, skipping download."
    else
        info "Downloading source tarball from $DOWNLOAD_URL ..."

        if command -v curl >/dev/null 2>&1; then
            curl -fSL -o "$TARBALL_PATH" "$DOWNLOAD_URL" \
                || die "Failed to download source tarball from $DOWNLOAD_URL"
        elif command -v wget >/dev/null 2>&1; then
            wget -q -O "$TARBALL_PATH" "$DOWNLOAD_URL" \
                || die "Failed to download source tarball from $DOWNLOAD_URL"
        else
            die "Neither curl nor wget found. Install one or use --local."
        fi

        ok "Source tarball downloaded: $TARBALL_NAME"
    fi
fi

# -- Build the RPM ------------------------------------------------------------

info "Running rpmbuild ..."

rpmbuild \
    --define "_topdir $RPMBUILD_DIR" \
    -ba "$RPMBUILD_DIR/SPECS/git-workflow.spec"

ok "rpmbuild completed successfully"

# -- Locate the output RPM ----------------------------------------------------

RPM_FILE="$(find "$RPMBUILD_DIR/RPMS" -name 'git-workflow-*.rpm' -not -name '*.src.rpm' -print -quit 2>/dev/null || true)"
SRPM_FILE="$(find "$RPMBUILD_DIR/SRPMS" -name 'git-workflow-*.src.rpm' -print -quit 2>/dev/null || true)"

echo ""
echo "==========================================="
echo "  Build complete"
echo "==========================================="
echo ""

if [ -n "$RPM_FILE" ]; then
    ok "RPM:  $RPM_FILE"
else
    info "RPM file not found in $RPMBUILD_DIR/RPMS (check build output above)"
fi

if [ -n "$SRPM_FILE" ]; then
    ok "SRPM: $SRPM_FILE"
fi

# -- Optionally install -------------------------------------------------------

if [ "$DO_INSTALL" = true ]; then
    if [ -z "$RPM_FILE" ]; then
        die "Cannot install: RPM file not found."
    fi

    info "Installing $RPM_FILE ..."

    if sudo rpm -i --force "$RPM_FILE"; then
        ok "git-workflow installed successfully"
        info "Verify with: git-workflow --help"
    else
        die "rpm -i failed. Check output above."
    fi
fi
