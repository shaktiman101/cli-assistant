use anyhow::Result;

/// Parsed response from the LLM
#[derive(Debug, Clone)]
pub struct ParsedResponse {
    pub explanation: String,
    pub command: String,
    pub is_explanation_only: bool,
}

/// Parse the LLM response to extract EXPLANATION and COMMAND sections
pub fn parse_response(response: &str) -> Result<ParsedResponse> {
    let mut explanation_lines = Vec::new();
    let mut command_lines = Vec::new();
    let mut current_section: Option<String> = None;

    for line in response.lines() {
        let line_upper = line.trim().to_uppercase();

        if line_upper.starts_with("EXPLANATION:") {
            current_section = Some("explanation".to_string());
            // Get any text after "EXPLANATION:"
            let remainder = line.trim_start_matches("EXPLANATION:")
                .trim_start_matches("explanation:")
                .trim();
            if !remainder.is_empty() {
                explanation_lines.push(remainder.to_string());
            }
        } else if line_upper.starts_with("COMMAND:") {
            current_section = Some("command".to_string());
            // Get any text after "COMMAND:"
            let remainder = line.trim_start_matches("COMMAND:")
                .trim_start_matches("command:")
                .trim();
            if !remainder.is_empty() {
                command_lines.push(remainder.to_string());
            }
        } else if current_section.as_deref() == Some("explanation") {
            let trimmed = line.trim();
            if !trimmed.is_empty() || !explanation_lines.is_empty() {
                explanation_lines.push(line.to_string());
            }
        } else if current_section.as_deref() == Some("command") {
            command_lines.push(line.to_string());
        }
    }

    let mut explanation = explanation_lines.join("\n").trim().to_string();
    let mut command = command_lines.join("\n").trim().to_string();

    // Clean up command - remove markdown code blocks if present
    command = clean_markdown_code_blocks(&command);

    // Check if this is an explanation-only response
    let is_explanation_only = command.eq_ignore_ascii_case("n/a")
        || command.eq_ignore_ascii_case("none")
        || command.eq_ignore_ascii_case("same")
        || command.is_empty();

    // If parsing failed and not explanation-only, try to extract something useful
    if command.is_empty() && !is_explanation_only {
        command = extract_command_from_code_blocks(response);
    }

    // For explanation-only mode, command can be empty
    if command.is_empty() && !is_explanation_only {
        anyhow::bail!(
            "Could not extract command from response. Response:\n{}",
            response
        );
    }

    // If explanation is empty but we have a command, provide a default
    if explanation.is_empty() {
        explanation = "Command suggestion".to_string();
    }

    Ok(ParsedResponse {
        explanation,
        command,
        is_explanation_only,
    })
}

/// Remove markdown code blocks from command string
fn clean_markdown_code_blocks(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return String::new();
    }

    let mut result_lines = lines.clone();

    // Remove starting ```bash or ``` if present
    if result_lines.first().map_or(false, |line| line.trim().starts_with("```")) {
        result_lines.remove(0);
    }

    // Remove ending ``` if present
    if result_lines.last().map_or(false, |line| line.trim() == "```") {
        result_lines.pop();
    }

    result_lines.join("\n").trim().to_string()
}

/// Extract command from code blocks in the response
fn extract_command_from_code_blocks(response: &str) -> String {
    let parts: Vec<&str> = response.split("```").collect();

    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 1 {
            // Odd indices are inside code blocks
            let trimmed = part.trim();
            // Skip if it starts with bash/sh (those are markers)
            let content = if trimmed.starts_with("bash") || trimmed.starts_with("sh") {
                trimmed.lines().skip(1).collect::<Vec<_>>().join("\n")
            } else {
                trimmed.to_string()
            };

            if !content.is_empty() {
                return content.trim().to_string();
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response_basic() {
        let response = r#"
EXPLANATION:
The path does not exist.

COMMAND:
ls /tmp
"#;

        let parsed = parse_response(response).unwrap();
        assert_eq!(parsed.explanation.trim(), "The path does not exist.");
        assert_eq!(parsed.command.trim(), "ls /tmp");
    }

    #[test]
    fn test_parse_response_with_code_blocks() {
        let response = r#"
EXPLANATION:
Need to add recursive flag

COMMAND:
```bash
ls -R /tmp
```
"#;

        let parsed = parse_response(response).unwrap();
        assert!(parsed.explanation.contains("recursive"));
        assert_eq!(parsed.command.trim(), "ls -R /tmp");
    }

    #[test]
    fn test_parse_response_multiline_command() {
        let response = r#"
EXPLANATION:
Use a pipeline

COMMAND:
find . -name "*.txt" | \
    xargs grep "pattern"
"#;

        let parsed = parse_response(response).unwrap();
        assert!(parsed.command.contains("find"));
        assert!(parsed.command.contains("xargs"));
    }

    #[test]
    fn test_clean_markdown_code_blocks() {
        let text = "```bash\nls -la\n```";
        let cleaned = clean_markdown_code_blocks(text);
        assert_eq!(cleaned, "ls -la");
    }

    #[test]
    fn test_extract_from_code_blocks() {
        let response = "Some text\n```\nls -la\n```\nMore text";
        let command = extract_command_from_code_blocks(response);
        assert_eq!(command, "ls -la");
    }
}
