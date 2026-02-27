use colored::*;

/// Display a section with a title and content
pub fn show_section(title: &str, content: &str, color: Color) {
    let separator = "─".repeat(60);

    println!();
    println!("{}", separator.color(color));
    println!("{}", title.bold().color(color));
    println!("{}", separator.color(color));
    println!("{}", content);
    println!();
}

/// Display the original command that failed
pub fn show_original_command(command: &str) {
    show_section("❌ Original Command", command, Color::Red);
}

/// Display the explanation from the LLM
pub fn show_explanation(explanation: &str) {
    show_section("ℹ️  Analysis", explanation, Color::Blue);
}

/// Display the suggested fix command
pub fn show_suggestion(command: &str) {
    show_section("✓ Suggested Fix", command, Color::Green);
}

/// Display all sections together
pub fn show_complete_suggestion(
    original_command: Option<&str>,
    explanation: &str,
    suggested_command: Option<&str>,
) {
    if let Some(cmd) = original_command {
        // Only show as "Original Command" if there's an error/fix mode
        if suggested_command.is_some() {
            show_original_command(cmd);
        } else {
            // For explain mode, show as "Command"
            show_section("📝 Command", cmd, Color::Cyan);
        }
    }

    show_explanation(explanation);

    if let Some(cmd) = suggested_command {
        show_suggestion(cmd);
    }
}

/// Display a success message
pub fn show_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}

/// Display an error message
pub fn show_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

/// Display a warning message
pub fn show_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

/// Display an info message
pub fn show_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_functions() {
        // These tests just ensure the functions don't panic
        // Visual output would need manual verification
        show_original_command("ls /nonexistent");
        show_explanation("The path does not exist");
        show_suggestion("ls /tmp");
        show_success("Command executed successfully");
        show_error("Command failed");
        show_warning("Warning message");
        show_info("Info message");
    }

    #[test]
    fn test_show_complete_suggestion() {
        show_complete_suggestion(
            Some("ls /bad"),
            "The path is invalid",
            "ls /tmp",
        );

        show_complete_suggestion(
            None,
            "Explanation only",
            "command",
        );
    }
}
