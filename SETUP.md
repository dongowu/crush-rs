# Crush-RS Setup Guide

## 🚀 Quick Setup

### 1. Build from Source
```bash
git clone <this-repo>
cd crush-rs
cargo build --release
```

### 2. Set API Keys (Choose Your Providers)

#### OpenAI (Industry Standard)
```bash
# Get API key from: https://platform.openai.com/api-keys
set OPENAI_API_KEY=sk-your-openai-key-here
```

#### DeepSeek (Cost-Effective) ⭐ Recommended
```bash
# Get API key from: https://platform.deepseek.com/
set DEEPSEEK_API_KEY=sk-your-deepseek-key-here
```

#### Ollama (Local & Free) ⭐ Recommended
```bash
# 1. Install from: https://ollama.ai/
# 2. Pull a model
ollama pull llama3.2
# 3. No API key needed!
```

#### Anthropic Claude
```bash
# Get API key from: https://console.anthropic.com/
set ANTHROPIC_API_KEY=sk-ant-your-key-here
```

#### Kimi2 (Chinese Support)
```bash
# Get API key from: https://platform.moonshot.cn/
set KIMI_API_KEY=sk-your-kimi-key-here
```

### 3. Run Crush
```bash
# Interactive mode - shows provider menu
.\target\release\crush.exe

# Or run with cargo
cargo run
```

## 💡 Usage Tips

### Interactive Mode (Recommended)
- Run `crush.exe` without arguments
- See all providers with status indicators
- Get setup help for missing API keys
- Configure default provider

### Direct Mode
```bash
# Chat with specific provider
crush.exe --provider deepseek chat
crush.exe --provider ollama chat

# Skip safety prompts (advanced)
crush.exe --yolo --provider openai chat

# Manage sessions
crush.exe sessions
crush.exe status
```

## 🛠️ Troubleshooting

### Common Issues

**"Provider requires API key"**
- Set the environment variable for your chosen provider
- Restart your terminal after setting variables

**"not a terminal" error**
- Use a proper terminal (Command Prompt, PowerShell, Terminal)
- Avoid running in IDE output windows

**Ollama connection failed**
- Make sure Ollama is installed and running
- Check that it's accessible at `http://localhost:11434`

### Debug Mode
```bash
set RUST_LOG=debug
crush.exe
```

## 🔧 Configuration Details

### API Endpoints & Authentication

| Provider | Base URL | Authentication Method | Default Model |
|----------|----------|----------------------|---------------|
| **OpenAI** | `https://api.openai.com/v1` | `Authorization: Bearer {api_key}` | `gpt-4` |
| **Anthropic** | `https://api.anthropic.com/v1` | `x-api-key: {api_key}` | `claude-3-sonnet-20240229` |
| **DeepSeek** | `https://api.deepseek.com/v1` | `Authorization: Bearer {api_key}` | `deepseek-chat` |
| **Ollama** | `http://localhost:11434/v1` | No authentication required | `llama3.2` |
| **Kimi2** | `https://api.moonshot.cn/v1` | `Authorization: Bearer {api_key}` | `moonshot-v1-8k` |

### Complete Configuration Example

Create your config file with all providers:

```json
{
  "default_provider": "deepseek",
  "providers": {
    "openai": {
      "api_type": "OpenAI",
      "api_key": "sk-your-openai-key-here",
      "base_url": "https://api.openai.com/v1",
      "model": "gpt-4"
    },
    "anthropic": {
      "api_type": "Anthropic",
      "api_key": "sk-ant-your-anthropic-key-here", 
      "base_url": "https://api.anthropic.com/v1",
      "model": "claude-3-sonnet-20240229"
    },
    "deepseek": {
      "api_type": "OpenAI",
      "api_key": "sk-your-deepseek-key-here",
      "base_url": "https://api.deepseek.com/v1", 
      "model": "deepseek-chat"
    },
    "ollama": {
      "api_type": "OpenAI",
      "api_key": null,
      "base_url": "http://localhost:11434/v1",
      "model": "llama3.2"
    },
    "kimi2": {
      "api_type": "OpenAI", 
      "api_key": "sk-your-kimi-key-here",
      "base_url": "https://api.moonshot.cn/v1",
      "model": "moonshot-v1-8k"
    }
  },
  "global_settings": {
    "auto_approve_safe_tools": false,
    "max_tokens": 4000,
    "temperature": 0.7
  }
}
```

### Available Models by Provider

#### OpenAI Models
- `gpt-4` - Most capable, best for complex tasks
- `gpt-4-turbo` - Faster GPT-4 variant
- `gpt-3.5-turbo` - Fast and cost-effective

#### Anthropic Models  
- `claude-3-opus-20240229` - Most powerful Claude model
- `claude-3-sonnet-20240229` - Balanced performance and speed
- `claude-3-haiku-20240307` - Fastest, most cost-effective

#### DeepSeek Models
- `deepseek-chat` - General purpose conversational AI
- `deepseek-coder` - Optimized for coding tasks

#### Ollama Models
- `llama3.2` - Latest Llama model, excellent general performance
- `codellama` - Specialized for code generation
- `mistral` - High-quality open model
- `qwen` - Multilingual capabilities

#### Kimi2 Models
- `moonshot-v1-8k` - 8K context window
- `moonshot-v1-32k` - 32K context window  
- `moonshot-v1-128k` - 128K context window (long conversations)

## 🎯 Recommended Setup

For the best experience, we recommend this combination:

### 🏆 Optimal Configuration
1. **Ollama** (Free & Private) - For local development and privacy-sensitive tasks
2. **DeepSeek** (Cost-Effective) - For general cloud AI assistance  
3. **OpenAI GPT-4** (Premium) - For complex reasoning and high-quality outputs

### 💰 Budget-Friendly Setup
1. **Ollama** - Free, runs locally
2. **DeepSeek** - Excellent performance at low cost

### 🚀 Power User Setup  
1. **All providers configured** - Switch between them based on task requirements
2. **Set DeepSeek as default** - Best balance of cost and performance
3. **Use Ollama for sensitive data** - Keep private information local

### Quick Setup Commands
```bash
# Copy example config
cp config.example.json ~/.config/crush/config.json

# Set your API keys
export DEEPSEEK_API_KEY="sk-your-deepseek-key"
export OPENAI_API_KEY="sk-your-openai-key"  # optional
export ANTHROPIC_API_KEY="sk-ant-your-key"  # optional

# Install Ollama (optional but recommended)
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull llama3.2

# Start Crush
crush.exe
```

This gives you local privacy, cost-effective cloud access, and premium capabilities when needed! 🎉