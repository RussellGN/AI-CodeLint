#!/usr/bin/env sh
set -e

REPO="RussellGN/AI-CodeLint"
APP_NAME="ai-codelint"

echo "Installing $APP_NAME..."

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64) TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *) echo "Unsupported macOS architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported Linux architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Get latest release tag
TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | cut -d '"' -f 4)

FILE="$APP_NAME-$TAG-$TARGET"
URL="https://github.com/$REPO/releases/download/$TAG/$FILE"

echo "Downloading $FILE..."
curl -L "$URL" -o "$APP_NAME"

chmod +x "$APP_NAME"

INSTALL_DIR="/usr/local/bin"

if [ ! -w "$INSTALL_DIR" ]; then
  INSTALL_DIR="$HOME/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$APP_NAME" "$INSTALL_DIR/$APP_NAME"

echo "Installed to $INSTALL_DIR/$APP_NAME"

case ":$PATH:" in
  *":$INSTALL_DIR:"*)
    echo "✔ Ready to use: $APP_NAME"
    ;;
  *)
    echo "⚠ Add to PATH:"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\""
    ;;
esac
