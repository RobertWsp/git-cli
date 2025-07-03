#!/bin/bash

# Git CLI with Emojis - Uninstallation Script
# This script removes git-cli from the system

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

# Functions
print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════╗"
    echo "║                                                  ║"
    echo "║           Git CLI with Emojis v0.2.0             ║"
    echo "║             Uninstallation Script               ║"
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

confirm_uninstall() {
    echo -e "${YELLOW}This will remove git-cli and its configuration files.${NC}"
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_step "Uninstallation cancelled."
        exit 0
    fi
}

check_permissions() {
    # Check if running as root for system-wide uninstallation
    if [[ $EUID -ne 0 ]] && [[ "$INSTALL_DIR" == "/usr/bin" ]]; then
        print_error "This script requires sudo privileges to remove from /usr/bin"
        echo "Please run: sudo $0"
        exit 1
    fi
}

remove_binary() {
    print_step "Removing binary from $INSTALL_DIR..."
    
    if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        rm -f "$INSTALL_DIR/$BINARY_NAME"
        print_success "Binary removed from $INSTALL_DIR"
    else
        print_warning "Binary not found at $INSTALL_DIR/$BINARY_NAME"
    fi
}

remove_cargo_package() {
    print_step "Checking for cargo-installed package..."
    
    if command -v cargo &> /dev/null; then
        if cargo uninstall git-cli-emoji &>/dev/null; then
            print_success "Cargo package uninstalled"
        elif cargo uninstall git-cli &>/dev/null; then
            print_success "Cargo package uninstalled"
        else
            print_warning "No cargo package found"
        fi
    fi
}

remove_configuration() {
    print_step "Removing configuration files..."
    
    # Ask about configuration removal
    echo -e "${YELLOW}Do you want to remove configuration files?${NC}"
    echo "  - $CONFIG_DIR/"
    echo "  - $HOME/emojis.json"
    read -p "Remove config files? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Remove config directory
        if [[ -d "$CONFIG_DIR" ]]; then
            rm -rf "$CONFIG_DIR"
            print_success "Configuration directory removed"
        fi
        
        # Remove emoji file
        if [[ -f "$HOME/emojis.json" ]]; then
            rm -f "$HOME/emojis.json"
            print_success "Emoji configuration removed"
        fi
    else
        print_step "Configuration files preserved"
    fi
}

remove_aliases() {
    print_step "Removing shell aliases..."
    
    # Remove aliases from common shell config files
    for shell_config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [[ -f "$shell_config" ]]; then
            # Create backup
            cp "$shell_config" "$shell_config.backup.$(date +%Y%m%d_%H%M%S)"
            
            # Remove alias lines
            sed -i '/# Git CLI with Emojis alias/d' "$shell_config"
            sed -i '/alias gcli=/d' "$shell_config"
            
            print_success "Aliases removed from $shell_config"
        fi
    done
}

verify_removal() {
    print_step "Verifying removal..."
    
    local errors=0
    
    # Check if binary still exists
    if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        print_error "Binary still exists at $INSTALL_DIR/$BINARY_NAME"
        errors=$((errors + 1))
    fi
    
    # Check if command is still available
    if command -v "$BINARY_NAME" &> /dev/null; then
        print_warning "Command '$BINARY_NAME' is still available in PATH"
        print_warning "You may need to restart your shell or check other installation locations"
    fi
    
    if [[ $errors -eq 0 ]]; then
        print_success "Uninstallation completed successfully"
    else
        print_error "Some files could not be removed. Please check manually."
        exit 1
    fi
}

show_completion() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              Uninstallation Complete!           ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}What was removed:${NC}"
    echo "  ✓ Binary from $INSTALL_DIR"
    echo "  ✓ Cargo package (if installed)"
    echo "  ✓ Shell aliases"
    echo ""
    echo -e "${YELLOW}Note:${NC} You may need to restart your shell for changes to take effect"
    echo ""
    echo -e "${BLUE}Thank you for using Git CLI with Emojis! 🙏${NC}"
}

main() {
    print_header
    
    # Check for help flag
    if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
        echo "Git CLI with Emojis - Uninstallation Script"
        echo ""
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --force           Skip confirmation prompts"
        echo "  --help, -h        Show this help message"
        echo ""
        echo "This script will:"
        echo "  1. Remove binary from /usr/bin/"
        echo "  2. Uninstall cargo package (if installed)"
        echo "  3. Remove configuration files (optional)"
        echo "  4. Remove shell aliases"
        exit 0
    fi
    
    # Skip confirmation if --force is specified
    if [[ "$1" != "--force" ]]; then
        confirm_uninstall
    fi
    
    check_permissions
    remove_binary
    remove_cargo_package
    remove_configuration
    remove_aliases
    verify_removal
    show_completion
    
    print_success "Git CLI with Emojis has been uninstalled! 👋"
}

# Handle interruption
trap 'print_error "Uninstallation interrupted"; exit 1' INT TERM

# Run main function
main "$@"
