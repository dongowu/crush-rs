use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;
use dialoguer::{Select, theme::ColorfulTheme};
use std::io::{self, Write};

use crate::{config::Config, session::Session, llm::LlmProvider};

#[derive(Parser)]
#[command(name = "crush")]
#[command(about = "A glamorous AI coding agent for your terminal 💘")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    #[arg(long, help = "Skip all permission prompts (use with extreme care)")]
    pub yolo: bool,
    
    #[arg(long, help = "Specify the LLM provider to use")]
    pub provider: Option<String>,
    
    #[arg(long, help = "Session name to use or create")]
    pub session: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start an interactive chat session")]
    Chat {
        #[arg(help = "Initial message to send")]
        message: Option<String>,
    },
    #[command(about = "List all available sessions")]
    Sessions,
    #[command(about = "Configure crush settings")]
    Config,
    #[command(about = "Show current configuration")]
    Status,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        let mut config = Config::load_or_create().await?;
        
        match &self.command {
            Some(Commands::Chat { message }) => {
                self.start_chat_session(&mut config, message.clone()).await?;
            }
            Some(Commands::Sessions) => {
                self.list_sessions().await?;
            }
            Some(Commands::Config) => {
                self.configure().await?;
            }
            Some(Commands::Status) => {
                self.show_status(&config).await?;
            }
            None => {
                // If no provider specified and no default provider, show model selection
                if self.provider.is_none() && config.default_provider.is_none() {
                    self.show_provider_selection_and_chat(&mut config).await?;
                } else {
                    self.start_chat_session(&mut config, None).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn start_chat_session(&self, config: &mut Config, initial_message: Option<String>) -> Result<()> {
        println!("{}", "🌟 Welcome to Crush - Your AI Coding Assistant".bright_magenta().bold());
        
        let provider = self.get_provider(config).await?;
        let mut session = Session::new_or_load(
            self.session.clone(),
            provider,
            self.yolo
        ).await?;
        
        if let Some(msg) = initial_message {
            session.send_message(&msg).await?;
        }
        
        session.start_interactive_loop().await?;
        
        Ok(())
    }
    
    async fn list_sessions(&self) -> Result<()> {
        let sessions = Session::list_all().await?;
        
        if sessions.is_empty() {
            println!("{}", "No sessions found.".dimmed());
        } else {
            println!("{}", "Available sessions:".bright_cyan().bold());
            for session in sessions {
                println!("  • {}", session.bright_white());
            }
        }
        
        Ok(())
    }
    
    async fn configure(&self) -> Result<()> {
        println!("{}", "Configuration wizard coming soon...".yellow());
        Ok(())
    }
    
    async fn show_status(&self, config: &Config) -> Result<()> {
        println!("{}", "Crush Status".bright_cyan().bold());
        println!("Provider: {}", config.default_provider.as_deref().unwrap_or("None").bright_white());
        println!("Config path: {}", config.config_path().display().to_string().dimmed());
        Ok(())
    }
    
    async fn get_provider(&self, config: &Config) -> Result<LlmProvider> {
        let provider_name = self.provider.as_ref()
            .or(config.default_provider.as_ref())
            .ok_or_else(|| anyhow::anyhow!("No LLM provider specified. Use --provider or configure a default."))?;
        
        LlmProvider::new(provider_name, config).await
    }
    
    async fn show_provider_selection_and_chat(&self, config: &mut Config) -> Result<()> {
        println!("{}", "🌟 Welcome to Crush - Your AI Coding Assistant".bright_magenta().bold());
        println!();
        
        // Get available providers and their status
        let mut available_providers = Vec::new();
        let mut provider_descriptions = Vec::new();
        
        for (name, provider_config) in &config.providers {
            let status = if name == "ollama" {
                // Ollama doesn't need API key
                "✅ Ready".green().to_string()
            } else if provider_config.api_key.is_some() {
                "✅ Ready".green().to_string()
            } else {
                "❌ No API Key".red().to_string()
            };
            
            let description = match name.as_str() {
                "openai" => format!("OpenAI GPT-4 - Industry leading AI model {}", status),
                "anthropic" => format!("Anthropic Claude - Advanced reasoning capabilities {}", status),
                "deepseek" => format!("DeepSeek - High performance, cost-effective {}", status),
                "ollama" => format!("Ollama - Local AI models (llama3.2) {}", status),
                "kimi2" => format!("Kimi - Moonshot AI with excellent Chinese support {}", status),
                _ => format!("{} - Custom provider {}", name, status),
            };
            
            available_providers.push(name.clone());
            provider_descriptions.push(description);
        }
        
        // Add option to set default
        available_providers.push("config".to_string());
        provider_descriptions.push("⚙️  Configure default provider".cyan().to_string());
        
        println!("{}", "Please select an AI provider:".bright_cyan().bold());
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&provider_descriptions)
            .interact()?;
        
        let selected_provider = &available_providers[selection];
        
        if selected_provider == "config" {
            self.configure_default_provider(config).await?;
            return Ok(());
        }
        
        // Check if the selected provider is ready
        let provider_config = config.providers.get(selected_provider).unwrap();
        if selected_provider != "ollama" && provider_config.api_key.is_none() {
            println!();
            println!("{}", format!("❌ {} requires an API key!", selected_provider).red().bold());
            self.show_provider_setup_instructions(selected_provider);
            return Ok(());
        }
        
        println!();
        println!("{}", format!("🚀 Starting chat with {}...", selected_provider).bright_green().bold());
        
        // Create provider and start chat
        match LlmProvider::new(selected_provider, config).await {
            Ok(provider) => {
                let mut session = Session::new_or_load(
                    self.session.clone(),
                    provider,
                    self.yolo
                ).await?;
                
                session.start_interactive_loop().await?;
            }
            Err(e) => {
                println!();
                println!("{}", "❌ Configuration Error".red().bold());
                println!("{}", format!("{}", e).bright_red());
                println!();
                if e.to_string().contains("base_url") {
                    println!("{}", "💡 Tip: Check your configuration file:".bright_yellow());
                    println!("   {}", config.config_path().display().to_string().bright_white());
                }
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    async fn configure_default_provider(&self, config: &mut Config) -> Result<()> {
        println!();
        println!("{}", "Configure Default Provider".bright_cyan().bold());
        
        let providers: Vec<String> = config.providers.keys().cloned().collect();
        let provider_names: Vec<&str> = providers.iter().map(|s| s.as_str()).collect();
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select default provider")
            .items(&provider_names)
            .interact()?;
        
        let selected = providers[selection].clone();
        config.default_provider = Some(selected.clone());
        config.save().await?;
        
        println!();
        println!("{}", format!("✅ Default provider set to: {}", selected).green().bold());
        
        Ok(())
    }
    
    fn show_provider_setup_instructions(&self, provider: &str) {
        println!();
        println!("{}", "Setup Instructions:".bright_yellow().bold());
        
        match provider {
            "openai" => {
                println!("1. Visit https://platform.openai.com/api-keys");
                println!("2. Create an API key");
                println!("3. Set environment variable:");
                println!("   {}", "set OPENAI_API_KEY=your-key-here".bright_white());
            }
            "anthropic" => {
                println!("1. Visit https://console.anthropic.com/");
                println!("2. Create an API key");
                println!("3. Set environment variable:");
                println!("   {}", "set ANTHROPIC_API_KEY=your-key-here".bright_white());
            }
            "deepseek" => {
                println!("1. Visit https://platform.deepseek.com/");
                println!("2. Create an API key");
                println!("3. Set environment variable:");
                println!("   {}", "set DEEPSEEK_API_KEY=your-key-here".bright_white());
            }
            "kimi2" => {
                println!("1. Visit https://platform.moonshot.cn/");
                println!("2. Create an API key");
                println!("3. Set environment variable:");
                println!("   {}", "set KIMI_API_KEY=your-key-here".bright_white());
            }
            _ => {
                println!("Please configure your API key for {}", provider);
            }
        }
        
        println!();
        println!("{}", "Then restart crush to use this provider.".dimmed());
    }
}