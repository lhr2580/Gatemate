# GateMate

![GitHub license](https://img.shields.io/github/license/lhr2580/Gatemate)
![GitHub stars](https://img.shields.io/github/stars/lhr2580/Gatemate)
![GitHub issues](https://img.shields.io/github/issues/lhr2580/Gatemate)
![GitHub forks](https://img.shields.io/github/forks/lhr2580/Gatemate)

GateMate 是一个开源的 API 密钥管理和智能路由网关工具，帮助开发者管理多个 API 密钥，实现智能路由和流量分发，降低 API 调用成本。

## ✨ 功能特性

- **多项目管理**: 创建和管理多个项目，独立分配 API 密钥
- **密钥管理**: 安全存储和管理多个 API 密钥，支持多种服务商
- **智能路由**: 支持轮询、随机、智能路由策略
- **局域网代理**: 内置 HTTP/WebSocket 服务器，支持局域网内访问
- **使用统计**: 详细的调用日志和费用统计
- **预算管理**: 设置月度预算，超支预警
- **数据导出**: 支持 CSV 和 PDF 导出调用记录
- **插件系统**: 支持通过插件扩展 Pro 功能
- **数据加密**: AES-256-GCM 加密 + OS Keychain 集成

## 🌐 支持的服务商

- OpenAI
- DeepSeek
- Qwen
- Anthropic
- Gemini
- Doubao
- YiYan (百度文心一言)
- OpenRouter
- Groq
- Mistral
- Together AI
- Replicate
- Hugging Face

## 🛠️ 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri
- **数据库**: SQLite
- **网络**: WebSocket + HTTP
- **加密**: AES-256-GCM + OS Keychain

## 📦 构建要求

- Rust 1.70+
- Node.js 18+
- pnpm

## 🚀 安装步骤

### 1. 克隆仓库

```bash
git clone https://github.com/lhr2580/Gatemate.git
cd Gatemate
```

### 2. 安装前端依赖

```bash
pnpm install
```

### 3. 构建前端

```bash
pnpm build
```

### 4. 构建 Tauri 应用

```bash
pnpm tauri build
```

## 🔧 开发模式

```bash
# 启动前端开发服务器
pnpm dev

# 启动 Tauri 开发模式
pnpm tauri dev
```

## 🐛 常见问题

### Rust 安装
如果没有安装 Rust，可以通过 [rustup](https://rustup.rs/) 安装：

```bash
# Windows (PowerShell)
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe; .\rustup-init.exe -y

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### pnpm 安装
如果没有安装 pnpm，可以通过 npm 安装：

```bash
npm install -g pnpm
```

### 构建失败
- 确保安装了 Visual Studio Build Tools（Windows）或 Xcode（macOS）
- 确保 Rust 版本 >= 1.70
- 确保 Node.js 版本 >= 18

### 数据目录权限
应用会在以下目录创建数据文件：
- Windows: `%LOCALAPPDATA%\gatemate\`
- macOS: `~/Library/Application Support/gatemate/`
- Linux: `~/.config/gatemate/`

确保用户对上述目录有读写权限。

## 💾 数据存储

应用数据存储在以下位置：

- **Windows**: `%LOCALAPPDATA%\gatemate\` 或 `%APPDATA%\gatemate\`
- **macOS**: `~/Library/Application Support/gatemate/`
- **Linux**: `~/.config/gatemate/`

数据结构：
- `data/gatemate.db` - 数据库文件
- `backups/` - 备份文件
- `logs/` - 日志文件
- `master_key.bin` - 加密密钥文件（自动生成）

## 🔌 插件系统

GateMate 支持通过动态插件扩展功能。插件接口定义在 `gatemate-plugin-api` crate 中。

### 创建插件

1. 参考 `gatemate-plugin-api` 定义的接口
2. 实现 `Plugin` trait
3. 编译为 `.dll` 文件
4. 将插件放入应用目录

### Pro 插件

Pro 功能（智能路由、PDF 导出、极光主题等）通过闭源插件实现，不包含在开源仓库中。

## 📝 贡献

欢迎贡献代码！请阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 🤝 支持

如果您喜欢这个项目，可以通过以下方式支持：

- ⭐ Star 这个仓库
- 🐛 报告问题
- 💡 提出功能建议

## 🔒 安全

请阅读 [SECURITY.md](SECURITY.md) 了解安全相关信息。

## 📊 API

### 健康检查

```
GET http://localhost:38080/health
```

响应:
```json
{
  "status": "ok",
  "service": "gatemate",
  "version": "2.0.0"
}
```