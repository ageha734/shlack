#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

REPO="ageha734/shlack"
BINARY_NAME="shlack"

determine_install_dir() {
    local os_type
    os_type=$(uname -s 2>/dev/null || echo "Unknown")

    case "$os_type" in
        CYGWIN*|MINGW*|MSYS*)
            if [ -n "$USERPROFILE" ]; then
                echo "$(cygpath -u "$USERPROFILE" 2>/dev/null || echo "$HOME")/bin"
            elif [ -n "$HOME" ]; then
                echo "$HOME/bin"
            else
                echo "/usr/local/bin"
            fi
            ;;
        Darwin*)
            if [ -w "/usr/local/bin" ] 2>/dev/null || [ "$(id -u 2>/dev/null || echo 1000)" -eq 0 ]; then
                echo "/usr/local/bin"
            elif [ -w "/opt/homebrew/bin" ] 2>/dev/null; then
                echo "/opt/homebrew/bin"
            elif [ -n "$HOME" ]; then
                echo "$HOME/.local/bin"
            else
                echo "/usr/local/bin"
            fi
            ;;
        Linux*|*BSD*|SunOS*|AIX*)
            if [ -w "/usr/local/bin" ] 2>/dev/null || [ "$(id -u 2>/dev/null || echo 1000)" -eq 0 ]; then
                echo "/usr/local/bin"
            elif [ -n "$HOME" ]; then
                echo "$HOME/.local/bin"
            else
                echo "/usr/local/bin"
            fi
            ;;
        *)
            if [ -n "$HOME" ]; then
                echo "$HOME/.local/bin"
            else
                echo "/usr/local/bin"
            fi
            ;;
    esac
}

INSTALL_DIR=$(determine_install_dir)

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

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *)          log_error "Unsupported operating system: $(uname -s)"; exit 1 ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        aarch64|arm64)  arch="aarch64" ;;
        *)              log_error "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac

    if [ "$os" = "windows" ]; then
        ASSET_NAME="${BINARY_NAME%.*}-${os}-${arch}.exe"
    else
        ASSET_NAME="${BINARY_NAME}-${os}-${arch}"
    fi

    log_info "Detected platform: $os-$arch"
}

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

install_binary() {
    local temp_file="/tmp/$ASSET_NAME"
    local final_binary_name="$BINARY_NAME"
    local os_type
    os_type=$(uname -s 2>/dev/null || echo "Unknown")

    case "$os_type" in
        CYGWIN*|MINGW*|MSYS*)
            if ! echo "$final_binary_name" | grep -q "\.exe$"; then
                final_binary_name="${BINARY_NAME}.exe"
            fi
            ;;
    esac

    if [ ! -d "$INSTALL_DIR" ]; then
        log_info "Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR" || {
            log_error "Failed to create install directory: $INSTALL_DIR"
            exit 1
        }
    fi

    log_info "Installing binary to $INSTALL_DIR/$final_binary_name"

    if ! cp "$temp_file" "$INSTALL_DIR/$final_binary_name"; then
        log_error "Failed to install binary"
        exit 1
    fi

    rm -f "$temp_file" 2>/dev/null || true

    chmod +x "$INSTALL_DIR/$final_binary_name" 2>/dev/null || true

    log_success "Binary installed successfully"
}

detect_shell() {
    local shell_name

    if [ -n "$ZSH_VERSION" ]; then
        echo "zsh"
    elif [ -n "$BASH_VERSION" ]; then
        echo "bash"
    elif [ -n "$FISH_VERSION" ]; then
        echo "fish"
    elif [ -n "$0" ] && echo "$0" | grep -q "fish"; then
        echo "fish"
    elif [ -n "$SHELL" ]; then
        basename "$SHELL"
    else
        echo "sh"
    fi
}

get_shell_profile() {
    local shell_type="$1"
    local os_type
    os_type=$(uname -s 2>/dev/null || echo "Unknown")

    case "$shell_type" in
        zsh)
            if [ "$os_type" = "Darwin" ]; then
                echo "$HOME/.zshrc"
            else
                echo "$HOME/.zshrc"
            fi
            ;;
        bash)
            if [ "$os_type" = "Darwin" ]; then
                echo "$HOME/.bash_profile"
            else
                echo "$HOME/.bashrc"
            fi
            ;;
        fish)
            echo "$HOME/.config/fish/config.fish"
            ;;
        *)
            echo "$HOME/.profile"
            ;;
    esac
}

get_path_export_command() {
    local shell_type="$1"
    local install_dir="$2"

    case "$shell_type" in
        fish)
            echo "set -gx PATH \$PATH $install_dir"
            ;;
        *)
            echo "export PATH=\"\$PATH:$install_dir\""
            ;;
    esac
}

check_path() {
    local current_shell profile_file export_command

    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_warning "Install directory $INSTALL_DIR is not in your PATH"

        current_shell=$(detect_shell)
        profile_file=$(get_shell_profile "$current_shell")
        export_command=$(get_path_export_command "$current_shell" "$INSTALL_DIR")

        log_info "Detected shell: $current_shell"
        log_info "Add the following line to your shell profile ($profile_file):"
        echo "$export_command"
        echo ""

        log_info "Shell-specific instructions:"
        echo "  Bash:       echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.bashrc"
        echo "  Zsh:        echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.zshrc"
        echo "  Fish:       echo 'set -gx PATH \$PATH $INSTALL_DIR' >> ~/.config/fish/config.fish"
        echo "  PowerShell: Add '$INSTALL_DIR' to your PATH environment variable"
        echo ""

        log_info "Or run the following command to add it temporarily:"
        echo "$export_command"
    else
        log_success "Install directory is already in your PATH"
    fi
}

verify_installation() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        log_success "Installation completed successfully!"
        log_info "You can now run: $BINARY_NAME"

        if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
            log_info "Version information:"
            "$INSTALL_DIR/$BINARY_NAME" --help 2>/dev/null || true
        fi
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

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

main "$@"
