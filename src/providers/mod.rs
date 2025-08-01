pub mod openai;
pub mod anthropic;
pub mod gemini;
pub mod deepseek;
pub mod kimi;
pub mod ollama;

use async_trait::async_trait;
use crate::config::ModelConfig;
use std::collections::VecDeque;

/// Represents an LLM model
#[derive(Clone, Debug)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub context_window: usize,
    pub default_max_tokens: usize,
    pub cost_per_1m_in: f32,
    pub cost_per_1m_out: f32,
    pub cost_per_1m_in_cached: Option<f32>,
    pub cost_per_1m_out_cached: Option<f32>,
    pub can_reason: bool,
    pub supports_attachments: bool,
}

impl From<&ModelConfig> for Model {
    fn from(config: &ModelConfig) -> Self {
        Model {
            id: config.id.clone(),
            name: config.name.clone(),
            context_window: config.context_window,
            default_max_tokens: config.default_max_tokens,
            cost_per_1m_in: config.cost_per_1m_in,
            cost_per_1m_out: config.cost_per_1m_out,
            cost_per_1m_in_cached: config.cost_per_1m_in_cached,
            cost_per_1m_out_cached: config.cost_per_1m_out_cached,
            can_reason: config.can_reason,
            supports_attachments: config.supports_attachments,
        }
    }
}

/// Trait for interacting with LLM providers
#[async_trait]
pub trait Provider {
    /// Returns the name of the provider
    fn name(&self) -> &str;

    /// Returns the available models
    fn models(&self) -> Vec<Model>;

    /// Generates a response from the LLM
    async fn generate_response(
        &self,
        model: &Model,
        history: &VecDeque<Message>,
        context: &str,
    ) -> anyhow::Result<String>;
}

/// Message role for conversation history
#[derive(Debug, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Message in conversation history
pub struct Message {
    pub role: Role,
    pub content: String,
}
