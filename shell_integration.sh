#!/usr/bin/env bash
# Shell integration for CLI Assistant
# Add this to your ~/.bashrc or ~/.zshrc:
#   source /path/to/shell_integration.sh

# Directory to store temporary error output
export CLI_ASSISTANT_TMPDIR="${CLI_ASSISTANT_TMPDIR:-/tmp/cli-assistant-$$}"
mkdir -p "$CLI_ASSISTANT_TMPDIR"

# Function to capture command output and errors
_cli_assistant_preexec() {
    # This runs before each command
    local cmd="$1"

    # Skip capturing the 'fix' command itself to avoid overwriting the previous command
    if [[ "$cmd" =~ ^fix($|[[:space:]]) ]] || [[ "$cmd" =~ ^cli-assistant($|[[:space:]]) ]]; then
        return
    fi

    # Skip VS Code internal commands
    if [[ "$cmd" =~ ^__vsc_ ]] || [[ "$cmd" =~ ^_cli_assistant_ ]]; then
        return
    fi

    # Skip empty commands
    if [[ -z "$cmd" ]]; then
        return
    fi

    export CLI_ASSISTANT_LAST_CMD="$cmd"
    export CLI_ASSISTANT_ERROR_FILE="$CLI_ASSISTANT_TMPDIR/last_error"
    # Clear previous error file
    : > "$CLI_ASSISTANT_ERROR_FILE"
}

_cli_assistant_precmd() {
    # This runs after each command
    export CLI_ASSISTANT_LAST_EXIT=$?
}

# Bash-specific hooks
if [ -n "$BASH_VERSION" ]; then
    # For bash, we use DEBUG trap and PROMPT_COMMAND
    _cli_assistant_bash_preexec() {
        local cmd="$BASH_COMMAND"

        # Skip if empty or internal command
        if [ -z "$cmd" ] || [ "$cmd" = "_cli_assistant_precmd" ]; then
            return
        fi

        # Skip VS Code and other internal hooks
        if [[ "$cmd" =~ ^__vsc_ ]] || [[ "$cmd" =~ ^_cli_assistant_ ]]; then
            return
        fi

        _cli_assistant_preexec "$cmd"
    }

    trap '_cli_assistant_bash_preexec' DEBUG

    if [[ "$PROMPT_COMMAND" != *"_cli_assistant_precmd"* ]]; then
        PROMPT_COMMAND="_cli_assistant_precmd${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
    fi
fi

# Zsh-specific hooks
if [ -n "$ZSH_VERSION" ]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec _cli_assistant_preexec
    add-zsh-hook precmd _cli_assistant_precmd
fi

# Main 'fix' command with full context
fix() {
    local user_prompt="$*"
    local cmd_args=()

    # Pass the last command if available
    if [ -n "$CLI_ASSISTANT_LAST_CMD" ]; then
        cmd_args+=(--last-cmd "$CLI_ASSISTANT_LAST_CMD")
    fi

    # Pass the exit code if available and non-zero
    if [ -n "$CLI_ASSISTANT_LAST_EXIT" ] && [ "$CLI_ASSISTANT_LAST_EXIT" -ne 0 ]; then
        cmd_args+=(--last-exit "$CLI_ASSISTANT_LAST_EXIT")
    fi

    # Pass error output if available
    if [ -f "$CLI_ASSISTANT_ERROR_FILE" ] && [ -s "$CLI_ASSISTANT_ERROR_FILE" ]; then
        cmd_args+=(--stderr "$(cat "$CLI_ASSISTANT_ERROR_FILE")")
    fi

    # Call the actual CLI tool
    if [ -n "$user_prompt" ]; then
        cli-assistant "${cmd_args[@]}" "$user_prompt"
    else
        cli-assistant "${cmd_args[@]}"
    fi
}

# Alternative: Simple fix command without hooks (fallback)
# This version just reads from history, useful if hooks don't work
fix_simple() {
    cli-assistant "$@"
}

echo "CLI Assistant shell integration loaded!"
echo "Usage: fix [optional context]"
