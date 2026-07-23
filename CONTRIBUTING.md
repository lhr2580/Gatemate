# 贡献指南

欢迎贡献代码！请阅读以下指南，了解如何参与项目。

## 📋 行为准则

请遵守 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) 中的行为准则。

## 🔧 开发环境

### 前置要求

- Rust 1.70+
- Node.js 18+
- pnpm

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建

```bash
pnpm tauri build
```

### 测试

```bash
# 前端测试
pnpm test

# 前端测试 UI
pnpm test:ui

# 前端测试覆盖率
pnpm test:coverage

# Rust 测试
cargo test --manifest-path src-tauri/gatemate-core/Cargo.toml
```

## 🐛 报告问题

请在 GitHub Issues 中报告问题。报告时请提供：

- 问题描述
- 复现步骤
- 截图（如有）
- 操作系统版本
- 应用版本

## 💡 功能请求

欢迎提出功能建议！请在 GitHub Issues 中创建功能请求。

## 📝 代码贡献

### 流程

1. Fork 仓库
2. 创建特性分支 (`git checkout -b feature/fooBar`)
3. 提交更改 (`git commit -m 'Add some fooBar'`)
4. 推送到分支 (`git push origin feature/fooBar`)
5. 创建 Pull Request

### 代码规范

- **Rust**: 遵循 Rust 官方代码规范，运行 `cargo fmt` 格式化代码，运行 `cargo clippy` 检查
- **TypeScript/Vue**: 遵循 ESLint 规则，运行 `pnpm build` 检查
- **提交信息**: 使用语义化提交信息

### Pull Request 要求

- 代码通过编译
- 没有新的警告
- 添加相关测试（如有）
- 更新文档（如有）
- 更新 CHANGELOG.md（如有）

## 🔌 插件开发

如果您想开发插件，请参考 `gatemate-plugin-api` crate 中的接口定义。

## 📄 许可证

贡献的代码将遵循 MIT License。

## 🤝 联系方式

- GitHub: [lhr2580/Gatemate](https://github.com/lhr2580/Gatemate)
- 邮箱: contact@gatemate.dev
