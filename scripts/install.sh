#!/bin/sh
set -e

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

LATEST_TAG=$(curl -s https://api.github.com/repos/zbrlang/zbr/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

URL="https://github.com/zbrlang/zbr/releases/download/$LATEST_TAG/$TARGET"
curl -L "$URL" -o /tmp/zbr-engine
chmod +x /tmp/zbr-engine
sudo mv /tmp/zbr-engine /usr/local/bin/zbr-engine

cat <<'EOF' | sudo tee /usr/local/bin/zbr > /dev/null
#!/bin/sh
case "$1" in
  run)
    shift
    /usr/local/bin/zbr-engine "$@"
    ;;
  version|-v|--version)
    /usr/local/bin/zbr-engine --version
    ;;
  help|--help)
    echo "Usage: zbr [command]"
    echo "Commands:"
    echo "  run          Start the bot"
    echo "  version      Show version"
    ;;
  *)
    echo "Unknown command: $1"
    echo "Use 'zbr help' for usage."
    exit 1
    ;;
esac
EOF
sudo chmod +x /usr/local/bin/zbr

echo "Successfully installed ZBR CLI ($LATEST_TAG) to /usr/local/bin/zbr"
