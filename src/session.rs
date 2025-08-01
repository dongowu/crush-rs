use crate::config::{CrushConfig, ProviderConfig};
use crate::lsp::LspClient;
use crate::mcp::McpClient;
use crate::providers::{Model, Provider, Role, Message as ProviderMessage, openai::OpenAiProvider, anthropic::AnthropicProvider, deepseek::DeepseekProvider, gemini::GeminiProvider, kimi::KimiProvider, ollama::OllamaProvider};
use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Represents a single message in the conversation history
#[derive(Debug, Clone)]
pub struct Message {
    role: Role,
    content: String,
}

/// Manages a coding session with an LLM
pub struct Session {
    name: String,
    config: CrushConfig,
    history: VecDeque<Message>,
    lsp_client: Option<LspClient>,
    mcp_client: Option<McpClient>,
    skip_prompts: bool,
    providers: HashMap<String, Box<dyn Provider>>,
    current_provider: Option<String>,
    current_model: Option<Model>,
}

impl Session {
    /// Creates a new session with the given name and configuration
    pub async fn new(name: &str, config: CrushConfig, skip_prompts: bool) -> Result<Self> {
        // Initialize providers
        let mut providers = HashMap::new();
        for (name, provider_config) in &config.providers {
            let provider: Box<dyn Provider> = match provider_config {
                ProviderConfig::Openai { base_url, api_key, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    Box::new(OpenAiProvider::new(base_url, api_key, models))
                }
                ProviderConfig::Anthropic { base_url, api_key, extra_headers, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    let headers: Vec<(String, String)> = extra_headers.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    Box::new(AnthropicProvider::new(
                        base_url,
                        api_key,
                        &headers,
                        models,
                    ))
                }
                ProviderConfig::Deepseek { base_url, api_key, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    Box::new(DeepseekProvider::new(base_url, api_key, models))
                }
                ProviderConfig::Gemini { api_key, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    Box::new(GeminiProvider::new(api_key, models))
                }
                ProviderConfig::Kimi { base_url, api_key, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    Box::new(KimiProvider::new(base_url, api_key, models))
                }
                ProviderConfig::Ollama { base_url, models } => {
                    let models = models.iter().map(|m| m.into()).collect();
                    Box::new(OllamaProvider::new(base_url, models))
                }
            };
            providers.insert(name.clone(), provider);
        }

        let mut session = Self {
            name: name.to_string(),
            config,
            history: VecDeque::new(),
            lsp_client: None,
            mcp_client: None,
            skip_prompts,
            providers,
            current_provider: None,
            current_model: None,
        };

        // Initialize LSP client if configured
        // TODO: Fix type mismatch between config and lsp module
        // if !session.config.lsp.is_empty() {
        //     session.lsp_client = Some(LspClient::new(&session.config.lsp).await?);
        // }

        // Initialize MCP client if configured
        // TODO: Fix type mismatch between config and mcp module
        // if !session.config.mcp.is_empty() {
        //     session.mcp_client = Some(McpClient::new(&session.config.mcp).await?);
        // }

        // Add system prompt to history
        session.add_message(
            Role::System,
            "You are an expert coding assistant. Help the user with their programming tasks.",
        );

        // Select initial provider and model
        session.select_provider()?;

        Ok(session)
    }

    /// Adds a new message to the conversation history
    pub fn add_message(&mut self, role: Role, content: &str) {
        self.history.push_back(Message {
            role,
            content: content.to_string(),
        });

        // Keep history within context window limits
        if self.history.len() > 20 {
            self.history.pop_front();
        }
    }

    /// Main REPL loop for user interaction
    pub async fn run(&mut self) -> Result<()> {
        println!("Welcome to Crush-RS session '{}'", self.name);
        if let (Some(provider), Some(model)) = (&self.current_provider, &self.current_model) {
            println!("Using provider: {}, model: {}", provider, model.name);
        }
        println!("Type your request or 'exit' to quit.");

        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        loop {
            print!("> ");
            tokio::io::stdout().flush().await?;

            if let Some(line) = reader.next_line().await? {
                match line.trim() {
                    "exit" | "quit" => break,
                    "switch" => {
                        self.select_provider()?;
                        if let (Some(provider), Some(model)) = (&self.current_provider, &self.current_model) {
                            println!("Switched to provider: {}, model: {}", provider, model.name);
                        }
                    }
                    "" => continue,
                    request => {
                        self.add_message(Role::User, request);
                        self.process_request(request).await?;
                    }
                }
            }
        }

        println!("Exiting session '{}'", self.name);
        Ok(())
    }

    /// Processes a user request
    async fn process_request(&mut self, request: &str) -> Result<()> {
        // Get relevant context
        let context = self.gather_context(request).await?;

        // Get current provider and model
        let (provider, model) = self.get_current_provider_and_model()?;

        // Generate response
        // Convert session messages to provider messages
        let provider_messages: VecDeque<ProviderMessage> = self.history.iter().map(|m| {
            ProviderMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            }
        }).collect();

        let response = provider
            .generate_response(&model, &provider_messages, &context)
            .await?;

        // Add response to history
        self.add_message(Role::Assistant, &response);

        // Show response to user
        println!("{}", response);

        Ok(())
    }

    /// Gathers context from various sources (LSP, MCP, etc.)
    async fn gather_context(&self, _request: &str) -> Result<String> {
        let context = String::new();

        // Add LSP context if available
        // TODO: Fix LSP context retrieval
        // if let Some(client) = &self.lsp_client {
        //     context.push_str(&client.get_context(request).await?.to_string());
        // }

        // Add MCP context if available
        // TODO: Fix MCP context retrieval
        // if let Some(client) = &self.mcp_client {
        //     context.push_str(&client.get_context(request).await?);
        // }

        // Add any other context sources here

        Ok(context)
    }

    /// Gets the current provider and model
    fn get_current_provider_and_model(&self) -> Result<(&dyn Provider, Model)> {
        let provider_name = self
            .current_provider
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No provider selected"))?;

        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_name))?
            .as_ref();

        let model = self
            .current_model
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No model selected"))?
            .clone();

        Ok((provider, model))
    }

    /// Selects an appropriate provider and model
    fn select_provider(&mut self) -> Result<()> {
        // Try to find a specific provider if configured
        for provider_name in ["kimi", "deepseek", "gemini", "anthropic", "openai", "ollama"] {
            if let Some(provider) = self.providers.get(provider_name) {
                if let Some(model) = provider.models().first() {
                    self.current_provider = Some(provider_name.to_string());
                    self.current_model = Some(model.clone());
                    return Ok(());
                }
            }
        }

        // If no specific provider found, use the first available
        for (provider_name, provider) in &self.providers {
            if let Some(model) = provider.models().first() {
                self.current_provider = Some(provider_name.clone());
                self.current_model = Some(model.clone());
                return Ok(());
            }
        }

        anyhow::bail!("No available providers or models configured")
    }

    /// Adds a system message to the conversation history
    pub fn add_system_message(&mut self, content: &str) {
        self.add_message(Role::System, content);
    }
}
