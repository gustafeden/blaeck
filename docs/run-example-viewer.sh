#!/bin/bash
set -e

REPO="gustafeden/blaeck"
BINARY="example_viewer"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS-$ARCH" in
  linux-x86_64)
    ARTIFACT="example_viewer-linux-x86_64"
    ;;
  darwin-x86_64)
    ARTIFACT="example_viewer-macos-x86_64"
    ;;
  darwin-arm64)
    ARTIFACT="example_viewer-macos-arm64"
    ;;
  *)
    echo "❌ Unsupported platform: $OS-$ARCH"
    echo "Supported: Linux x86_64, macOS x86_64, macOS ARM64"
    exit 1
    ;;
esac

# Get latest release
echo "🔍 Fetching latest release..."
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
  echo "❌ Could not fetch latest release"
  exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST/$ARTIFACT"
TEMP_DIR=$(mktemp -d)
TEMP_BINARY="$TEMP_DIR/$BINARY"

# Download binary
echo "📦 Downloading $BINARY $LATEST..."
if ! curl -fsSL "$URL" -o "$TEMP_BINARY"; then
  echo "❌ Download failed"
  rm -rf "$TEMP_DIR"
  exit 1
fi

chmod +x "$TEMP_BINARY"

echo "🚀 Starting example viewer..."
echo "   (Press q or Ctrl+C to quit)"
echo ""

# Run the binary
"$TEMP_BINARY"

# Cleanup
echo ""
echo "🧹 Cleaning up..."
rm -rf "$TEMP_DIR"
echo "✨ Done!"
