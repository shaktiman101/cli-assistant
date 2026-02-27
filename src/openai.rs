use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

/// OpenAI API client
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    model: String,
}

/// Message in the chat completion request
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Chat completion request payload
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

/// Choice in the chat completion response
#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiError>,
}

/// API error details
#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
}

impl OpenAIClient {
    /// Create a new OpenAI API client
    pub fn new(api_key: String, model: String, timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(OpenAIClient {
            client,
            api_key,
            model,
        })
    }

    /// Send a chat completion request to OpenAI API
    pub fn chat_completion(&self, prompt: &str) -> Result<String> {
        // Build the request payload
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: 1024,
            temperature: 0.7,
        };

        // Send the HTTP POST request
        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .context("Failed to send request to OpenAI API")?;

        // Check HTTP status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "OpenAI API request failed with status {}: {}",
                status,
                error_text
            );
        }

        // Parse the JSON response
        let chat_response: ChatResponse = response
            .json()
            .context("Failed to parse OpenAI API response")?;

        // Check for API errors
        if let Some(error) = chat_response.error {
            anyhow::bail!("OpenAI API error: {}", error.message);
        }

        // Extract the response content
        chat_response
            .choices
            .first()
            .and_then(|choice| Some(choice.message.content.clone()))
            .context("No response content in OpenAI API response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = OpenAIClient::new(
            "test-key".to_string(),
            "gpt-4o".to_string(),
            Duration::from_secs(30),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_chat_request_serialization() {
        let request = ChatRequest {
            model: "gpt-4o".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            max_tokens: 1024,
            temperature: 0.7,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4o"));
        assert!(json.contains("Hello"));
        assert!(json.contains("1024"));
    }
}
