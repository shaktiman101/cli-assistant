use anyhow::{Context, Result};
use std::env;

/// Configuration for the CLI assistant
#[derive(Debug, Clone)]
pub struct Config {
    /// OpenAI API key
    pub api_key: String,
    /// Model to use (default: gpt-4o)
    pub model: String,
    /// Timeout for API requests in seconds (default: 30)
    pub timeout: u64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable is not set.\n\nPlease set it:\n  export OPENAI_API_KEY='sk-...'")?;

        let model = env::var("CLI_ASSISTANT_MODEL")
            .unwrap_or_else(|_| "gpt-4o".to_string());

        let timeout = env::var("CLI_ASSISTANT_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30);

        Ok(Config {
            api_key,
            model,
            timeout,
        })
    }

    /// Get the API key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the model name
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get the timeout in seconds
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        env::set_var("OPENAI_API_KEY", "test-key");
        env::remove_var("CLI_ASSISTANT_MODEL");
        env::remove_var("CLI_ASSISTANT_TIMEOUT");

        let config = Config::from_env().unwrap();
        assert_eq!(config.model(), "gpt-4o");
        assert_eq!(config.timeout(), 30);

        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_config_custom_values() {
        env::set_var("OPENAI_API_KEY", "test-key");
        env::set_var("CLI_ASSISTANT_MODEL", "gpt-3.5-turbo");
        env::set_var("CLI_ASSISTANT_TIMEOUT", "60");

        let config = Config::from_env().unwrap();
        assert_eq!(config.model(), "gpt-3.5-turbo");
        assert_eq!(config.timeout(), 60);

        env::remove_var("OPENAI_API_KEY");
        env::remove_var("CLI_ASSISTANT_MODEL");
        env::remove_var("CLI_ASSISTANT_TIMEOUT");
    }

    #[test]
    fn test_config_missing_api_key() {
        env::remove_var("OPENAI_API_KEY");
        let result = Config::from_env();
        assert!(result.is_err());
    }
}
