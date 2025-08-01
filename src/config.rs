use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct CrushConfig {
    #[serde(default)]
    pub lsp: HashMap<String, LspConfig>,
    #[serde(default)]
    pub mcp: HashMap<String, McpConfig>,
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    #[serde(default)]
    pub permissions: PermissionsConfig,
    #[serde(default)]
    pub options: OptionsConfig,
}

#[derive(Debug, Deserialize)]
pub struct LspConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpConfig {
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
    Http {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    Sse {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
    Openai {
        base_url: String,
        api_key: String,
        models: Vec<ModelConfig>,
    },
    Anthropic {
        base_url: String,
        api_key: String,
        #[serde(default)]
        extra_headers: HashMap<String, String>,
        models: Vec<ModelConfig>,
    },
    Deepseek {
        base_url: String,
        api_key: String,
        models: Vec<ModelConfig>,
    },
    Gemini {
        api_key: String,
        models: Vec<ModelConfig>,
    },
    Kimi {
        base_url: String,
        api_key: String,
        models: Vec<ModelConfig>,
    },
    Ollama {
        base_url: String,
        models: Vec<ModelConfig>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub context_window: usize,
    pub default_max_tokens: usize,
    pub cost_per_1m_in: f32,
    pub cost_per_1m_out: f32,
    #[serde(default)]
    pub cost_per_1m_in_cached: Option<f32>,
    #[serde(default)]
    pub cost_per_1m_out_cached: Option<f32>,
    #[serde(default)]
    pub can_reason: bool,
    #[serde(default)]
    pub supports_attachments: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct PermissionsConfig {
    #[serde(default)]
    pub allowed_tools: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OptionsConfig {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub debug_lsp: bool,
}

pub async fn load_config() -> anyhow::Result<CrushConfig> {
    let mut config = config::Config::builder();

    // Add configuration sources in priority order
    config = config.add_source(
        config::File::with_name(".crush").required(false)
    );
    config = config.add_source(
        config::File::with_name("crush").required(false)
    );

    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push("crush");
        config_dir.push("crush");
        config = config.add_source(
            config::File::from(config_dir).required(false)
        );
    }

    // Add environment variables
    config = config.add_source(
        config::Environment::with_prefix("CRUSH")
            .separator("__")
            .try_parsing(true),
    );

    let config = config.build()?;
    config.try_deserialize().map_err(|e| e.into())
}
