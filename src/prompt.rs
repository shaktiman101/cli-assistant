use crate::shell::{CommandContext, ShellInfo};

/// Mode of operation for the assistant
#[derive(Debug, Clone, PartialEq)]
pub enum AssistantMode {
    /// Fix a failed command (has error)
    Fix,
    /// Explain what a command does
    Explain,
    /// Improve or modify a working command
    Improve,
}

/// Detect the mode based on context
pub fn detect_mode(context: &CommandContext) -> AssistantMode {
    // If there's an error (exit code != 0 or stderr present), it's Fix mode
    if context.exit_code.is_some() && context.exit_code != Some(0) {
        return AssistantMode::Fix;
    }

    if let Some(ref stderr) = context.stderr {
        if !stderr.trim().is_empty() {
            return AssistantMode::Fix;
        }
    }

    // If there's a user prompt, analyze it to determine intent
    if let Some(ref prompt) = context.user_prompt {
        let prompt_lower = prompt.to_lowercase();

        // Question words indicate Explain mode
        let question_indicators = [
            "what", "how", "why", "explain", "meaning", "mean", "does",
            "understand", "tell me", "show me", "describe", "purpose"
        ];

        if question_indicators.iter().any(|&word| prompt_lower.contains(word)) {
            return AssistantMode::Explain;
        }

        // Improvement words indicate Improve mode
        let improvement_indicators = [
            "make", "change", "modify", "improve", "better", "add",
            "instead", "alternative", "faster", "safer", "simpler"
        ];

        if improvement_indicators.iter().any(|&word| prompt_lower.contains(word)) {
            return AssistantMode::Improve;
        }
    }

    // Default: if command succeeded without user prompt, explain it
    AssistantMode::Explain
}

/// Create a structured prompt for the LLM
pub fn create_prompt(context: &CommandContext, shell_info: &ShellInfo, mode_override: Option<AssistantMode>) -> String {
    // Use override if provided, otherwise detect automatically
    let mode = mode_override.unwrap_or_else(|| detect_mode(context));

    // System prompt based on mode
    let system_prompt = match mode {
        AssistantMode::Fix => "You are a helpful command-line assistant. Your task is to analyze a failed command and provide a fixed version with explanation.",
        AssistantMode::Explain => "You are a helpful command-line assistant. Your task is to explain what a command does, breaking down its components and purpose.",
        AssistantMode::Improve => "You are a helpful command-line assistant. Your task is to improve or modify a command based on the user's request.",
    };

    let mut parts = vec![
        system_prompt.to_string(),
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

    // Add response format instructions based on mode
    match mode {
        AssistantMode::Fix => {
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
        }
        AssistantMode::Explain => {
            parts.extend(vec![
                String::new(),
                "## Your Response Format".to_string(),
                "Please provide your response in the following format:".to_string(),
                String::new(),
                "EXPLANATION:".to_string(),
                "[Detailed explanation of what the command does, breaking down each part]".to_string(),
                String::new(),
                "COMMAND:".to_string(),
                "[The same command, or 'N/A' if just explaining]".to_string(),
                String::new(),
                "Important:".to_string(),
                "- Focus on explaining clearly and comprehensively".to_string(),
                "- Break down complex commands into understandable parts".to_string(),
                "- If the command is the same, you can put 'N/A' in COMMAND section".to_string(),
            ]);
        }
        AssistantMode::Improve => {
            parts.extend(vec![
                String::new(),
                "## Your Response Format".to_string(),
                "Please provide your response in the following format:".to_string(),
                String::new(),
                "EXPLANATION:".to_string(),
                "[Explanation of the improvements made]".to_string(),
                String::new(),
                "COMMAND:".to_string(),
                "[The improved command]".to_string(),
                String::new(),
                "Important:".to_string(),
                "- Only output the EXPLANATION and COMMAND sections".to_string(),
                "- The COMMAND section should contain the improved command".to_string(),
                "- Explain what changes were made and why".to_string(),
                "- Make sure the command is safe to execute".to_string(),
            ]);
        }
    }

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

        let prompt = create_prompt(&context, &shell_info, None);

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

        let prompt = create_prompt(&context, &shell_info, None);

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

        let prompt = create_prompt(&context, &shell_info, None);

        assert!(prompt.contains("pwd"));
        assert!(!prompt.contains("Exit code"));
        assert!(!prompt.contains("## Error Output"));
        assert!(!prompt.contains("## User Request"));
    }

    #[test]
    fn test_mode_detection_fix() {
        let context = CommandContext {
            command: "ls".to_string(),
            exit_code: Some(2),
            stderr: Some("error".to_string()),
            user_prompt: None,
        };
        assert_eq!(detect_mode(&context), AssistantMode::Fix);
    }

    #[test]
    fn test_mode_detection_explain() {
        let context = CommandContext {
            command: "ls".to_string(),
            exit_code: None,
            stderr: None,
            user_prompt: Some("what does this do?".to_string()),
        };
        assert_eq!(detect_mode(&context), AssistantMode::Explain);
    }

    #[test]
    fn test_mode_detection_improve() {
        let context = CommandContext {
            command: "ls".to_string(),
            exit_code: None,
            stderr: None,
            user_prompt: Some("make it better".to_string()),
        };
        assert_eq!(detect_mode(&context), AssistantMode::Improve);
    }
}
