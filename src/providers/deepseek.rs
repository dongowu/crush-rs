use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::VecDeque;
use crate::providers::{Provider, Role, Message, Model};
use anyhow::{anyhow, Result};

/// Provider for Deepseek Kimi2 model
#[derive(Debug)]
pub struct DeepseekProvider {
    base_url: String,
    api_key: String,
    models: Vec<Model>,
}

impl DeepseekProvider {
    /// Creates a new DeepseekProvider instance
    pub fn new(base_url: &str, api_key: &str, models: Vec<Model>) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            models,
        }
    }
}

#[async_trait]
impl Provider for DeepseekProvider {
    fn name(&self) -> &str {
        "Deepseek"
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
        let url = format!("{}/chat/completions", self.base_url);

        // Prepare messages for the API request
        let mut messages = Vec::new();

        // Add system message with context
        messages.push(DeepseekMessage {
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
                Role::Tool => "tool",
            };

            messages.push(DeepseekMessage {
                role: role.to_string(),
                content: message.content.clone(),
            });
        }

        // Build request payload
        let payload = json!({
            "model": &model.id,
            "messages": messages,
            "max_tokens": model.default_max_tokens,
            "temperature": 0.7,
            "stream": false
        });

        // Send request to Deepseek API
        let response = client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow!(
                "Deepseek API error: {} - {}",
                status,
                body
            ));
        }

        // Parse response
        let response_body: DeepseekResponse = response.json().await?;

        if let Some(choice) = response_body.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!("No response from Deepseek API"))
        }
    }
}

/// Deepseek message format for API requests
#[derive(Debug, Serialize, Deserialize)]
struct DeepseekMessage {
    role: String,
    content: String,
}

/// Deepseek API response structure
#[derive(Debug, Deserialize)]
struct DeepseekResponse {
    choices: Vec<DeepseekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepseekChoice {
    message: DeepseekMessage,
}
