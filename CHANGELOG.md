# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- 添加前端测试框架 vitest
- 添加 CODEOWNERS 文件
- 添加 .env.example 环境变量示例文件
- 添加 Dependabot 配置

### Changed

- 补全 package.json 和 Cargo.toml 元数据字段
- 更新 README 添加界面预览和插件 API 文档

### Fixed

- 修复 App.vue 模板中 Math.random() 响应式问题
- 修复 CI 工作流未安装 pnpm 问题
- 删除重复的 tauri.conf.json 文件
- 增强主密钥存储安全性，移除明文回退存储

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