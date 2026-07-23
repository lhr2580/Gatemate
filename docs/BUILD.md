# 构建指南

## 环境要求

- Node.js >= 18.0.0
- Rust >= 1.75.0
- pnpm >= 8.0.0
- Tauri CLI (可选)

## 快速开始

### 1. 安装依赖

```bash
pnpm install
```

### 2. 构建前端

```bash
pnpm build
```

### 3. 开发模式

```bash
pnpm tauri dev
```

### 4. 生产构建

```bash
pnpm tauri build
```

## 构建选项

### 前端构建

```bash
# 开发模式
pnpm dev

# 生产构建
pnpm build

# 构建并预览
pnpm preview
```

### Rust 后端

```bash
# 检查代码
cargo check

# 开发构建
cargo build

# 生产构建
cargo build --release

# 运行测试
cargo test
```

## Pro 插件构建

> Pro 插件为闭源组件，需要单独构建并放置到应用目录。

### 构建 Pro 插件

1. 在独立仓库中构建：

```bash
cd gatemate-pro-plugin
cargo build --release --target x86_64-pc-windows-msvc
```

2. 复制构建产物：

```bash
cp target/x86_64-pc-windows-msvc/release/gatemate_pro_plugin.dll <app_data_dir>/plugins/
```

### 插件目录位置

- Windows: `%LOCALAPPDATA%\gatemate\plugins\`
- macOS: `~/Library/Application Support/gatemate/plugins/`
- Linux: `~/.config/gatemate/plugins/`

## 常见问题

### 构建脚本权限问题

如果遇到 esbuild 构建脚本被忽略的问题：

```bash
pnpm approve-builds --include esbuild
```

### Tauri 配置文件问题

确保 `src-tauri/tauri.conf.json` 存在且格式正确。构建脚本会自动处理配置文件复制。

### 依赖更新

```bash
pnpm update
cargo update
```

## 目录结构

```
gatemate/
├── src/                    # 前端源码
│   ├── components/         # Vue 组件
│   ├── App.vue            # 主应用组件
│   └── main.ts            # 入口文件
├── src-tauri/             # Tauri 后端
│   ├── gatemate-core/     # 核心业务逻辑
│   ├── gatemate-plugin-api/ # 插件 API 定义
│   └── tauri.conf.json    # Tauri 配置
├── dist/                  # 前端构建产物
└── package.json           # 前端依赖
```

## 跨平台构建

### Windows

```bash
pnpm tauri build --target x86_64-pc-windows-msvc
```

### macOS

```bash
pnpm tauri build --target aarch64-apple-darwin
```

### Linux

```bash
pnpm tauri build --target x86_64-unknown-linux-gnu
```

## 调试

### 前端调试

在开发模式下打开浏览器开发者工具：
- Windows: F12
- macOS: Cmd + Option + I

### Rust 调试

使用 VS Code 配合 Rust 插件进行调试，配置 `.vscode/launch.json`：

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug gatemate-core",
      "program": "${workspaceFolder}/src-tauri/target/debug/gatemate.exe",
      "args": [],
      "cwd": "${workspaceFolder}/src-tauri"
    }
  ]
}
```

## 更多信息

- [Tauri 文档](https://tauri.app/docs/)
- [Vue 3 文档](https://vuejs.org/)
- [Rust 文档](https://doc.rust-lang.org/)