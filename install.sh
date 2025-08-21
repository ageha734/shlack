#!/bin/bash

# shlack installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/ageha734/shlack/master/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="ageha734/shlack"
BINARY_NAME="shlack"
INSTALL_DIR="$HOME/.local/bin"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os arch

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *)          log_error "Unsupported operating system: $(uname -s)"; exit 1 ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        aarch64|arm64)  arch="aarch64" ;;
        *)              log_error "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac

    # Set platform-specific variables
    if [ "$os" = "windows" ]; then
        BINARY_NAME="${BINARY_NAME}.exe"
        ASSET_NAME="${BINARY_NAME%.*}-${os}-${arch}.exe"
    else
        ASSET_NAME="${BINARY_NAME}-${os}-${arch}"
    fi

    log_info "Detected platform: $os-$arch"
}

# Get latest release version
get_latest_version() {
    log_info "Fetching latest release information..."

    if command -v curl >/dev/null 2>&1; then
        LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        LATEST_VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi

    if [ -z "$LATEST_VERSION" ]; then
        log_error "Failed to get latest version"
        exit 1
    fi

    log_info "Latest version: $LATEST_VERSION"
}

# Download binary
download_binary() {
    local download_url="https://github.com/$REPO/releases/download/$LATEST_VERSION/$ASSET_NAME"
    local temp_file="/tmp/$ASSET_NAME"

    log_info "Downloading $ASSET_NAME..."
    log_info "Download URL: $download_url"

    if command -v curl >/dev/null 2>&1; then
        if ! curl -fsSL "$download_url" -o "$temp_file"; then
            log_error "Failed to download binary"
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -q "$download_url" -O "$temp_file"; then
            log_error "Failed to download binary"
            exit 1
        fi
    fi

    if [ ! -f "$temp_file" ]; then
        log_error "Downloaded file not found"
        exit 1
    fi

    log_success "Binary downloaded successfully"
}

# Install binary
install_binary() {
    local temp_file="/tmp/$ASSET_NAME"

    # Create install directory if it doesn't exist
    if [ ! -d "$INSTALL_DIR" ]; then
        log_info "Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi

    # Move binary to install directory
    log_info "Installing binary to $INSTALL_DIR/$BINARY_NAME"
    mv "$temp_file" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"

    log_success "Binary installed successfully"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_warning "Install directory $INSTALL_DIR is not in your PATH"
        log_info "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
        log_info "Or run the following command to add it temporarily:"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
    else
        log_success "Install directory is already in your PATH"
    fi
}

# Verify installation
verify_installation() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        log_success "Installation completed successfully!"
        log_info "You can now run: $BINARY_NAME"

        # Try to run the binary to show version
        if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
            log_info "Version information:"
            "$INSTALL_DIR/$BINARY_NAME" --help 2>/dev/null || true
        fi
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# Main installation process
main() {
    log_info "Starting shlack installation..."
    echo ""

    detect_platform
    get_latest_version
    download_binary
    install_binary
    check_path
    verify_installation

    echo ""
    log_success "Installation complete! 🎉"

    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        log_info "Don't forget to add $INSTALL_DIR to your PATH to use shlack from anywhere."
    fi
}

# Run main function
main "$@"
