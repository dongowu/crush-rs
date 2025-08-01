use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_provider: Option<String>,
    pub providers: HashMap<String, ProviderConfig>,
    pub global_settings: GlobalSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_type: ApiType,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiType {
    OpenAI,
    Anthropic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub auto_approve_safe_tools: bool,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

impl Default for Config {
    fn default() -> Self {
        let mut providers = HashMap::new();
        
        providers.insert("openai".to_string(), ProviderConfig {
            api_type: ApiType::OpenAI,
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            model: Some("gpt-4".to_string()),
        });
        
        providers.insert("anthropic".to_string(), ProviderConfig {
            api_type: ApiType::Anthropic,
            api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            base_url: Some("https://api.anthropic.com/v1".to_string()),
            model: Some("claude-3-sonnet-20240229".to_string()),
        });
        
        providers.insert("deepseek".to_string(), ProviderConfig {
            api_type: ApiType::OpenAI, // DeepSeek uses OpenAI-compatible API
            api_key: std::env::var("DEEPSEEK_API_KEY").ok(),
            base_url: Some("https://api.deepseek.com/v1".to_string()),
            model: Some("deepseek-chat".to_string()),
        });
        
        providers.insert("ollama".to_string(), ProviderConfig {
            api_type: ApiType::OpenAI, // Ollama uses OpenAI-compatible API
            api_key: None, // Ollama typically doesn't require API key for local usage
            base_url: Some("http://localhost:11434/v1".to_string()),
            model: Some("llama3.2".to_string()), // Default model, can be changed
        });
        
        providers.insert("kimi2".to_string(), ProviderConfig {
            api_type: ApiType::OpenAI, // Kimi uses OpenAI-compatible API
            api_key: std::env::var("KIMI_API_KEY").ok(),
            base_url: Some("https://api.moonshot.cn/v1".to_string()),
            model: Some("moonshot-v1-8k".to_string()),
        });
        
        Self {
            default_provider: None,
            providers,
            global_settings: GlobalSettings {
                auto_approve_safe_tools: false,
                max_tokens: Some(4000),
                temperature: Some(0.7),
            },
        }
    }
}

impl Config {
    pub async fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path_static();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }
    
    pub async fn save(&self) -> Result<()> {
        let config_path = self.config_path();
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;
        
        Ok(())
    }
    
    pub fn config_path(&self) -> PathBuf {
        Self::config_path_static()
    }
    
    fn config_path_static() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("crush")
            .join("config.json")
    }
    
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }
}