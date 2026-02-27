# Fixit 🔧

AI-powered command-line assistant that **fixes**, **explains**, and **improves** your shell commands using OpenAI's GPT models.

**Fast, intelligent, and easy to use** - built in Rust for maximum performance.

## Why Fixit?

- **⚡ Lightning fast** - ~50-100ms startup
- **📦 Single binary** - no dependencies
- **🚀 Tiny footprint** - ~3-5MB binary
- **🎯 Three modes** - Fix, Explain, or Improve
- **🧠 Smart detection** - Automatically understands your intent

## Features

### 🔧 **Fix Mode** - Fix Failed Commands
- Automatic error detection
- AI-powered error analysis
- Suggests corrected commands
- Safe execution with confirmation

### 📖 **Explain Mode** - Understand Commands
- Break down complex commands
- Explain what each part does
- Learn command-line patterns
- No execution - just learning

### ⚡ **Improve Mode** - Make Commands Better
- Suggest improvements
- Add useful flags
- Optimize performance
- Enhance safety

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- OpenAI API key

### One-Command Install ⚡ (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/shaktiman101/cli-assistant/main/remote-install.sh | bash
```

This will:
1. Download the repository
2. Build the binary in release mode
3. Install to `~/.local/bin/fixit`
4. Set up shell integration in your `.bashrc` or `.zshrc`

### Quick Install (from cloned repo)

```bash
# Clone the repository
git clone https://github.com/shaktiman101/cli-assistant.git
cd cli-assistant

# Run the installer
./install.sh
```

### Manual Installation

```bash
# Build the binary
cargo build --release

# Copy to a directory in your PATH
cp target/release/fixit ~/.local/bin/

# Add shell integration to your shell config
echo 'source ~/.local/fixit/shell_integration.sh' >> ~/.bashrc  # or ~/.zshrc
```

## Configuration

### Required

Set your OpenAI API key:

```bash
export OPENAI_API_KEY="sk-..."
```

Add to your shell config to make it permanent:

```bash
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.bashrc  # or ~/.zshrc
```

### Optional

Change the default model (default: `gpt-4o`):

```bash
export FIXIT_MODEL="gpt-3.5-turbo"
```

Adjust API timeout (default: 30 seconds):

```bash
export FIXIT_TIMEOUT="60"
```

## Usage

Fixit has **smart mode detection** that automatically understands whether you want to:
- 🔧 **Fix** a failed command
- 📖 **Explain** what a command does
- ⚡ **Improve** or modify a command

### Mode 1: Fix Failed Commands

Just run a command that fails, then type `fixit`:

```bash
$ ls /nonexistent
ls: cannot access '/nonexistent': No such file or directory

$ fixit
```

The assistant will:
1. Analyze the failed command and error
2. Provide an explanation of what went wrong
3. Suggest a corrected command
4. Ask for confirmation before executing

### Mode 2: Explain Commands

Understand what any command does:

```bash
$ find . -type f -mtime -7 -exec grep -l 'pattern' {} \;
$ fixit "what does this command do?"
```

Or:

```bash
$ tar -xzf archive.tar.gz
$ fixit "explain each flag"
```

Or use the explicit flag:

```bash
$ fixit --explain
```

The assistant will:
1. Break down the command into parts
2. Explain what each component does
3. Describe the overall purpose
4. **No execution** - just explanation

### Mode 3: Improve Commands

Make commands better or modify them:

```bash
$ ls -la
$ fixit "make this show human-readable file sizes"
```

Or:

```bash
$ grep -r "pattern" .
$ fixit "make it faster and ignore .git folders"
```

Or use the explicit flag:

```bash
$ fixit --improve "make it faster"
```

The assistant will:
1. Understand your request
2. Provide an improved version
3. Explain what changed and why
4. Ask for confirmation before executing

### Command-Line Options

```bash
# Show help
fixit --help

# Show version
fixit --version

# Force explain mode (just explain, don't execute)
fixit --explain

# Force improve mode (suggest improvements)
fixit --improve

# Skip confirmation (dangerous!)
fixit --no-confirm

# Manually specify command and error
fixit --last-cmd "ls /bad" --last-exit 2 --stderr "No such file"
```

## How It Works

1. **Shell Integration** captures:
   - The command you ran
   - Its exit code
   - Any error output (stderr)

2. **Context Gathering** collects:
   - Shell type (bash/zsh)
   - Operating system
   - Current working directory

3. **AI Analysis** sends everything to OpenAI's API with a structured prompt

4. **Smart Parsing** extracts:
   - Explanation of what went wrong
   - Corrected command to execute

5. **Safe Execution** asks for your confirmation before running anything

## Examples

### Example 1: Path doesn't exist

```bash
$ cd /nonexistant
bash: cd: /nonexistant: No such file or directory

$ fix
ℹ️  Initializing...
ℹ️  Using model: gpt-4o
ℹ️  Analyzing command...

────────────────────────────────────────────────────────────
❌ Original Command
────────────────────────────────────────────────────────────
cd /nonexistant

────────────────────────────────────────────────────────────
ℹ️  Analysis
────────────────────────────────────────────────────────────
The directory path has a typo: "nonexistant" should be "nonexistent"

────────────────────────────────────────────────────────────
✓ Suggested Fix
────────────────────────────────────────────────────────────
cd /nonexistent

Execute this command? [Y/n/edit]:
```

### Example 2: Missing flag

```bash
$ grep "pattern" file.txt
grep: file.txt: No such file or directory

$ fix "search recursively in current directory"
```

### Example 3: Permission denied

```bash
$ docker ps
permission denied while trying to connect to the Docker daemon socket

$ fix
```

## Performance Comparison

| Metric | Python Version | Rust Version |
|--------|---------------|--------------|
| Startup time | ~300ms | ~50-100ms |
| Binary size | ~50MB | ~3-5MB |
| Memory usage | ~50MB | ~5-10MB |
| Dependencies | Python + 5 packages | None (static binary) |
| Cold start | Noticeable lag | Nearly instant |

## Troubleshooting

### Command not found: fixit

Make sure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### API key error

Verify your API key is set:

```bash
echo $OPENAI_API_KEY
```

If empty, set it:

```bash
export OPENAI_API_KEY="sk-..."
```

### Shell integration not working

Reload your shell config:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### Build errors

Make sure you have the latest Rust toolchain:

```bash
rustup update
```

## Development

### Building from source

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized binary)
cargo build --release
```

### Running tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
cargo fmt --check
```

## Comparison with Python Version

| Feature | Python | Rust |
|---------|--------|------|
| Models | 100+ (via LiteLLM) | OpenAI only |
| Startup | Slow (~300ms) | Fast (~50-100ms) |
| Installation | pip install | Single binary |
| Dependencies | Many | None |
| Memory | ~50MB | ~5-10MB |
| Maintainability | Good | Excellent (type-safe) |
| Error handling | Good | Excellent (Result types) |
| Terminal UI | Rich (beautiful) | Colored (clean) |

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy`
6. Format code: `cargo fmt`
7. Submit a pull request

## License

MIT License - see LICENSE file for details

## Related Projects

- [Python version](../cli-assistant/) - Multi-model support with LiteLLM
- Original inspiration from various command-line AI assistants

## Support

For issues, questions, or suggestions:
- Open an issue on GitHub
- Check existing issues for solutions
- Read the troubleshooting section

---

Made with 🦀 Rust and ❤️ for the command line
