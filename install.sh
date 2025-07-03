#!/bin/bash

# Git CLI with Emojis - Installation Script
# This script compiles and installs git-cli to /usr/bin/

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_NAME="git-cli"
INSTALL_DIR="/usr/bin"
CONFIG_DIR="$HOME/.config/git-cli"
REPO_URL="https://github.com/yourusername/git-cli.git"

# Functions
print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════╗"
    echo "║                                                  ║"
    echo "║           Git CLI with Emojis v0.2.0             ║"
    echo "║              Installation Script                 ║"
    echo "║                                                  ║"
    echo "╚══════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_step() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    print_step "Checking prerequisites..."
    
    # Check if running as root for system-wide installation
    if [[ $EUID -ne 0 ]] && [[ "$INSTALL_DIR" == "/usr/bin" ]]; then
        print_error "This script requires sudo privileges to install to /usr/bin"
        echo "Please run: sudo $0"
        exit 1
    fi
    
    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Check if Cargo is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust toolchain."
        exit 1
    fi
    
    # Check if Git is installed
    if ! command -v git &> /dev/null; then
        print_error "Git is not installed. Please install Git first."
        exit 1
    fi
    
    # Check Rust version
    rust_version=$(rustc --version | awk '{print $2}')
    print_success "Rust version: $rust_version"
    
    # Check Git version
    git_version=$(git --version | awk '{print $3}')
    print_success "Git version: $git_version"
}

install_from_source() {
    print_step "Installing from source..."
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    # Clone repository
    if [[ -d "/var/www/html/robert/git-cli" ]]; then
        print_step "Using local repository..."
        cp -r "/var/www/html/robert/git-cli" .
        cd git-cli
    else
        print_step "Cloning repository..."
        git clone "$REPO_URL" git-cli
        cd git-cli
    fi
    
    # Build in release mode
    print_step "Building git-cli in release mode..."
    cargo build --release --quiet
    
    if [[ ! -f "target/release/$BINARY_NAME" ]]; then
        print_error "Build failed - binary not found"
        exit 1
    fi
    
    # Install binary
    print_step "Installing binary to $INSTALL_DIR..."
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    # Cleanup
    cd /
    rm -rf "$TEMP_DIR"
    
    print_success "Binary installed successfully!"
}

install_from_cargo() {
    print_step "Installing from cargo..."
    
    # Try to install from crates.io (if published)
    if cargo install git-cli-emoji --quiet 2>/dev/null; then
        print_success "Installed from crates.io!"
        return 0
    else
        print_warning "Package not found on crates.io, falling back to source installation"
        return 1
    fi
}

setup_configuration() {
    print_step "Setting up configuration..."
    
    # Create config directory
    mkdir -p "$CONFIG_DIR"
    
    # Create default config if it doesn't exist
    if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
        cat > "$CONFIG_DIR/config.toml" << 'EOF'
[general]
default_emoji = "✨"
auto_push = false
confirm_before_push = true
debug = false

[commit]
enforce_conventional = true
max_title_length = 50
max_body_length = 72
auto_capitalize_title = true

[hooks]
run_pre_commit = true
auto_fix_lint = true
retry_on_failure = true
EOF
        print_success "Created default configuration at $CONFIG_DIR/config.toml"
    else
        print_warning "Configuration already exists at $CONFIG_DIR/config.toml"
    fi
    
    # Set proper permissions
    chmod 755 "$CONFIG_DIR"
    chmod 644 "$CONFIG_DIR/config.toml"
}

setup_emojis() {
    print_step "Setting up emoji configuration..."
    
    # Create custom emojis file if it doesn't exist
    if [[ ! -f "$HOME/emojis.json" ]]; then
        cat > "$HOME/emojis.json" << 'EOF'
{
  "emojis": [
    {
      "emoji": "✨",
      "entity": "&#x2728;",
      "code": ":sparkles:",
      "description": "Introduce new features",
      "name": "sparkles"
    },
    {
      "emoji": "🐛",
      "entity": "&#x1f41b;",
      "code": ":bug:",
      "description": "Fix a bug",
      "name": "bug"
    },
    {
      "emoji": "🔧",
      "entity": "&#x1f527;",
      "code": ":wrench:",
      "description": "Add or update configuration files",
      "name": "wrench"
    },
    {
      "emoji": "📝",
      "entity": "&#x1f4dd;",
      "code": ":memo:",
      "description": "Add or update documentation",
      "name": "memo"
    },
    {
      "emoji": "🎨",
      "entity": "&#x1f3a8;",
      "code": ":art:",
      "description": "Improve structure / format of the code",
      "name": "art"
    },
    {
      "emoji": "⚡️",
      "entity": "&#x26a1;",
      "code": ":zap:",
      "description": "Improve performance",
      "name": "zap"
    },
    {
      "emoji": "💄",
      "entity": "&#ff99cc;",
      "code": ":lipstick:",
      "description": "Add or update the UI and style files",
      "name": "lipstick"
    },
    {
      "emoji": "✅",
      "entity": "&#x2705;",
      "code": ":white_check_mark:",
      "description": "Add, update, or pass tests",
      "name": "white-check-mark"
    },
    {
      "emoji": "🔒️",
      "entity": "&#x1f512;",
      "code": ":lock:",
      "description": "Fix security issues",
      "name": "lock"
    },
    {
      "emoji": "♻️",
      "entity": "&#x267e;",
      "code": ":recycle:",
      "description": "Refactor code",
      "name": "recycle"
    },
    {
      "emoji": "🚀",
      "entity": "&#1F680;",
      "code": ":rocket:",
      "description": "Deploy stuff",
      "name": "rocket"
    },
    {
      "emoji": "🐳",
      "entity": "&#1f433;",
      "code": ":whale:",
      "description": "Work about Docker",
      "name": "whale"
    }
  ]
}
EOF
        print_success "Created emoji configuration at $HOME/emojis.json"
    else
        print_warning "Emoji configuration already exists at $HOME/emojis.json"
    fi
}

create_alias() {
    print_step "Setting up shell alias..."
    
    # Add alias to common shell config files
    for shell_config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [[ -f "$shell_config" ]]; then
            if ! grep -q "alias gcli=" "$shell_config"; then
                echo "" >> "$shell_config"
                echo "# Git CLI with Emojis alias" >> "$shell_config"
                echo "alias gcli='git-cli'" >> "$shell_config"
                print_success "Added alias 'gcli' to $shell_config"
            fi
        fi
    done
}

verify_installation() {
    print_step "Verifying installation..."
    
    # Check if binary exists and is executable
    if [[ -x "$INSTALL_DIR/$BINARY_NAME" ]]; then
        print_success "Binary found at $INSTALL_DIR/$BINARY_NAME"
    else
        print_error "Binary not found or not executable"
        exit 1
    fi
    
    # Test the binary
    if "$INSTALL_DIR/$BINARY_NAME" --version &>/dev/null; then
        version=$("$INSTALL_DIR/$BINARY_NAME" --version)
        print_success "Installation verified: $version"
    else
        print_error "Binary test failed"
        exit 1
    fi
}

show_usage() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║               Installation Complete!             ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Usage:${NC}"
    echo "  git-cli                    # Interactive mode"
    echo "  git-cli --help             # Show help"
    echo "  git-cli --debug            # Debug mode"
    echo "  gcli                       # Short alias"
    echo ""
    echo -e "${BLUE}Configuration:${NC}"
    echo "  Config file: $CONFIG_DIR/config.toml"
    echo "  Emoji file:  $HOME/emojis.json"
    echo ""
    echo -e "${BLUE}Examples:${NC}"
    echo "  git-cli --emoji '✨' --title 'Add new feature'"
    echo "  git-cli --no-interactive --emoji '🐛' --title 'Fix bug'"
    echo ""
    echo -e "${YELLOW}Tip:${NC} Restart your shell or run 'source ~/.bashrc' to use the 'gcli' alias"
}

main() {
    print_header
    
    # Check for help flag
    if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
        echo "Git CLI with Emojis - Installation Script"
        echo ""
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --local-only      Skip cargo installation attempt"
        echo "  --help, -h        Show this help message"
        echo ""
        echo "This script will:"
        echo "  1. Check prerequisites (Rust, Cargo, Git)"
        echo "  2. Try to install from cargo (if available)"
        echo "  3. Fall back to source compilation"
        echo "  4. Install binary to /usr/bin/"
        echo "  5. Set up configuration files"
        echo "  6. Create shell aliases"
        exit 0
    fi
    
    check_prerequisites
    
    # Try cargo installation first (unless --local-only is specified)
    if [[ "$1" != "--local-only" ]]; then
        if ! install_from_cargo; then
            install_from_source
        fi
    else
        install_from_source
    fi
    
    setup_configuration
    setup_emojis
    create_alias
    verify_installation
    show_usage
    
    print_success "Git CLI with Emojis has been successfully installed! 🎉"
}

# Handle interruption
trap 'print_error "Installation interrupted"; exit 1' INT TERM

# Run main function
main "$@"
