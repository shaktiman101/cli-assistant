#!/usr/bin/env bash
# Fixit Installer

set -e

echo "🔧 Installing Fixit..."
echo ""

# Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
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
    echo "❌ Unsupported shell. Please use bash or zsh."
    exit 1
fi

echo "📍 Detected shell: $SHELL_NAME"
echo "📝 Config file: $SHELL_CONFIG"
echo ""

# Build in release mode
echo "🦀 Building Rust binary (this may take a few minutes)..."
cargo build --release

# Verify build succeeded
if [ ! -f "target/release/fixit" ]; then
    echo "❌ Build failed. Binary not found at target/release/fixit"
    exit 1
fi

echo "✓ Build successful!"
echo ""

# Install binary
INSTALL_BIN_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_BIN_DIR"

echo "📦 Installing binary..."
cp target/release/fixit "$INSTALL_BIN_DIR/"
chmod +x "$INSTALL_BIN_DIR/fixit"
echo "✓ Installed to $INSTALL_BIN_DIR/fixit"
echo ""

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_BIN_DIR:"* ]]; then
    echo "⚠️  Warning: $INSTALL_BIN_DIR is not in your PATH"
    echo "   Add this line to $SHELL_CONFIG:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

# Add shell integration
INTEGRATION_LINE="source $HOME/.local/fixit/shell_integration.sh"
INTEGRATION_DIR="$HOME/.local/fixit"

mkdir -p "$INTEGRATION_DIR"
cp shell_integration.sh "$INTEGRATION_DIR/"

if grep -q "fixit/shell_integration.sh" "$SHELL_CONFIG" 2>/dev/null; then
    echo "✓ Shell integration already present in $SHELL_CONFIG"
else
    echo "" >> "$SHELL_CONFIG"
    echo "# Fixit" >> "$SHELL_CONFIG"
    echo "$INTEGRATION_LINE" >> "$SHELL_CONFIG"
    echo "✓ Added shell integration to $SHELL_CONFIG"
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "⚙️  Configuration required:"
echo ""
echo "Set your OpenAI API key:"
echo "  export OPENAI_API_KEY='sk-...'"
echo ""
echo "(Optional) Set a different model (default: gpt-4o):"
echo "  export FIXIT_MODEL='gpt-3.5-turbo'"
echo ""
echo "Add to $SHELL_CONFIG to make it permanent:"
echo "  echo 'export OPENAI_API_KEY=\"sk-...\"' >> $SHELL_CONFIG"
echo ""
echo "To start using:"
echo "  1. Reload your shell: source $SHELL_CONFIG"
echo "  2. Try it: ls /nonexistent"
echo "  3. Then run: fixit"
echo ""
echo "📊 Binary size: $(du -h "$INSTALL_BIN_DIR/fixit" | cut -f1)"
echo "🚀 Enjoy Fixit!"
echo ""
