#!/bin/sh
set -e

# Detect OS and Arch
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    if [ "$ARCH" = "x86_64" ]; then TARGET="zbr-linux-x64";
    elif [ "$ARCH" = "aarch64" ]; then TARGET="zbr-linux-arm64";
    else echo "Unsupported architecture: $ARCH"; exit 1; fi
    ;;
  darwin)
    if [ "$ARCH" = "x86_64" ]; then TARGET="zbr-darwin-x64";
    elif [ "$ARCH" = "arm64" ]; then TARGET="zbr-darwin-arm64";
    else echo "Unsupported architecture: $ARCH"; exit 1; fi
    ;;
  *) echo "Unsupported OS: $OS"; exit 1; ;;
esac

echo "Installing ZBR CLI for $OS $ARCH..."

# Get latest release tag
LATEST_TAG=$(curl -s https://api.github.com/repos/zbrlang/zbr/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Download and install
URL="https://github.com/zbrlang/zbr/releases/download/$LATEST_TAG/$TARGET"
curl -L "$URL" -o /tmp/zbr
chmod +x /tmp/zbr
sudo mv /tmp/zbr /usr/local/bin/zbr

echo "Successfully installed ZBR CLI ($LATEST_TAG) to /usr/local/bin/zbr"
