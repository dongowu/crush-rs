# Crush-RS

A Rust implementation of the [Crush](https://github.com/charmbracelet/crush) AI coding assistant for your terminal. Crush-RS brings the power of large language models to your development workflow directly from the command line.

## Features

- **Multi-Model Support**: Integrates with OpenAI, Anthropic, Deepseek, Gemini, Kimi (Moonshot), and Ollama providers
- **Session Management**: Maintain conversation history across requests
- **LSP Integration**: Leverages Language Server Protocol for contextual awareness
- **MCP Support**: Extend capabilities with Model Context Protocol
- **Terminal-First**: Designed specifically for terminal workflows
- **Local Model Support**: Full support for Ollama local models

## Installation

1. Install Rust using [rustup](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/your-username/crush-rs
   ```
3. Build the project:
   ```bash
   cd crush-rs
   cargo build --release
   ```
4. Run the executable:
   ```bash
   ./target/release/crush-rs
   ```

## Configuration

Create a `crush.json` file in your project directory or `~/.config/crush/`:

```json
{
  "$schema": "https://charm.land/crush.json",
  "providers": {
    "deepseek": {
      "type": "deepseek",
      "base_url": "https://api.deepseek.com/v1",
      "api_key": "$DEEPSEEK_API_KEY",
      "models": [
        {
          "id": "deepseek-kimi2",
          "name": "Deepseek Kimi2",
          "cost_per_1m_in": 0.35,
          "cost_per_1m_out": 1.4,
          "context_window": 128000,
          "default_max_tokens": 8000,
          "can_reason": true,
          "supports_attachments": true
        }
      ]
    }
  },
  "lsp": {
    "rust": {
      "command": "rust-analyzer"
    }
  }
}
```

## Usage

Start a new session:

```bash
crush-rs --session my-project
```

Interact with the assistant:

```
> How do I reverse a string in Rust?
```

Example response using Deepseek Kimi2:

```rust
// Reversing a string in Rust
let s = "hello";
let reversed: String = s.chars().rev().collect();
println!("{}", reversed); // Output: "olleh"
```

Switch models during a session:
```
> switch
Switched to provider: deepseek, model: Deepseek Kimi2
```

## Environment Variables

Set your API keys in your environment:

```bash
# Deepseek
export DEEPSEEK_API_KEY=your-api-key

# Kimi (Moonshot)
export KIMI_API_KEY=your-api-key

# Gemini
export GEMINI_API_KEY=your-api-key

# OpenAI
export OPENAI_API_KEY=your-api-key

# Anthropic
export ANTHROPIC_API_KEY=your-api-key
```

For Ollama, make sure you have Ollama running locally:

```bash
# Install and start Ollama
ollama serve

# Pull some models
ollama pull llama3.2
ollama pull qwen2.5-coder
```

## Contributing

Contributions are welcome! Please open an issue or pull request for any improvements.

## License

FSL-1.1-MIT © 2025