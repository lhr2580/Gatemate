# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- 添加 GitHub Actions CI 工作流，支持 Windows/macOS/Linux 多平台构建
- 添加 `/health` 健康检查端点
- 添加 `.editorconfig` 和 `rustfmt.toml` 工程规范文件
- 添加单元测试 (crypto, db 模块)
- 添加 `providers.rs` 模块，统一管理服务商配置和费用计算
- 添加 `SECURITY.md` 安全策略文档

### Changed

- 将 Chart.js 从 CDN 加载改为 npm 依赖，解决 CSP 阻止问题
- 将日期查询从 LIKE 改为范围查询，提升数据库查询性能
- 更新 `export_pdf` 命令，使用 `printpdf` 库生成真正的 PDF 文件
- 更新 `SECURITY.md` 安全上报渠道

### Fixed

- 修复 FFI 内存安全问题：`free_string` 使用 `CString::from_raw` 正确释放内存
- 修复 `plugin_loader.rs` 的 double-free 问题，移除对输入参数的 free 调用
- 修复 `chrono::Duration::months(1)` 已废弃问题，改用 `Month::succ()`
- 修复 `c_char_to_string` 函数，使用 `CStr::from_ptr` 复制数据而非窃取所有权

### Security

- 使用 `zeroize` 特性确保敏感数据使用后安全清除
- AES-256-GCM 加密 API 密钥存储

## [2.0.0] - 2026-07-21

### Added

- 初始版本发布
- 支持 OpenAI, DeepSeek, Qwen, Anthropic, Gemini, Doubao, YiYan 等服务商
- API 密钥管理（加密存储）
- 项目管理
- 调用日志记录
- 费用统计
- 路由策略（轮询/随机）
- 局域网代理服务