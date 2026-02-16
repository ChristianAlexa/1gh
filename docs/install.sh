#!/bin/sh
set -e

REPO="christianalexa/1gh"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="1gh"

OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" != "Darwin" ]; then
  echo "Error: This installer currently supports macOS only." >&2
  exit 1
fi

case "$ARCH" in
  arm64)  ASSET="1gh-macos-arm64" ;;
  x86_64) ASSET="1gh-macos-x86_64" ;;
  *)      echo "Error: Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"

echo "Downloading ${BINARY_NAME} for macOS ${ARCH}..."
curl -fsSL "$URL" -o "/tmp/${BINARY_NAME}"
chmod +x "/tmp/${BINARY_NAME}"

echo "Installing to ${INSTALL_DIR}/${BINARY_NAME} (may require sudo)..."
if [ -w "$INSTALL_DIR" ]; then
  mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
else
  sudo mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
fi

echo "Done! Run '1gh' to start."
