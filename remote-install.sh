#!/usr/bin/env bash
# Remote installer for Fixit
# Usage: curl -fsSL https://raw.githubusercontent.com/shaktiman101/cli-assistant/main/remote-install.sh | bash

set -e

REPO_URL="https://github.com/shaktiman101/cli-assistant"
REPO_NAME="cli-assistant"
INSTALL_DIR="$HOME/.local/fixit-source"
BINARY_DIR="$HOME/.local/bin"

echo "🚀 Fixit - Remote Installer"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check for required tools
if ! command -v git &> /dev/null; then
    echo "❌ Error: git is not installed"
    echo "   Please install git first: sudo apt install git"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is not installed"
    echo ""
    echo "Install Rust with:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

# Detect user's shell
# Priority: 1) Current shell environment 2) Default shell from $SHELL
SHELL_CONFIG=""
SHELL_NAME=""

# First, check if we're currently running in zsh or bash
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
    SHELL_NAME="zsh"
elif [ -n "$BASH_VERSION" ]; then
    # On macOS, use .bash_profile if .bashrc doesn't exist
    if [[ "$OSTYPE" == "darwin"* ]] && [[ ! -f "$HOME/.bashrc" ]]; then
        SHELL_CONFIG="$HOME/.bash_profile"
    else
        SHELL_CONFIG="$HOME/.bashrc"
    fi
    SHELL_NAME="bash"
else
    # Fallback: detect from $SHELL environment variable
    USER_SHELL=$(basename "$SHELL" 2>/dev/null || echo "bash")

    if [[ "$USER_SHELL" == "zsh" ]]; then
        SHELL_CONFIG="$HOME/.zshrc"
        SHELL_NAME="zsh"
    elif [[ "$USER_SHELL" == "bash" ]]; then
        if [[ "$OSTYPE" == "darwin"* ]] && [[ ! -f "$HOME/.bashrc" ]]; then
            SHELL_CONFIG="$HOME/.bash_profile"
        else
            SHELL_CONFIG="$HOME/.bashrc"
        fi
        SHELL_NAME="bash"
    else
        echo "⚠️  Could not detect shell. Defaulting to bash."
        SHELL_CONFIG="$HOME/.bashrc"
        SHELL_NAME="bash"
        echo "   If you use zsh, manually add to ~/.zshrc:"
        echo "   source ~/.local/fixit/shell_integration.sh"
    fi
fi

echo "📍 Detected shell: $SHELL_NAME"
echo "📝 Config file: $SHELL_CONFIG"
echo ""

# Clean up previous installation if exists
if [ -d "$INSTALL_DIR" ]; then
    echo "🧹 Removing previous installation..."
    rm -rf "$INSTALL_DIR"
fi

# Clone repository
echo "📥 Downloading CLI Assistant from GitHub..."
git clone --depth 1 "$REPO_URL" "$INSTALL_DIR" > /dev/null 2>&1

if [ ! -d "$INSTALL_DIR" ]; then
    echo "❌ Failed to download repository"
    exit 1
fi

cd "$INSTALL_DIR"

# Build in release mode
echo "🦀 Building Rust binary (this may take a few minutes)..."
cargo build --release --quiet

# Verify build succeeded
if [ ! -f "target/release/fixit" ]; then
    echo "❌ Build failed. Binary not found"
    exit 1
fi

echo "✓ Build successful!"
echo ""

# Install binary
mkdir -p "$BINARY_DIR"
echo "📦 Installing binary to $BINARY_DIR/fixit..."
cp target/release/fixit "$BINARY_DIR/"
chmod +x "$BINARY_DIR/fixit"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$BINARY_DIR:"* ]]; then
    echo "📝 Adding $BINARY_DIR to PATH..."
    echo "" >> "$SHELL_CONFIG"
    echo "# Add .local/bin to PATH for CLI tools" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_CONFIG"
fi

# Add shell integration
INTEGRATION_DIR="$HOME/.local/fixit"
mkdir -p "$INTEGRATION_DIR"
cp shell_integration.sh "$INTEGRATION_DIR/"

INTEGRATION_LINE="source \$HOME/.local/fixit/shell_integration.sh"

if grep -q "fixit/shell_integration.sh" "$SHELL_CONFIG" 2>/dev/null; then
    echo "✓ Shell integration already present"
else
    echo "📝 Adding shell integration..."
    echo "" >> "$SHELL_CONFIG"
    echo "# Fixit - Shell Integration" >> "$SHELL_CONFIG"
    echo "$INTEGRATION_LINE" >> "$SHELL_CONFIG"
fi

# Get binary size
BINARY_SIZE=$(du -h "$BINARY_DIR/fixit" | cut -f1)

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Installation complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Binary size: $BINARY_SIZE"
echo "📁 Installed to: $BINARY_DIR/fixit"
echo ""
echo "⚙️  Next steps:"
echo ""
echo "1️⃣  Set your OpenAI API key:"
echo "    export OPENAI_API_KEY='sk-...'"
echo ""
echo "2️⃣  Make it permanent (add to $SHELL_CONFIG):"
echo "    echo 'export OPENAI_API_KEY=\"sk-...\"' >> $SHELL_CONFIG"
echo ""
echo "3️⃣  Reload your shell:"
echo "    source $SHELL_CONFIG"
echo ""
echo "4️⃣  Try it out:"
echo "    ls /nonexistent"
echo "    fixit"
echo ""
echo "🎉 Happy fixing!"
echo ""
