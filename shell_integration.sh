#!/usr/bin/env bash
# Shell integration for Fixit
# Add this to your ~/.bashrc or ~/.zshrc:
#   source /path/to/shell_integration.sh

# Directory to store temporary error output
export FIXIT_TMPDIR="${FIXIT_TMPDIR:-/tmp/fixit-$$}"
mkdir -p "$FIXIT_TMPDIR"

# Function to capture command output and errors
_fixit_preexec() {
    # This runs before each command
    local cmd="$1"

    # Skip capturing the 'fixit' command itself to avoid overwriting the previous command
    if [[ "$cmd" =~ ^fixit($|[[:space:]]) ]]; then
        return
    fi

    # Skip VS Code internal commands
    if [[ "$cmd" =~ ^__vsc_ ]] || [[ "$cmd" =~ ^_fixit_ ]]; then
        return
    fi

    # Skip empty commands
    if [[ -z "$cmd" ]]; then
        return
    fi

    export FIXIT_LAST_CMD="$cmd"
    export FIXIT_ERROR_FILE="$FIXIT_TMPDIR/last_error"
    # Clear previous error file
    : > "$FIXIT_ERROR_FILE"
}

_fixit_precmd() {
    # This runs after each command
    export FIXIT_LAST_EXIT=$?
}

# Bash-specific hooks
if [ -n "$BASH_VERSION" ]; then
    # For bash, we use DEBUG trap and PROMPT_COMMAND
    _fixit_bash_preexec() {
        local cmd="$BASH_COMMAND"

        # Skip if empty or internal command
        if [ -z "$cmd" ] || [ "$cmd" = "_fixit_precmd" ]; then
            return
        fi

        # Skip VS Code and other internal hooks
        if [[ "$cmd" =~ ^__vsc_ ]] || [[ "$cmd" =~ ^_fixit_ ]]; then
            return
        fi

        _fixit_preexec "$cmd"
    }

    trap '_fixit_bash_preexec' DEBUG

    if [[ "$PROMPT_COMMAND" != *"_fixit_precmd"* ]]; then
        PROMPT_COMMAND="_fixit_precmd${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
    fi
fi

# Zsh-specific hooks
if [ -n "$ZSH_VERSION" ]; then
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec _fixit_preexec
    add-zsh-hook precmd _fixit_precmd
fi

# Main 'fixit' command with full context
fixit() {
    local user_prompt="$*"
    local cmd_args=()

    # Pass the last command if available
    if [ -n "$FIXIT_LAST_CMD" ]; then
        cmd_args+=(--last-cmd "$FIXIT_LAST_CMD")
    fi

    # Pass the exit code if available and non-zero
    if [ -n "$FIXIT_LAST_EXIT" ] && [ "$FIXIT_LAST_EXIT" -ne 0 ]; then
        cmd_args+=(--last-exit "$FIXIT_LAST_EXIT")
    fi

    # Pass error output if available
    if [ -f "$FIXIT_ERROR_FILE" ] && [ -s "$FIXIT_ERROR_FILE" ]; then
        cmd_args+=(--stderr "$(cat "$FIXIT_ERROR_FILE")")
    fi

    # Call the actual CLI tool
    if [ -n "$user_prompt" ]; then
        fixit "${cmd_args[@]}" "$user_prompt"
    else
        fixit "${cmd_args[@]}"
    fi
}

echo "Fixit shell integration loaded!"
echo "Usage: fixit [optional context]"
echo "Flags: --explain (just explain), --improve (suggest improvements)"
