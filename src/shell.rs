use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

/// Information about the shell environment
#[derive(Debug, Clone)]
pub struct ShellInfo {
    pub shell: String,
    pub os: String,
    pub cwd: String,
}

/// Context captured from the command
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub command: String,
    pub exit_code: Option<i32>,
    pub stderr: Option<String>,
    pub user_prompt: Option<String>,
}

impl ShellInfo {
    /// Get information about the current shell environment
    pub fn capture() -> Result<Self> {
        let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());

        let os = if cfg!(target_os = "linux") {
            "Linux".to_string()
        } else if cfg!(target_os = "macos") {
            "macOS".to_string()
        } else {
            "unknown".to_string()
        };

        let cwd = env::current_dir()
            .context("Failed to get current working directory")?
            .display()
            .to_string();

        Ok(ShellInfo { shell, os, cwd })
    }
}

impl CommandContext {
    /// Capture command context from arguments or history
    pub fn capture(
        last_cmd: Option<String>,
        last_exit: Option<i32>,
        stderr: Option<String>,
        user_prompt: Option<String>,
    ) -> Result<Self> {
        // Use provided command or fall back to history
        let command = match last_cmd {
            Some(cmd) => cmd,
            None => get_last_command_from_history()
                .context("No command provided and could not read from history")?,
        };

        Ok(CommandContext {
            command,
            exit_code: last_exit,
            stderr,
            user_prompt,
        })
    }
}

/// Detect and return the shell history file path
fn get_shell_history_file() -> Option<PathBuf> {
    let shell = env::var("SHELL").unwrap_or_default();

    // Check for zsh
    if shell.contains("zsh") {
        let zsh_history = dirs::home_dir()?.join(".zsh_history");
        if zsh_history.exists() {
            return Some(zsh_history);
        }
    }

    // Check for bash
    if shell.contains("bash") {
        let bash_history = dirs::home_dir()?.join(".bash_history");
        if bash_history.exists() {
            return Some(bash_history);
        }
    }

    // Try both if shell is unknown
    for path in [
        dirs::home_dir()?.join(".zsh_history"),
        dirs::home_dir()?.join(".bash_history"),
    ] {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Get the last command from shell history
fn get_last_command_from_history() -> Result<String> {
    let history_file = get_shell_history_file()
        .context("Could not find shell history file (.bash_history or .zsh_history)")?;

    let mut file = File::open(&history_file)
        .with_context(|| format!("Failed to open history file: {}", history_file.display()))?;

    // Get file size
    let file_size = file.metadata()?.len();

    // Seek to last 10KB (or start of file if smaller)
    let seek_pos = if file_size > 10240 {
        file_size - 10240
    } else {
        0
    };
    file.seek(SeekFrom::Start(seek_pos))?;

    // Read the tail of the file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Convert to string (lossy to handle encoding issues)
    let content = String::from_utf8_lossy(&buffer);

    // Process lines in reverse to find the last valid command
    for line in content.lines().rev() {
        let trimmed = line.trim();

        // Skip empty lines and commands starting with space (private)
        if trimmed.is_empty() || trimmed.starts_with(' ') {
            continue;
        }

        // Handle zsh extended history format: : timestamp:0;command
        let command = if trimmed.starts_with(':') && trimmed.contains(';') {
            trimmed.split_once(';')
                .map(|(_, cmd)| cmd)
                .unwrap_or(trimmed)
        } else {
            trimmed
        };

        // Skip the current 'fix' or 'cli-assistant' command
        if command.starts_with("fix") || command.starts_with("cli-assistant") {
            continue;
        }

        return Ok(command.to_string());
    }

    anyhow::bail!("No valid command found in history")
}

// Add dirs dependency for home_dir
// Note: This requires adding dirs = "5.0" to Cargo.toml

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_info_capture() {
        let info = ShellInfo::capture().unwrap();
        assert!(!info.shell.is_empty());
        assert!(!info.os.is_empty());
        assert!(!info.cwd.is_empty());
    }

    #[test]
    fn test_command_context_with_provided_command() {
        let context = CommandContext::capture(
            Some("ls -la".to_string()),
            Some(0),
            None,
            Some("test".to_string()),
        )
        .unwrap();

        assert_eq!(context.command, "ls -la");
        assert_eq!(context.exit_code, Some(0));
        assert_eq!(context.user_prompt, Some("test".to_string()));
    }
}
