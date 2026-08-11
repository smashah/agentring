#!/bin/bash
# Derive the macOS AppIcon from the canonical physical-ring mark. This is a
# deterministic format/polarity conversion; it does not generate a second logo.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOURCE="${REPO_ROOT}/assets/brand/Gemini_Generated_Image_8x0lj88x0lj88x0l.png"
DESTINATION="${REPO_ROOT}/Resources/AppIcon.icns"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/agentring-appicon.XXXXXX")"
ICONSET="${WORK_DIR}/AppIcon.iconset"

cleanup() {
  case "${WORK_DIR}" in
    "${TMPDIR:-/tmp}"/agentring-appicon.*) rm -rf -- "${WORK_DIR}" ;;
  esac
}
trap cleanup EXIT

for tool in magick iconutil; do
  if ! command -v "${tool}" >/dev/null 2>&1; then
    echo "missing required tool: ${tool}" >&2
    exit 69
  fi
done

mkdir -p "${ICONSET}"

magick -size 1024x1024 xc:none \
  -fill '#08090c' \
  -draw 'roundrectangle 0,0 1023,1023 224,224' \
  "${WORK_DIR}/background.png"

magick "${SOURCE}" \
  -channel RGB -negate +channel \
  -resize 680x680 \
  "${WORK_DIR}/ring.png"

magick "${WORK_DIR}/background.png" "${WORK_DIR}/ring.png" \
  -gravity center -geometry +0+8 -composite \
  "${WORK_DIR}/master.png"

render_size() {
  local output_name="$1"
  local pixels="$2"
  magick "${WORK_DIR}/master.png" -resize "${pixels}x${pixels}" "${ICONSET}/${output_name}"
}

render_size icon_16x16.png 16
render_size icon_16x16@2x.png 32
render_size icon_32x32.png 32
render_size icon_32x32@2x.png 64
render_size icon_128x128.png 128
render_size icon_128x128@2x.png 256
render_size icon_256x256.png 256
render_size icon_256x256@2x.png 512
render_size icon_512x512.png 512
render_size icon_512x512@2x.png 1024

iconutil -c icns "${ICONSET}" -o "${DESTINATION}"
echo "updated ${DESTINATION} from ${SOURCE}"
