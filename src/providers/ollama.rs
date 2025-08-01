use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::VecDeque;
use crate::providers::{Provider, Role, Message, Model};
use anyhow::{anyhow, Result};

/// Provider for Ollama local models
#[derive(Debug)]
pub struct OllamaProvider {
    base_url: String,
    models: Vec<Model>,
}

impl OllamaProvider {
    /// Creates a new OllamaProvider instance
    pub fn new(base_url: &str, models: Vec<Model>) -> Self {
        Self {
            base_url: base_url.to_string(),
            models,
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn models(&self) -> Vec<Model> {
        self.models.clone()
    }

    async fn generate_response(
        &self,
        model: &Model,
        history: &VecDeque<Message>,
        context: &str,
    ) -> Result<String> {
        let client = Client::new();
        let url = format!("{}/api/chat", self.base_url);

        // Prepare messages for the API request
        let mut messages = Vec::new();

        // Add system message with context
        messages.push(OllamaMessage {
            role: "system".to_string(),
            content: format!(
                "You are an expert coding assistant. Context:\n{}",
                context
            ),
        });

        // Add conversation history
        for message in history {
            let role = match message.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "user", // Treat tool messages as user messages
            };

            messages.push(OllamaMessage {
                role: role.to_string(),
                content: message.content.clone(),
            });
        }

        // Build request payload
        let payload = json!({
            "model": &model.id,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": 0.7,
                "num_predict": model.default_max_tokens
            }
        });

        // Send request to Ollama API
        let response = client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow!(
                "Ollama API error: {} - {}",
                status,
                body
            ));
        }

        // Parse response
        let response_body: OllamaResponse = response.json().await?;
        
        Ok(response_body.message.content)
    }
}

/// Ollama message format for API requests
#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

/// Ollama API response structure
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
    done: bool,
}