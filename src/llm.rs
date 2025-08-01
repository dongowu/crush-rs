use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::{Config, ProviderConfig, ApiType};

#[derive(Debug, Clone)]
pub struct LlmProvider {
    pub name: String,
    pub config: ProviderConfig,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl LlmProvider {
    pub async fn new(provider_name: &str, config: &Config) -> Result<Self> {
        let provider_config = config.get_provider(provider_name)
            .ok_or_else(|| anyhow!("Provider '{}' not found in configuration", provider_name))?;
        
        // Only check for API key if it's not Ollama (which can work without API key for local usage)
        if provider_name != "ollama" && provider_config.api_key.is_none() {
            return Err(anyhow!("API key not set for provider '{}'", provider_name));
        }
        
        // Validate base_url for specific providers
        if let Some(base_url) = &provider_config.base_url {
            if provider_name == "anthropic" && (base_url.contains("console.anthropic.com") || base_url.contains("claude.ai")) {
                return Err(anyhow!(
                    "Invalid base_url for Anthropic provider: '{}'\n\
                    This appears to be a web console URL, not an API endpoint.\n\
                    For Anthropic API, either:\n\
                    - Set base_url to null (recommended) to use default: https://api.anthropic.com/v1\n\
                    - Or set it to a valid API endpoint like: https://api.anthropic.com/v1", 
                    base_url
                ));
            }
        }
        
        Ok(Self {
            name: provider_name.to_string(),
            config: provider_config.clone(),
            client: Client::new(),
        })
    }
    
    pub async fn send_message(&self, messages: &[ChatMessage]) -> Result<ChatResponse> {
        match self.config.api_type {
            ApiType::OpenAI => self.send_openai_message(messages).await,
            ApiType::Anthropic => self.send_anthropic_message(messages).await,
            ApiType::Custom => self.send_openai_message(messages).await, // Default to OpenAI format
        }
    }
    
    async fn send_openai_message(&self, messages: &[ChatMessage]) -> Result<ChatResponse> {
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let model = self.config.model.as_deref().unwrap_or("gpt-4");
        
        let request_body = json!({
            "model": model,
            "messages": messages.iter().map(|m| json!({
                "role": m.role,
                "content": m.content
            })).collect::<Vec<_>>(),
            "temperature": 0.7,
            "max_tokens": 4000
        });
        
        let mut request_builder = self.client
            .post(&format!("{}/chat/completions", base_url))
            .header("Content-Type", "application/json");
        
        // Add Authorization header only if API key is present (for Ollama compatibility)
        if let Some(api_key) = &self.config.api_key {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request_builder
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            
            // Check for common error patterns
            let error_msg = if error_text.contains("<!DOCTYPE html>") {
                if error_text.contains("cloudflare") || error_text.contains("Just a moment") {
                    format!("Cloudflare protection detected. The API endpoint may be incorrect or blocked.\nStatus: {}\nPlease check your base_url configuration.", status)
                } else {
                    format!("Received HTML instead of JSON response. The API endpoint may be incorrect.\nStatus: {}", status)
                }
            } else if error_text.contains("unauthorized") || error_text.contains("invalid_api_key") {
                format!("Authentication failed. Please check your API key.\nStatus: {}", status)
            } else if error_text.contains("rate_limit") || error_text.contains("quota") {
                format!("Rate limit exceeded. Please wait and try again.\nStatus: {}", status)
            } else if status == 404 {
                format!("API endpoint not found. Please check your base_url configuration.\nStatus: {}", status)
            } else if status == 500 || status == 502 || status == 503 {
                format!("Server error. The API service may be temporarily unavailable.\nStatus: {}", status)
            } else {
                format!("API request failed.\nStatus: {}\nResponse: {}", status, 
                    if error_text.len() > 500 { 
                        format!("{}...", &error_text[..500])
                    } else { 
                        error_text 
                    }
                )
            };
            
            return Err(anyhow!("{}", error_msg));
        }
        
        let response_body: serde_json::Value = response.json().await?;
        
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .to_string();
        
        let usage = response_body.get("usage").and_then(|u| {
            Some(Usage {
                prompt_tokens: u["prompt_tokens"].as_u64()? as u32,
                completion_tokens: u["completion_tokens"].as_u64()? as u32,
                total_tokens: u["total_tokens"].as_u64()? as u32,
            })
        });
        
        Ok(ChatResponse { content, usage })
    }
    
    async fn send_anthropic_message(&self, messages: &[ChatMessage]) -> Result<ChatResponse> {
        let api_key = self.config.api_key.as_ref().unwrap();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.anthropic.com/v1");
        let model = self.config.model.as_deref().unwrap_or("claude-3-sonnet-20240229");
        
        // Convert messages to Anthropic format
        let system_message = messages.iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());
        
        let conversation_messages: Vec<_> = messages.iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({
                "role": m.role,
                "content": m.content
            }))
            .collect();
        
        let mut request_body = json!({
            "model": model,
            "messages": conversation_messages,
            "max_tokens": 4000
        });
        
        if let Some(system) = system_message {
            request_body["system"] = json!(system);
        }
        
        let response = self.client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            
            // Check for common error patterns
            let error_msg = if error_text.contains("<!DOCTYPE html>") {
                if error_text.contains("cloudflare") || error_text.contains("Just a moment") {
                    format!("Cloudflare protection detected. The API endpoint may be incorrect or blocked.\nStatus: {}\nPlease check your base_url configuration.", status)
                } else {
                    format!("Received HTML instead of JSON response. The API endpoint may be incorrect.\nStatus: {}", status)
                }
            } else if error_text.contains("unauthorized") || error_text.contains("invalid_api_key") {
                format!("Authentication failed. Please check your API key.\nStatus: {}", status)
            } else if error_text.contains("rate_limit") || error_text.contains("quota") {
                format!("Rate limit exceeded. Please wait and try again.\nStatus: {}", status)
            } else if status == 404 {
                format!("API endpoint not found. Please check your base_url configuration.\nStatus: {}", status)
            } else if status == 500 || status == 502 || status == 503 {
                format!("Server error. The API service may be temporarily unavailable.\nStatus: {}", status)
            } else {
                format!("API request failed.\nStatus: {}\nResponse: {}", status, 
                    if error_text.len() > 500 { 
                        format!("{}...", &error_text[..500])
                    } else { 
                        error_text 
                    }
                )
            };
            
            return Err(anyhow!("{}", error_msg));
        }
        
        let response_body: serde_json::Value = response.json().await?;
        
        let content = response_body["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .to_string();
        
        let usage = response_body.get("usage").and_then(|u| {
            Some(Usage {
                prompt_tokens: u["input_tokens"].as_u64()? as u32,
                completion_tokens: u["output_tokens"].as_u64()? as u32,
                total_tokens: (u["input_tokens"].as_u64()? + u["output_tokens"].as_u64()?) as u32,
            })
        });
        
        Ok(ChatResponse { content, usage })
    }
}

impl ChatMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn system(content: impl Into<String>) -> Self {
        Self::new("system", content)
    }
    
    pub fn user(content: impl Into<String>) -> Self {
        Self::new("user", content)
    }
    
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new("assistant", content)
    }
}