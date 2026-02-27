#!/usr/bin/env bash
# Remote installer for CLI Assistant
# Usage: curl -fsSL https://raw.githubusercontent.com/shaktiman101/cli-assistant/main/remote-install.sh | bash

set -e

REPO_URL="https://github.com/shaktiman101/cli-assistant"
REPO_NAME="cli-assistant"
INSTALL_DIR="$HOME/.local/cli-assistant-source"
BINARY_DIR="$HOME/.local/bin"

echo "🚀 CLI Assistant - Remote Installer"
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

# Detect shell
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
    SHELL_NAME="zsh"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
    SHELL_NAME="bash"
else
    # Try to detect from $SHELL environment variable
    if [[ "$SHELL" == *"zsh"* ]]; then
        SHELL_CONFIG="$HOME/.zshrc"
        SHELL_NAME="zsh"
    else
        SHELL_CONFIG="$HOME/.bashrc"
        SHELL_NAME="bash"
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
if [ ! -f "target/release/cli-assistant" ]; then
    echo "❌ Build failed. Binary not found"
    exit 1
fi

echo "✓ Build successful!"
echo ""

# Install binary
mkdir -p "$BINARY_DIR"
echo "📦 Installing binary to $BINARY_DIR/cli-assistant..."
cp target/release/cli-assistant "$BINARY_DIR/"
chmod +x "$BINARY_DIR/cli-assistant"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$BINARY_DIR:"* ]]; then
    echo "📝 Adding $BINARY_DIR to PATH..."
    echo "" >> "$SHELL_CONFIG"
    echo "# Add .local/bin to PATH for CLI tools" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_CONFIG"
fi

# Add shell integration
INTEGRATION_DIR="$HOME/.local/cli-assistant"
mkdir -p "$INTEGRATION_DIR"
cp shell_integration.sh "$INTEGRATION_DIR/"

INTEGRATION_LINE="source \$HOME/.local/cli-assistant/shell_integration.sh"

if grep -q "cli-assistant/shell_integration.sh" "$SHELL_CONFIG" 2>/dev/null; then
    echo "✓ Shell integration already present"
else
    echo "📝 Adding shell integration..."
    echo "" >> "$SHELL_CONFIG"
    echo "# CLI Assistant - Shell Integration" >> "$SHELL_CONFIG"
    echo "$INTEGRATION_LINE" >> "$SHELL_CONFIG"
fi

# Get binary size
BINARY_SIZE=$(du -h "$BINARY_DIR/cli-assistant" | cut -f1)

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Installation complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Binary size: $BINARY_SIZE"
echo "📁 Installed to: $BINARY_DIR/cli-assistant"
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
echo "    fix"
echo ""
echo "🎉 Happy fixing!"
echo ""
