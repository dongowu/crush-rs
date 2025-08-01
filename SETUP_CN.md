# Crush-RS 安装配置指南

## 🚀 快速开始

### 1. 从源码构建
```bash
git clone <this-repo>
cd crush-rs
cargo build --release
```

### 2. 设置 API 密钥（选择你需要的提供商）

#### OpenAI（行业标准）
```bash
# 从这里获取 API 密钥: https://platform.openai.com/api-keys
set OPENAI_API_KEY=sk-your-openai-key-here
```

#### DeepSeek（性价比推荐） ⭐ 推荐
```bash
# 从这里获取 API 密钥: https://platform.deepseek.com/
set DEEPSEEK_API_KEY=sk-your-deepseek-key-here
```

#### Ollama（本地免费） ⭐ 推荐
```bash
# 1. 从这里安装: https://ollama.ai/
# 2. 拉取模型
ollama pull llama3.2
# 3. 无需 API 密钥！
```

#### Anthropic Claude
```bash
# 从这里获取 API 密钥: https://console.anthropic.com/
set ANTHROPIC_API_KEY=sk-ant-your-key-here
```

#### Kimi2（中文支持优秀）
```bash
# 从这里获取 API 密钥: https://platform.moonshot.cn/
set KIMI_API_KEY=sk-your-kimi-key-here
```

### 3. 运行 Crush
```bash
# 交互式模式 - 显示提供商菜单
.\target\release\crush.exe

# 或者使用 cargo 运行
cargo run
```

## 💡 使用技巧

### 交互式模式（推荐）
- 直接运行 `crush.exe`，无需参数
- 查看所有提供商的状态指示器
- 获取缺失 API 密钥的设置帮助
- 配置默认提供商

### 直接模式
```bash
# 与特定提供商聊天
crush.exe --provider deepseek chat
crush.exe --provider ollama chat

# 跳过安全提示（高级用户）
crush.exe --yolo --provider openai chat

# 管理会话
crush.exe sessions
crush.exe status
```

## 🛠️ 故障排除

### 常见问题

**"提供商需要 API 密钥"**
- 为你选择的提供商设置环境变量
- 设置变量后重启终端

**"不是终端" 错误**
- 使用正确的终端（命令提示符、PowerShell、终端）
- 避免在 IDE 输出窗口中运行

**Ollama 连接失败**
- 确保 Ollama 已安装并运行
- 检查是否可以访问 `http://localhost:11434`

### 调试模式
```bash
set RUST_LOG=debug
crush.exe
```

## 🔧 配置详情

### API 端点和认证

| 提供商 | 基础 URL | 认证方法 | 默认模型 |
|----------|----------|----------------------|---------------|
| **OpenAI** | `https://api.openai.com/v1` | `Authorization: Bearer {api_key}` | `gpt-4` |
| **Anthropic** | `https://api.anthropic.com/v1` | `x-api-key: {api_key}` | `claude-3-sonnet-20240229` |
| **DeepSeek** | `https://api.deepseek.com/v1` | `Authorization: Bearer {api_key}` | `deepseek-chat` |
| **Ollama** | `http://localhost:11434/v1` | 无需认证 | `llama3.2` |
| **Kimi2** | `https://api.moonshot.cn/v1` | `Authorization: Bearer {api_key}` | `moonshot-v1-8k` |

### 完整配置示例

创建包含所有提供商的配置文件：

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

### 各提供商可用模型

#### OpenAI 模型
- `gpt-4` - 最强能力，适合复杂任务
- `gpt-4-turbo` - 更快的 GPT-4 变体
- `gpt-3.5-turbo` - 快速且经济实惠

#### Anthropic 模型  
- `claude-3-opus-20240229` - 最强大的 Claude 模型
- `claude-3-sonnet-20240229` - 性能和速度的平衡
- `claude-3-haiku-20240307` - 最快，最经济

#### DeepSeek 模型
- `deepseek-chat` - 通用对话 AI
- `deepseek-coder` - 编程任务优化

#### Ollama 模型
- `llama3.2` - 最新 Llama 模型，综合性能优秀
- `codellama` - 代码生成专用
- `mistral` - 高质量开源模型
- `qwen` - 多语言能力强

#### Kimi2 模型
- `moonshot-v1-8k` - 8K 上下文窗口
- `moonshot-v1-32k` - 32K 上下文窗口  
- `moonshot-v1-128k` - 128K 上下文窗口（长対话）

## 🎯 推荐配置

针对最佳体验，我们推荐以下组合：

### 🏆 最优配置
1. **Ollama**（免费私有） - 用于本地开发和隐私敏感任务
2. **DeepSeek**（性价比高） - 用于一般云端 AI 辅助  
3. **OpenAI GPT-4**（高端） - 用于复杂推理和高质量输出

### 💰 预算友好配置
1. **Ollama** - 免费，本地运行
2. **DeepSeek** - 低成本，性能优秀

### 🚀 高级用户配置  
1. **配置所有提供商** - 根据任务需求切换
2. **设置 DeepSeek 为默认** - 成本和性能的最佳平衡
3. **使用 Ollama 处理敏感数据** - 保持私有信息本地化

### 快速设置命令
```bash
# 复制示例配置
copy config.example.json %APPDATA%\crush\config.json

# 设置你的 API 密钥
set DEEPSEEK_API_KEY=sk-your-deepseek-key
set OPENAI_API_KEY=sk-your-openai-key     # 可选
set ANTHROPIC_API_KEY=sk-ant-your-key     # 可选

# 安装 Ollama（可选但推荐）
# 访问 https://ollama.ai/ 下载安装程序
ollama pull llama3.2

# 启动 Crush
crush.exe
```

这样你就拥有了本地隐私保护、经济实惠的云端访问，以及需要时的高端能力！🎉

## 📞 获取 API 密钥的详细步骤

### OpenAI
1. 访问 [OpenAI Platform](https://platform.openai.com/)
2. 注册账户并登录
3. 点击 "API keys" 
4. 点击 "Create new secret key"
5. 复制生成的密钥（以 `sk-` 开头）

### DeepSeek  
1. 访问 [DeepSeek Platform](https://platform.deepseek.com/)
2. 注册账户并登录
3. 在控制台创建 API 密钥
4. 复制生成的密钥

### Anthropic Claude
1. 访问 [Anthropic Console](https://console.anthropic.com/)
2. 注册账户并登录  
3. 创建新的 API 密钥
4. 复制生成的密钥（以 `sk-ant-` 开头）

### Kimi (月之暗面)
1. 访问 [Kimi 开放平台](https://platform.moonshot.cn/)
2. 注册账户并登录
3. 在 API 管理中创建密钥
4. 复制生成的密钥

### Ollama（本地，无需密钥）
1. 访问 [Ollama 官网](https://ollama.ai/)
2. 下载适用于你系统的安装程序
3. 安装完成后运行 `ollama pull llama3.2`
4. 服务会自动在 `localhost:11434` 启动

## 🔐 安全建议

- 🔑 **保护你的 API 密钥** - 永远不要在代码中硬编码或公开分享
- 🌐 **使用环境变量** - 推荐的密钥存储方式
- 💰 **监控使用量** - 定期检查 API 使用情况和费用
- 🏠 **使用 Ollama 处理敏感数据** - 保持私有信息不离开你的设备
- 🔄 **定期轮换密钥** - 定期更新 API 密钥以提高安全性