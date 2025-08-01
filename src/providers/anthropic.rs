use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::providers::{Provider, Role, Message, Model};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct AnthropicProvider {
    base_url: String,
    api_key: String,
    extra_headers: Vec<(String, String)>,
    models: Vec<Model>,
}

impl AnthropicProvider {
    pub fn new(
        base_url: &str,
        api_key: &str,
        extra_headers: &[(String, String)],
        models: Vec<Model>,
    ) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            extra_headers: extra_headers.to_vec(),
            models,
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "Anthropic"
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
        let url = format!("{}/messages", self.base_url);

        // Prepare messages for the API request
        let mut messages = Vec::new();

        // Add context as a system prompt
        messages.push(AnthropicMessage {
            role: "user".to_string(),
            content: vec![AnthropicContent {
                content_type: "text".to_string(),
                text: format!(
                    "You are an expert coding assistant. Context:\n{}",
                    context
                ),
            }],
        });

        // Add conversation history
        for message in history {
            let role = match message.role {
                Role::System => "user", // Treat system messages as user messages
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "user", // Treat tool messages as user messages
            };

            messages.push(AnthropicMessage {
                role: role.to_string(),
                content: vec![AnthropicContent {
                    content_type: "text".to_string(),
                    text: message.content.clone(),
                }],
            });
        }

        // Build request payload
        let payload = AnthropicRequest {
            model: model.id.clone(),
            messages,
            max_tokens: model.default_max_tokens,
            system: "You are an expert coding assistant.".to_string(),
        };

        // Build request headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", self.api_key))?,
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            "anthropic-beta",
            header::HeaderValue::from_static("messages-2023-12-15"),
        );

        // Add extra headers
        for (key, value) in &self.extra_headers {
            headers.insert(
                header::HeaderName::from_bytes(key.as_bytes())?,
                header::HeaderValue::from_str(value)?,
            );
        }

        // Send request to Anthropic API
        let response = client
            .post(&url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow!(
                "Anthropic API error: {} - {}",
                status,
                body
            ));
        }

        // Parse response
        let response_body: AnthropicResponse = response.json().await?;

        if let Some(content) = response_body.content.first() {
            Ok(content.text.clone())
        } else {
            Err(anyhow!("No response content from Anthropic API"))
        }
    }
}

/// Anthropic message format for API requests
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    system: String,
    max_tokens: usize,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

/// Anthropic API response structure
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    text: String,
}
