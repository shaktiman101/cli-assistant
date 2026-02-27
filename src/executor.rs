use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::process::Command;

use crate::display::{show_error, show_success, show_warning};

/// User's choice for command execution
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionChoice {
    Execute,
    Cancel,
    Edit(String),
}

/// Ask user to confirm command execution
pub fn confirm_execution(command: &str) -> Result<ExecutionChoice> {
    print!("{} ", "Execute this command? [Y/n/edit]:".yellow().bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();

    match response.as_str() {
        "" | "y" | "yes" => Ok(ExecutionChoice::Execute),
        "n" | "no" => Ok(ExecutionChoice::Cancel),
        "e" | "edit" => {
            // Allow user to edit the command
            print!("{} ", format!("Edit command [{}]:", command).yellow());
            io::stdout().flush()?;

            let mut edited = String::new();
            io::stdin().read_line(&mut edited)?;
            let edited = edited.trim();

            if edited.is_empty() {
                // If user just hit enter, use original command
                Ok(ExecutionChoice::Execute)
            } else {
                Ok(ExecutionChoice::Edit(edited.to_string()))
            }
        }
        _ => {
            show_warning("Invalid choice. Assuming 'no'.");
            Ok(ExecutionChoice::Cancel)
        }
    }
}

/// Execute a shell command
pub fn execute_command(command: &str) -> Result<bool> {
    // Check for cd command
    if command.trim().starts_with("cd ") {
        show_warning(
            "Note: 'cd' commands won't affect your current shell.\n\
             To change directory, copy the command and run it directly in your shell."
        );
    }

    // Get the user's shell
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    // Execute the command
    let output = Command::new(&shell)
        .arg("-c")
        .arg(command)
        .output()
        .context(format!("Failed to execute command with {}", shell))?;

    // Display output
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Check exit status
    if output.status.success() {
        show_success("Command executed successfully");
        Ok(true)
    } else {
        show_error(&format!(
            "Command failed with exit code: {}",
            output.status.code().unwrap_or(-1)
        ));
        Ok(false)
    }
}

/// Execute command with confirmation
pub fn execute_with_confirmation(command: &str, skip_confirm: bool) -> Result<()> {
    if skip_confirm {
        execute_command(command)?;
        return Ok(());
    }

    loop {
        match confirm_execution(command)? {
            ExecutionChoice::Execute => {
                execute_command(command)?;
                break;
            }
            ExecutionChoice::Cancel => {
                println!("{}", "Execution cancelled.".yellow());
                break;
            }
            ExecutionChoice::Edit(edited_command) => {
                // Recursively execute the edited command with confirmation
                execute_with_confirmation(&edited_command, false)?;
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_command() {
        // Test a simple command that should succeed
        let result = execute_command("echo 'test'");
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should return true for success
    }

    #[test]
    fn test_execute_failing_command() {
        // Test a command that should fail
        let result = execute_command("false");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for failure
    }

    #[test]
    fn test_cd_command_warning() {
        // This test just ensures cd commands don't panic
        let result = execute_command("cd /tmp");
        assert!(result.is_ok());
    }
}
