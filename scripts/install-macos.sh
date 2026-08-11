#!/bin/bash
# Build the release binary, assemble Agent Ring.app, replace the installed
# copy at /Applications, and re-sign with the local signing identity.
#
# Pinned to the identity already used by the installed app (Agent Ring Local
# Signing, SHA-1 756D61D90B739AE33588463EF14A2B1AB3104B52) so TCC grants keyed
# to the code signature survive rebuilds (docs/PRD.md, M4). The script never
# creates or switches identities; if the keychain cannot sign non-interactively
# it prints the exact codesign error and leaves the installed app untouched.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
APP_NAME="Agent Ring"
APP_DIR="/Applications/${APP_NAME}.app"
IDENTITY="756D61D90B739AE33588463EF14A2B1AB3104B52"
KEYCHAIN="/Users/Mohammed/Library/Keychains/agentring-signing.keychain-db"

echo "==> cargo build --release"
cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"

BUNDLE_DIR="${REPO_ROOT}/dist/${APP_NAME}.app"
echo "==> assembling ${BUNDLE_DIR}"
rm -rf "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}/Contents/MacOS" "${BUNDLE_DIR}/Contents/Resources"
cp "${REPO_ROOT}/target/release/agentring" "${BUNDLE_DIR}/Contents/MacOS/agentring"
cp "${REPO_ROOT}/Resources/Info.plist" "${BUNDLE_DIR}/Contents/Info.plist"
cp "${REPO_ROOT}/Resources/AppIcon.icns" "${BUNDLE_DIR}/Contents/Resources/AppIcon.icns"

echo "==> signing with ${IDENTITY} (keychain: ${KEYCHAIN})"
codesign --force --deep --sign "${IDENTITY}" \
  --keychain "${KEYCHAIN}" "${BUNDLE_DIR}"

echo "==> replacing ${APP_DIR}"
rm -rf "${APP_DIR}"
mv "${BUNDLE_DIR}" "${APP_DIR}"

echo "==> verifying signature"
codesign -dv --verbose=4 "${APP_DIR}" 2>&1 | grep -E "Identifier|Authority|Signature"
echo "==> done: ${APP_DIR}"