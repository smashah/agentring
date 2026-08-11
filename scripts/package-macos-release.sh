#!/bin/bash
# Build a separate public Agent Ring bundle. This script never reads from or
# writes to /Applications, so the locally signed install and its TCC grants are
# left untouched.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
APP_NAME="Agent Ring"
EXECUTABLE="agentring"
INFO_PLIST="${REPO_ROOT}/Resources/Info.plist"
APP_ICON="${REPO_ROOT}/Resources/AppIcon.icns"

MODE="release"
if [[ "${1:-}" == "--prepare-only" ]]; then
  MODE="prepare"
elif [[ $# -gt 0 ]]; then
  echo "usage: $0 [--prepare-only]" >&2
  exit 64
fi

for tool in cargo codesign ditto plutil security shasum spctl xcrun; do
  if ! command -v "${tool}" >/dev/null 2>&1; then
    echo "missing required tool: ${tool}" >&2
    exit 69
  fi
done

VERSION="$(plutil -extract CFBundleShortVersionString raw "${INFO_PLIST}")"
DIST_DIR="${REPO_ROOT}/dist"
mkdir -p "${DIST_DIR}"
OUTPUT_DIR="$(mktemp -d "${DIST_DIR}/agentring-public-${VERSION}.XXXXXX")"
BUNDLE_DIR="${OUTPUT_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${BUNDLE_DIR}/Contents"
ARCHIVE_NAME="Agent-Ring-${VERSION}-macOS.zip"
ARCHIVE_PATH="${OUTPUT_DIR}/${ARCHIVE_NAME}"
NOTARY_UPLOAD_PATH="${OUTPUT_DIR}/notary-upload.zip"
NOTARY_RESULT_PATH="${OUTPUT_DIR}/notary-result.json"

echo "==> building release binary"
cargo build --release --manifest-path "${REPO_ROOT}/Cargo.toml"

echo "==> assembling separate distribution bundle"
mkdir -p "${CONTENTS_DIR}/MacOS" "${CONTENTS_DIR}/Resources"
cp "${REPO_ROOT}/target/release/${EXECUTABLE}" "${CONTENTS_DIR}/MacOS/${EXECUTABLE}"
cp "${INFO_PLIST}" "${CONTENTS_DIR}/Info.plist"
cp "${APP_ICON}" "${CONTENTS_DIR}/Resources/AppIcon.icns"
plutil -lint "${CONTENTS_DIR}/Info.plist"

if [[ "${MODE}" == "prepare" ]]; then
  echo "==> prepared unsigned bundle: ${BUNDLE_DIR}"
  echo "Certificate issuance is still required before signing and notarization."
  exit 0
fi

IDENTITY="${AGENTRING_DEVELOPER_IDENTITY:-}"
if [[ -z "${IDENTITY}" ]]; then
  IDENTITIES="$(
    security find-identity -v -p codesigning |
      sed -n 's/.*"\(Developer ID Application:.*\)"/\1/p'
  )"
  IDENTITY_COUNT="$(printf '%s\n' "${IDENTITIES}" | sed '/^$/d' | wc -l | tr -d ' ')"
  if [[ "${IDENTITY_COUNT}" -ne 1 ]]; then
    echo "expected exactly one Developer ID Application identity; found ${IDENTITY_COUNT}" >&2
    echo "Issue the prepared certificate in issue #11, import it into the login Keychain, then retry." >&2
    exit 78
  fi
  IDENTITY="${IDENTITIES}"
fi

if ! security find-identity -v -p codesigning | grep -Fq "${IDENTITY}"; then
  echo "Developer ID Application identity is not available to codesign: ${IDENTITY}" >&2
  exit 78
fi

echo "==> signing with hardened runtime and secure timestamp"
codesign --force --deep --options runtime --timestamp --sign "${IDENTITY}" "${BUNDLE_DIR}"
codesign --verify --deep --strict --verbose=2 "${BUNDLE_DIR}"

echo "==> creating notarization upload"
ditto -c -k --keepParent "${BUNDLE_DIR}" "${NOTARY_UPLOAD_PATH}"

NOTARY_ARGS=()
if [[ -n "${AGENTRING_NOTARY_PROFILE:-}" ]]; then
  NOTARY_ARGS+=(--keychain-profile "${AGENTRING_NOTARY_PROFILE}")
elif [[ -n "${APPLE_API_KEY_PATH:-}" && -n "${APPLE_API_KEY_ID:-}" ]]; then
  NOTARY_ARGS+=(--key "${APPLE_API_KEY_PATH}" --key-id "${APPLE_API_KEY_ID}")
  if [[ -n "${APPLE_API_ISSUER_ID:-}" ]]; then
    NOTARY_ARGS+=(--issuer "${APPLE_API_ISSUER_ID}")
  fi
else
  NOTARY_ARGS+=(--keychain-profile agentring-notary)
fi

echo "==> submitting to Apple notary service"
xcrun notarytool submit "${NOTARY_UPLOAD_PATH}" \
  "${NOTARY_ARGS[@]}" \
  --wait --timeout 30m --output-format json >"${NOTARY_RESULT_PATH}"

NOTARY_STATUS="$(plutil -extract status raw -o - "${NOTARY_RESULT_PATH}" 2>/dev/null || true)"
if [[ "${NOTARY_STATUS}" != "Accepted" ]]; then
  echo "notarization was not accepted; inspect ${NOTARY_RESULT_PATH}" >&2
  exit 1
fi

echo "==> stapling and validating notarization ticket"
xcrun stapler staple "${BUNDLE_DIR}"
xcrun stapler validate "${BUNDLE_DIR}"
codesign --verify --deep --strict --verbose=2 "${BUNDLE_DIR}"
spctl --assess --type execute --verbose=4 "${BUNDLE_DIR}"

echo "==> creating final public archive"
ditto -c -k --keepParent "${BUNDLE_DIR}" "${ARCHIVE_PATH}"
(
  cd "${OUTPUT_DIR}"
  shasum -a 256 "${ARCHIVE_NAME}" >"${ARCHIVE_NAME}.sha256"
)

echo "==> public artifact ready"
echo "${ARCHIVE_PATH}"
echo "${ARCHIVE_PATH}.sha256"
