use crate::shell::{CommandContext, ShellInfo};

/// Create a structured prompt for the LLM
pub fn create_prompt(context: &CommandContext, shell_info: &ShellInfo) -> String {
    let mut parts = vec![
        "You are a helpful command-line assistant. Your task is to explain a bash/sh type commands or analyze a failed or problematic command and provide a fixed version.".to_string(),
        String::new(),
        "## Context".to_string(),
        format!("Shell: {}", shell_info.shell),
        format!("OS: {}", shell_info.os),
        format!("Working directory: {}", shell_info.cwd),
        String::new(),
        "## Command".to_string(),
        format!("```bash\n{}\n```", context.command),
    ];

    // Add exit code if present
    if let Some(exit_code) = context.exit_code {
        parts.push(String::new());
        parts.push(format!("Exit code: {}", exit_code));
    }

    // Add error output if present
    if let Some(ref stderr) = context.stderr {
        if !stderr.trim().is_empty() {
            parts.push(String::new());
            parts.push("## Error Output".to_string());
            parts.push(format!("```\n{}\n```", stderr));
        }
    }

    // Add user request if present
    if let Some(ref user_prompt) = context.user_prompt {
        if !user_prompt.trim().is_empty() {
            parts.push(String::new());
            parts.push("## User Request".to_string());
            parts.push(user_prompt.clone());
        }
    }

    // Add response format instructions
    parts.extend(vec![
        String::new(),
        "## Your Response Format".to_string(),
        "Please provide your response in the following format:".to_string(),
        String::new(),
        "EXPLANATION:".to_string(),
        "[Brief explanation of what went wrong and how to fix it]".to_string(),
        String::new(),
        "COMMAND:".to_string(),
        "[The corrected command to execute]".to_string(),
        String::new(),
        "Important:".to_string(),
        "- Only output the EXPLANATION and COMMAND sections".to_string(),
        "- The COMMAND section should contain ONLY the command to execute, nothing else".to_string(),
        "- Keep the explanation concise and focused".to_string(),
        "- Make sure the command is safe to execute".to_string(),
    ]);

    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_prompt_basic() {
        let context = CommandContext {
            command: "ls /nonexistent".to_string(),
            exit_code: Some(2),
            stderr: Some("No such file or directory".to_string()),
            user_prompt: None,
        };

        let shell_info = ShellInfo {
            shell: "/bin/bash".to_string(),
            os: "Linux".to_string(),
            cwd: "/home/user".to_string(),
        };

        let prompt = create_prompt(&context, &shell_info);

        assert!(prompt.contains("ls /nonexistent"));
        assert!(prompt.contains("Exit code: 2"));
        assert!(prompt.contains("No such file or directory"));
        assert!(prompt.contains("EXPLANATION:"));
        assert!(prompt.contains("COMMAND:"));
    }

    #[test]
    fn test_create_prompt_with_user_request() {
        let context = CommandContext {
            command: "ls /tmp".to_string(),
            exit_code: None,
            stderr: None,
            user_prompt: Some("make it recursive".to_string()),
        };

        let shell_info = ShellInfo {
            shell: "/bin/zsh".to_string(),
            os: "macOS".to_string(),
            cwd: "/Users/user".to_string(),
        };

        let prompt = create_prompt(&context, &shell_info);

        assert!(prompt.contains("ls /tmp"));
        assert!(prompt.contains("make it recursive"));
        assert!(prompt.contains("## User Request"));
    }

    #[test]
    fn test_create_prompt_minimal() {
        let context = CommandContext {
            command: "pwd".to_string(),
            exit_code: None,
            stderr: None,
            user_prompt: None,
        };

        let shell_info = ShellInfo {
            shell: "/bin/bash".to_string(),
            os: "Linux".to_string(),
            cwd: "/home/user".to_string(),
        };

        let prompt = create_prompt(&context, &shell_info);

        assert!(prompt.contains("pwd"));
        assert!(!prompt.contains("Exit code"));
        assert!(!prompt.contains("## Error Output"));
        assert!(!prompt.contains("## User Request"));
    }
}
