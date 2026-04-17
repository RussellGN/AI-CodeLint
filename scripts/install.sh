#!/usr/bin/env sh
set -e

REPO="RussellGN/AI-CodeLint"
APP_NAME="ai-codelint"

# ── Colors ────────────────────────────────────────────────────────────────────
if [ -t 1 ]; then
  BOLD="\033[1m"
  DIM="\033[2m"
  GREEN="\033[0;32m"
  YELLOW="\033[0;33m"
  RED="\033[0;31m"
  CYAN="\033[0;36m"
  RESET="\033[0m"
else
  BOLD=""; DIM=""; GREEN=""; YELLOW=""; RED=""; CYAN=""; RESET=""
fi

info()    { printf "${CYAN}  →${RESET}  %s\n" "$1"; }
success() { printf "${GREEN}  ✔${RESET}  %s\n" "$1"; }
warn()    { printf "${YELLOW}  ⚠${RESET}  %s\n" "$1"; }
die()     { printf "${RED}  ✖${RESET}  %s\n" "$1" >&2; exit 1; }
header()  { printf "\n${BOLD}%s${RESET}\n" "$1"; }

# ── Banner ────────────────────────────────────────────────────────────────────
printf "\n${BOLD}${CYAN}AI-CodeLint Installer${RESET}\n"
printf "${DIM}────────────────────────────────────${RESET}\n\n"

# ── Detect platform ───────────────────────────────────────────────────────────
header "Detecting platform..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *) die "Unsupported macOS architecture: $ARCH" ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) die "Unsupported Linux architecture: $ARCH" ;;
    esac
    ;;
  *)
    die "Unsupported OS: $OS"
    ;;
esac

info "Platform: $OS / $ARCH ($TARGET)"

# ── Fetch latest release tag ──────────────────────────────────────────────────
header "Fetching latest release..."

TAG=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
      | grep '"tag_name":' | cut -d '"' -f 4)

[ -z "$TAG" ] && die "Could not fetch latest release tag. Check your internet connection."

info "Latest version: $TAG"

# ── Download ──────────────────────────────────────────────────────────────────
header "Downloading binary..."

FILE="${APP_NAME}-${TAG}-${TARGET}"
URL="https://github.com/$REPO/releases/download/$TAG/$FILE"

TMP_FILE="$(mktemp)"
# -L follows redirects, --fail exits non-zero on HTTP errors
if ! curl -fL --progress-bar "$URL" -o "$TMP_FILE"; then
  die "Download failed. URL: $URL\nCheck that release '$TAG' has an asset for target '$TARGET'."
fi

# Sanity-check: a valid binary is never just a few bytes
FILESIZE=$(wc -c < "$TMP_FILE")
if [ "$FILESIZE" -lt 10240 ]; then
  die "Downloaded file is suspiciously small (${FILESIZE} bytes). The asset may not exist for this target."
fi

chmod +x "$TMP_FILE"
success "Downloaded $FILE ($(( FILESIZE / 1024 )) KB)"

# ── Install ───────────────────────────────────────────────────────────────────
header "Installing..."

# Prefer /usr/local/bin; fall back to ~/bin (no sudo needed)
if [ -w "/usr/local/bin" ]; then
  INSTALL_DIR="/usr/local/bin"
elif [ "$(id -u)" -eq 0 ]; then
  INSTALL_DIR="/usr/local/bin"
  mkdir -p "$INSTALL_DIR"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$TMP_FILE" "$INSTALL_DIR/$APP_NAME"
success "Installed to $INSTALL_DIR/$APP_NAME"

# ── PATH setup ────────────────────────────────────────────────────────────────
header "Configuring PATH..."

already_in_path() {
  case ":${PATH}:" in
    *":$INSTALL_DIR:"*) return 0 ;;
    *) return 1 ;;
  esac
}

add_to_shell_config() {
  LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
  ADDED=0

  for RC in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.profile"; do
    if [ -f "$RC" ]; then
      if ! grep -qF "$INSTALL_DIR" "$RC" 2>/dev/null; then
        printf "\n# Added by ai-codelint installer\n%s\n" "$LINE" >> "$RC"
        success "Added PATH entry to $RC"
        ADDED=1
      else
        info "PATH already present in $RC"
        ADDED=1
      fi
    fi
  done

  if [ "$ADDED" -eq 0 ]; then
    # No RC file found — create ~/.profile
    printf "\n# Added by ai-codelint installer\n%s\n" "$LINE" >> "$HOME/.profile"
    success "Created PATH entry in ~/.profile"
  fi
}

if already_in_path; then
  success "$INSTALL_DIR is already in PATH"
else
  add_to_shell_config
  warn "PATH updated — reload your shell or run:"
  printf "      ${CYAN}export PATH=\"$INSTALL_DIR:\$PATH\"${RESET}\n"
fi

# ── Done ──────────────────────────────────────────────────────────────────────
printf "\n${DIM}────────────────────────────────────${RESET}\n"
printf "${BOLD}${GREEN}  Installation complete!${RESET}\n\n"
printf "  Run ${CYAN}${APP_NAME} --help${RESET} to get started.\n"

if ! already_in_path; then
  printf "  ${YELLOW}Note:${RESET} Restart your terminal (or source your shell config) first.\n"
fi

printf "\n"
