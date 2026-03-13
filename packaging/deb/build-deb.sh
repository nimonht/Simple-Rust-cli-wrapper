#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT_DEFAULT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PKG_NAME="git-workflow"
INSTALL_FLAG=0

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
info()  { echo "[INFO]  $*"; }
ok()    { echo "[OK]    $*"; }
error() { echo "[ERROR] $*" >&2; }

# Cross-platform sed -i wrapper (GNU vs BSD/macOS)
sedi() {
    if sed --version >/dev/null 2>&1; then
        sed -i "$@"
    else
        sed -i '' "$@"
    fi
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS] [PROJECT_ROOT]

Build a .deb package for ${PKG_NAME}.

Arguments:
  PROJECT_ROOT    Path to the project root (default: current working directory)

Options:
  --install       Install the .deb package after building (requires sudo/root)
  -h, --help      Show this help message
EOF
}

# ---------------------------------------------------------------------------
# Parse arguments
# ---------------------------------------------------------------------------
PROJECT_ROOT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --install)
            INSTALL_FLAG=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            error "Unknown option: $1"
            usage
            exit 1
            ;;
        *)
            PROJECT_ROOT="$1"
            shift
            ;;
    esac
done

if [[ -z "${PROJECT_ROOT}" ]]; then
    PROJECT_ROOT="${PROJECT_ROOT_DEFAULT}"
fi

PROJECT_ROOT="$(cd "${PROJECT_ROOT}" && pwd)"

# ---------------------------------------------------------------------------
# Validate project root
# ---------------------------------------------------------------------------
if [[ ! -f "${PROJECT_ROOT}/Cargo.toml" ]]; then
    error "Cargo.toml not found in ${PROJECT_ROOT}"
    error "Run this script from the project root or pass it as an argument."
    exit 1
fi

# ---------------------------------------------------------------------------
# Read version from Cargo.toml
# ---------------------------------------------------------------------------
VERSION="$(grep -m1 '^version' "${PROJECT_ROOT}/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')"
if [[ -z "${VERSION}" ]]; then
    error "Could not read version from Cargo.toml"
    exit 1
fi

# Debian versions cannot contain hyphens except to separate upstream version
# from the debian revision.  Convert e.g. "1.0.0-rc1" -> "1.0.0~rc1" so that
# dpkg-deb is happy and the version sorts correctly (~ sorts before anything).
DEB_VERSION="${VERSION//-/\~}"

info "Package: ${PKG_NAME}"
info "Upstream version: ${VERSION}"
info "Debian version:   ${DEB_VERSION}"

# ---------------------------------------------------------------------------
# Detect architecture
# ---------------------------------------------------------------------------
ARCH="$(dpkg --print-architecture 2>/dev/null || echo "amd64")"
info "Architecture: ${ARCH}"

# ---------------------------------------------------------------------------
# Build the project
# ---------------------------------------------------------------------------
info "Building ${PKG_NAME} with cargo build --release ..."
(cd "${PROJECT_ROOT}" && cargo build --release)
ok "Build succeeded."

BINARY="${PROJECT_ROOT}/target/release/${PKG_NAME}"
if [[ ! -f "${BINARY}" ]]; then
    error "Expected binary not found at ${BINARY}"
    exit 1
fi

# ---------------------------------------------------------------------------
# Prepare .deb staging directory
# ---------------------------------------------------------------------------
STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "${STAGE_DIR}"' EXIT

DEB_ROOT="${STAGE_DIR}/${PKG_NAME}_${DEB_VERSION}_${ARCH}"

info "Staging .deb contents in ${DEB_ROOT} ..."

# Control file ----------------------------------------------------------
mkdir -p "${DEB_ROOT}/DEBIAN"

CONTROL_TEMPLATE="${SCRIPT_DIR}/control.template"
if [[ ! -f "${CONTROL_TEMPLATE}" ]]; then
    error "control.template not found at ${CONTROL_TEMPLATE}"
    exit 1
fi

sed "s/@@VERSION@@/${DEB_VERSION}/g" "${CONTROL_TEMPLATE}" \
    > "${DEB_ROOT}/DEBIAN/control"

# Ensure the control file uses the detected architecture
sedi "s/^Architecture:.*/Architecture: ${ARCH}/" "${DEB_ROOT}/DEBIAN/control"

ok "Control file generated."

# Binary -----------------------------------------------------------------
mkdir -p "${DEB_ROOT}/usr/bin"
install -m 0755 "${BINARY}" "${DEB_ROOT}/usr/bin/${PKG_NAME}"
ok "Installed binary to usr/bin/${PKG_NAME}"

# License / copyright ---------------------------------------------------
DOC_DIR="${DEB_ROOT}/usr/share/doc/${PKG_NAME}"
mkdir -p "${DOC_DIR}"

LICENSE_FILE="${PROJECT_ROOT}/LICENSE"
if [[ -f "${LICENSE_FILE}" ]]; then
    # Debian policy: the copyright file lives at /usr/share/doc/<pkg>/copyright
    cp "${LICENSE_FILE}" "${DOC_DIR}/copyright"
    ok "Installed license to usr/share/doc/${PKG_NAME}/copyright"
else
    error "LICENSE file not found at ${LICENSE_FILE} -- skipping copyright."
fi

# ---------------------------------------------------------------------------
# Build the .deb
# ---------------------------------------------------------------------------
OUTPUT_DIR="${PROJECT_ROOT}/target"
mkdir -p "${OUTPUT_DIR}"

DEB_FILE="${OUTPUT_DIR}/${PKG_NAME}_${DEB_VERSION}_${ARCH}.deb"

info "Building .deb package ..."
dpkg-deb --build "${DEB_ROOT}" "${DEB_FILE}"
ok "Package created: ${DEB_FILE}"

# Show package info
dpkg-deb --info "${DEB_FILE}"

# ---------------------------------------------------------------------------
# Optional install
# ---------------------------------------------------------------------------
if [[ "${INSTALL_FLAG}" -eq 1 ]]; then
    info "Installing ${DEB_FILE} ..."
    if [[ "$(id -u)" -eq 0 ]]; then
        dpkg -i "${DEB_FILE}"
    else
        sudo dpkg -i "${DEB_FILE}"
    fi
    ok "${PKG_NAME} installed."
fi

ok "Done."
