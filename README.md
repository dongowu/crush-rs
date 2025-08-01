# Crush-RS 🦀💘

> A glamorous AI coding agent for your terminal, written in Rust.

Inspired by [Charmbracelet's Crush](https://github.com/charmbracelet/crush), this is a high-performance Rust implementation that brings AI assistance directly to your terminal with beautiful interactive interfaces.

## ✨ Features

- 🤖 **Multi-LLM Support** - OpenAI, Anthropic, DeepSeek, Ollama, and Kimi
- 🎨 **Interactive Provider Selection** - Beautiful terminal UI for choosing AI models
- 💬 **Persistent Chat Sessions** - Save and resume conversations
- 🔧 **Smart Tool Execution** - Run commands with intelligent permission system
- 🛡️ **Safety First** - YOLO mode for advanced users, permission prompts for safety
- 📁 **Session Management** - Multiple concurrent projects and conversations
- ⚡ **Cross-Platform** - Windows, macOS, and Linux support

## 🚀 Quick Start

### Installation

1. **Clone and build:**
   ```bash
   git clone <this-repo>
   cd crush-rs
   cargo build --release
   ```

2. **Set up API keys (optional):**
   ```bash
   # Windows
   set OPENAI_API_KEY=your-openai-key
   set DEEPSEEK_API_KEY=your-deepseek-key
   set ANTHROPIC_API_KEY=your-anthropic-key
   set KIMI_API_KEY=your-kimi-key
   # Ollama doesn't need an API key - just install and run locally
   ```

3. **Run Crush:**
   ```bash
   # Interactive mode (recommended)
   .\target\release\crush.exe
   
   # Or with cargo
   cargo run
   ```

📖 **Detailed Setup Guide:** [SETUP.md](SETUP.md) | [中文安装指南](SETUP_CN.md)

## 🎯 Usage

### Interactive Mode (Recommended)

Simply run `crush.exe` to see a beautiful provider selection menu:

```
🌟 Welcome to Crush - Your AI Coding Assistant

Please select an AI provider:
❯ OpenAI GPT-4 - Industry leading AI model ✅ Ready
  Anthropic Claude - Advanced reasoning capabilities ❌ No API Key
  DeepSeek - High performance, cost-effective ✅ Ready  
  Ollama - Local AI models (llama3.2) ✅ Ready
  Kimi - Moonshot AI with excellent Chinese support ❌ No API Key
  ⚙️  Configure default provider
```

### Direct Commands

```bash
# Start chat with specific provider
crush.exe --provider deepseek chat
crush.exe --provider ollama chat

# List sessions
crush.exe sessions

# Show status
crush.exe status

# YOLO mode (skip all permission prompts)
crush.exe --yolo --provider openai chat
```

## 🤖 Supported AI Providers

| Provider | Description | API Key Required | Default Model |
|----------|-------------|------------------|---------------|
| **OpenAI** | Industry-leading GPT models | ✅ | gpt-4 |
| **Anthropic** | Advanced reasoning with Claude | ✅ | claude-3-sonnet |
| **DeepSeek** | High performance, cost-effective | ✅ | deepseek-chat |
| **Ollama** | Local AI models, privacy-focused | ❌ | llama3.2 |
| **Kimi2** | Excellent Chinese language support | ✅ | moonshot-v1-8k |

## 🛠️ Available Tools

### Safe Tools (No Permission Required)
- `ls` / `list_files` - List directory contents
- `cat` / `read_file` - Read file contents  
- `pwd` / `get_current_directory` - Show current directory
- `git status` - Show git repository status
- `git log` - Show commit history
- `which` - Find command location
- `echo` - Echo messages

### Protected Tools (Require Permission)
- `shell` / `bash` / `cmd` - Execute shell commands
- `write_file` - Write content to files

## 📁 Configuration

### Config Locations
- **Windows:** `%APPDATA%\crush\config.json`
- **macOS:** `~/Library/Application Support/crush/config.json`  
- **Linux:** `~/.config/crush/config.json`

### Session Storage  
- **Windows:** `%APPDATA%\crush\sessions\`
- **macOS:** `~/Library/Application Support/crush/sessions/`
- **Linux:** `~/.local/share/crush/sessions/`

### Example Configuration
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
      "api_key": "sk-ant-your-key-here",
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

### API Endpoints Reference

| Provider | API Endpoint | Authentication | Models Available |
|----------|-------------|----------------|------------------|
| **OpenAI** | `https://api.openai.com/v1` | Bearer Token | `gpt-4`, `gpt-3.5-turbo` |
| **Anthropic** | `https://api.anthropic.com/v1` | x-api-key Header | `claude-3-sonnet-20240229`, `claude-3-haiku-20240307` |
| **DeepSeek** | `https://api.deepseek.com/v1` | Bearer Token | `deepseek-chat`, `deepseek-coder` |
| **Ollama** | `http://localhost:11434/v1` | No Auth Required | `llama3.2`, `codellama`, `mistral` |
| **Kimi2** | `https://api.moonshot.cn/v1` | Bearer Token | `moonshot-v1-8k`, `moonshot-v1-32k`, `moonshot-v1-128k` |

### Configuration Setup

1. **Copy Example Configuration:**
   ```bash
   # Linux/macOS
   cp config.example.json ~/.config/crush/config.json
   
   # Windows
   copy config.example.json %APPDATA%\crush\config.json
   ```

2. **Set API Keys:**
   Replace `null` or placeholder values with your actual API keys:
   ```json
   "api_key": "sk-your-actual-api-key-here"
   ```

3. **Customize Models (Optional):**
   - **OpenAI**: `gpt-4`, `gpt-4-turbo`, `gpt-3.5-turbo`
   - **Anthropic**: `claude-3-sonnet-20240229`, `claude-3-haiku-20240307`, `claude-3-opus-20240229`
   - **DeepSeek**: `deepseek-chat`, `deepseek-coder`
   - **Ollama**: `llama3.2`, `codellama`, `mistral`, `qwen`
   - **Kimi2**: `moonshot-v1-8k`, `moonshot-v1-32k`, `moonshot-v1-128k`

## 📖 Provider Setup Guides

### Ollama (Local AI)
1. **Install Ollama:** Visit [ollama.ai](https://ollama.ai/)  
2. **Pull a model:** `ollama pull llama3.2`
3. **Start server:** `ollama serve` (usually auto-starts)
4. **Use with Crush:** No API key needed!

### DeepSeek (Cost-Effective)
1. **Get API key:** [platform.deepseek.com](https://platform.deepseek.com/)
2. **Set environment:** `set DEEPSEEK_API_KEY=your-key`
3. **Enjoy low-cost AI:** Great performance at competitive prices

### Kimi2 (Chinese Support)  
1. **Get API key:** [platform.moonshot.cn](https://platform.moonshot.cn/)
2. **Set environment:** `set KIMI_API_KEY=your-key`
3. **Perfect for Chinese:** Excellent Chinese language understanding

## 🎮 Interactive Commands

Once in a chat session:
- `exit`, `quit`, `:q` - Exit the session
- `clear`, `:clear` - Clear the screen  
- `help`, `:help` - Show available commands
- `status`, `:status` - Show session information

## 🔒 Safety Features

- **Permission System** - Asks before running potentially dangerous commands
- **Safe Tools Whitelist** - Read-only operations don't require permission
- **YOLO Mode** - For advanced users who want to skip all prompts
- **Session Isolation** - Each session is independent and secure

## 🚧 Development Status

- [x] Core CLI interface with clap
- [x] Multi-LLM provider support (OpenAI/Anthropic/DeepSeek/Ollama/Kimi) 
- [x] Interactive provider selection menu
- [x] Session management and persistence
- [x] Tool execution with permission system
- [x] Cross-platform compatibility
- [ ] LSP integration for enhanced code context
- [ ] MCP (Model Context Protocol) support
- [ ] Plugin system for extensibility

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by [Charmbracelet's Crush](https://github.com/charmbracelet/crush)
- Built with ❤️ using Rust 🦀
- Thanks to the amazing Rust community

---

**Ready to crush your coding tasks? Run `crush.exe` and let's get started! 🚀**