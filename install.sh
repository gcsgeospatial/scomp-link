#!/bin/bash
# Install script for scomp-link Rust binary

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
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

# Default installation directory
INSTALL_DIR="$HOME/.local/bin"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --system)
            INSTALL_DIR="/usr/local/bin"
            SYSTEM_INSTALL=true
            shift
            ;;
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --system          Install system-wide to /usr/local/bin (requires sudo)"
            echo "  --install-dir DIR Install to specific directory"
            echo "  --help            Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

print_info "Installing scomp-link Rust binary..."

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo (Rust) is not installed. Please install Rust first:"
    print_info "Visit https://rustup.rs/ or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if we need sudo for system installation
if [[ "$SYSTEM_INSTALL" == "true" ]]; then
    if [[ $EUID -ne 0 ]]; then
        print_warning "System installation requires sudo privileges"
        SUDO="sudo"
    fi
fi

# Create installation directory if it doesn't exist
if [[ "$SYSTEM_INSTALL" == "true" ]]; then
    $SUDO mkdir -p "$INSTALL_DIR"
else
    mkdir -p "$INSTALL_DIR"
fi

# Build the project
print_info "Building scomp-link (this may take a few minutes)..."
cargo build --release

# Check if build was successful
if [[ ! -f "target/release/scomp-link" ]]; then
    print_error "Build failed - binary not found"
    exit 1
fi

# Install the binary
print_info "Installing binary to $INSTALL_DIR..."
if [[ "$SYSTEM_INSTALL" == "true" ]]; then
    $SUDO cp target/release/scomp-link "$INSTALL_DIR/"
    $SUDO chmod +x "$INSTALL_DIR/scomp-link"
else
    cp target/release/scomp-link "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/scomp-link"
fi

# Check if installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    print_warning "Installation directory $INSTALL_DIR is not in your PATH"
    print_info "Add it to your PATH by adding this line to your shell profile:"
    print_info "export PATH=\"$INSTALL_DIR:\$PATH\""
    if [[ "$INSTALL_DIR" == "$HOME/.local/bin" ]]; then
        print_info "For bash: echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
        print_info "For zsh: echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
    fi
fi

# Verify installation
print_info "Verifying installation..."
if "$INSTALL_DIR/scomp-link" --version &> /dev/null; then
    print_success "Installation completed successfully!"
    print_info "You can now run: scomp-link --help"
    print_info "Version: $("$INSTALL_DIR/scomp-link" --version)"
else
    print_error "Installation verification failed"
    exit 1
fi

# Check for ImageMagick
if ! command -v magick &> /dev/null && ! command -v convert &> /dev/null; then
    print_warning "ImageMagick is not installed or not in PATH"
    print_info "scomp-link requires ImageMagick for image generation"
    print_info "Install it with:"
    print_info "  Ubuntu/Debian: sudo apt install imagemagick"
    print_info "  macOS: brew install imagemagick"
    print_info "  Windows: Download from https://imagemagick.org/script/download.php#windows"
fi

print_success "Setup complete! Run 'scomp-link --help' to get started."