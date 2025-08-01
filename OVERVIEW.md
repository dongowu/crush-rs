# Crush-RS Project Overview

## 📁 Project Structure

```
crush-rs/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI interface & interactive menu
│   ├── config.rs        # Configuration management
│   ├── llm.rs          # LLM provider implementations
│   ├── session.rs      # Chat session management
│   └── tools.rs        # Tool execution system
├── target/release/
│   └── crush.exe       # Compiled binary
├── README.md           # Main documentation
├── SETUP.md           # Setup and installation guide
├── config.example.json # Example configuration
└── Cargo.toml         # Rust dependencies
```

## 🔧 Core Components

### CLI Interface (`cli.rs`)
- Interactive provider selection menu
- Command-line argument parsing
- Provider status detection
- Setup instructions display

### LLM Providers (`llm.rs`)
- OpenAI API integration
- Anthropic API integration  
- Support for OpenAI-compatible APIs (DeepSeek, Ollama, Kimi)
- Flexible authentication handling

### Session Management (`session.rs`)
- Persistent chat conversations
- Session storage and retrieval
- Interactive chat loop
- Built-in commands (help, status, clear, etc.)

### Tool System (`tools.rs`)
- Safe tool execution with permission system
- File operations (read, write, list)
- Git integration (status, log)
- Shell command execution
- YOLO mode for advanced users

### Configuration (`config.rs`)
- JSON-based configuration storage
- Provider settings management
- Cross-platform config locations
- Default values and environment variable integration

## 🎯 Key Features

### ✨ Interactive Experience
- Beautiful terminal UI with color coding
- Real-time provider status indicators
- Intuitive keyboard navigation
- Contextual help and setup guidance

### 🔒 Security & Safety
- Permission-based tool execution
- Safe vs. dangerous tool classification
- User confirmation for risky operations
- Session isolation

### 🤖 Multi-LLM Support
- **5 Different Providers**: OpenAI, Anthropic, DeepSeek, Ollama, Kimi
- **Flexible APIs**: OpenAI-compatible and Anthropic-compatible
- **Local & Cloud**: Support for both local (Ollama) and cloud providers
- **Cost Options**: From free (Ollama) to premium (OpenAI)

### 💾 Persistence
- Chat history preservation
- Session continuation across restarts
- Configuration persistence
- Cross-platform data storage

## 📊 Technical Details

### Dependencies (Key)
- **clap**: Command-line interface
- **tokio**: Async runtime
- **reqwest**: HTTP client for API calls
- **dialoguer**: Interactive terminal UI
- **colored**: Terminal colors
- **serde**: JSON serialization
- **anyhow**: Error handling

### Supported Platforms
- ✅ Windows (PowerShell, CMD)
- ✅ macOS (Terminal, iTerm)
- ✅ Linux (bash, zsh, fish)

### Architecture Pattern
- Modular design with clear separation of concerns
- Async/await throughout for responsive UI
- Error handling with comprehensive user feedback
- Configuration-driven provider system

## 🚀 Getting Started

1. **Clone & Build**: `cargo build --release`
2. **Set API Keys**: Configure your preferred providers
3. **Run**: `crush.exe` for interactive mode
4. **Enjoy**: Beautiful AI assistance in your terminal!

## 🔮 Future Roadmap

- [ ] LSP integration for enhanced code context
- [ ] MCP (Model Context Protocol) support
- [ ] Plugin system for custom tools
- [ ] Advanced session management
- [ ] Tool result caching
- [ ] Multi-language support

---

**Crush-RS: Where beautiful terminal UI meets powerful AI assistance! 🦀💘**