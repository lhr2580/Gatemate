# 支持指南

## 如何获取帮助

### GitHub Issues

如果您遇到问题或有功能请求，请在 GitHub Issues 中创建新话题：

[提交 Issue](https://github.com/lhr2580/Gatemate/issues)

### 常见问题

#### 1. 应用启动失败

**问题**: `disk I/O error` 或 `无法创建数据目录`

**解决方案**:
- 确保您有足够的权限
- 检查磁盘空间是否充足
- 应用数据目录：
  - Windows: `%LOCALAPPDATA%\gatemate\`
  - macOS: `~/Library/Application Support/gatemate/`
  - Linux: `~/.config/gatemate/`

#### 2. API Key 测试失败

**问题**: Key 测试返回错误

**解决方案**:
- 检查 API Key 是否正确
- 检查网络连接
- 检查服务商状态

#### 3. HTTP 服务器无法启动

**问题**: 端口 38080 被占用

**解决方案**:
- 关闭占用端口的其他应用
- 修改环境变量 `GATEMATE_SERVER_PORT`

#### 4. 构建失败

**问题**: `pnpm tauri build` 失败

**解决方案**:
- 确保安装了 Rust 1.70+
- 确保安装了 Node.js 18+
- 确保安装了 Visual Studio Build Tools（Windows）或 Xcode（macOS）

### 联系我们

- **GitHub**: [lhr2580/Gatemate](https://github.com/lhr2580/Gatemate)
- **邮箱**: contact@gatemate.dev

## 支持计划

| 版本 | 支持状态 |
|------|----------|
| Latest | ✅ 支持 |
| < 1.0 | ❌ 不支持 |

## 社区支持

欢迎加入社区讨论！

- GitHub Discussions（即将开放）
