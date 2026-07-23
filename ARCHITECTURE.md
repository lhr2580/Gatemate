# GateMate 架构说明

## 项目概览

GateMate 是一个 API Key 管理和路由网关工具，帮助开发者管理多个 AI 服务提供商的 API Key，实现智能路由、成本控制和用量监控。

## 技术栈

- **前端**: Vue 3 + TypeScript + Tauri
- **后端**: Rust (Actix Web)
- **数据库**: SQLite
- **加密**: AES-256-GCM + OS Keychain (keyring crate)
- **构建工具**: pnpm + Vite

## 项目结构

```
gatemate/
├── src/                    # 前端 Vue 源码
│   ├── components/         # 可复用组件
│   │   ├── DashboardView.vue    # 仪表盘视图
│   │   ├── KeysView.vue         # API Key 管理视图
│   │   ├── LogsView.vue         # 调用日志视图
│   │   ├── SettingsModal.vue    # 设置模态框
│   │   ├── BudgetModal.vue      # 预算设置模态框
│   │   ├── AddKeyModal.vue      # 添加/编辑 Key 模态框
│   │   └── CommandPalette.vue   # 命令面板
│   ├── App.vue             # 主应用组件
│   └── main.ts             # 应用入口
├── src-tauri/              # Tauri 后端
│   ├── src/                # Tauri 主进程
│   ├── gatemate-core/      # 核心业务逻辑
│   │   ├── src/
│   │   │   ├── lib.rs          # 核心 API 定义
│   │   │   ├── server.rs       # HTTP 服务器
│   │   │   ├── constants.rs    # 常量定义
│   │   │   ├── crypto.rs       # 加密模块
│   │   │   ├── database.rs     # 数据库操作
│   │   │   └── services/       # 业务服务
│   │   └── Cargo.toml
│   └── gatemate-plugin-api/ # API 插件
└── package.json
```

## 架构设计

### 1. 分层架构

```
┌─────────────────────────────────────────────┐
│              前端 UI 层                      │
│  Vue 3 Components + Tauri Invoke API        │
├─────────────────────────────────────────────┤
│              Tauri 桥接层                    │
│  Commands / Events / Window Management      │
├─────────────────────────────────────────────┤
│              HTTP API 层                     │
│  Actix Web + REST Endpoints                 │
├─────────────────────────────────────────────┤
│              业务逻辑层                       │
│  Service Layer (Key, Project, Log, Route)   │
├─────────────────────────────────────────────┤
│              数据访问层                       │
│  SQLite + Database Abstraction              │
├─────────────────────────────────────────────┤
│              基础设施层                       │
│  Crypto, Keyring, Configuration             │
└─────────────────────────────────────────────┘
```

### 2. 组件职责

| 组件 | 职责 | 状态管理 |
|------|------|----------|
| DashboardView | 用量统计、图表展示 | 通过 props 接收数据 |
| KeysView | Key 列表、操作按钮 | 通过 props 接收 keys |
| LogsView | 调用日志列表、过滤 | 通过 props 接收 logs |
| SettingsModal | 系统设置面板 | 内部状态 + emit 更新 |
| CommandPalette | 全局快速搜索 | 内部状态 |

### 3. 数据流

```
前端用户操作
    ↓
Tauri Command (invoke)
    ↓
Rust Handler (gatemate-core)
    ↓
Service Layer
    ↓
Database Operation
    ↓
返回结果
    ↓
前端状态更新
    ↓
UI 渲染
```

### 4. 加密安全

- **API Key 加密**: 使用 AES-256-GCM 加密存储
- **密钥管理**: 主密钥存储在 OS Keychain
  - Windows: Credential Manager
  - macOS: Keychain
  - Linux: libsecret
- **数据隔离**: 每个项目独立的密钥空间

### 5. 路由策略

支持三种路由策略：

1. **轮询 (Round Robin)**: 均匀分配请求到所有可用 Key
2. **随机 (Random)**: 随机选择 Key
3. **智能路由 (Smart)**: 根据关键词/正则匹配选择特定 Key

### 6. 成本控制

- **日限额**: 每个 Key 的日花费上限
- **月限额**: 每个 Key 的月花费上限
- **项目预算**: 项目级别的月度预算
- **实时预警**: 用量达到 90%/100% 时发送通知

## 关键模块

### Server 模块

位于 `src-tauri/gatemate-core/src/server.rs`，提供 REST API：

- `/health` - 健康检查
- `/api/v1/keys` - Key 管理
- `/api/v1/projects` - 项目管理
- `/api/v1/logs` - 调用日志
- `/api/v1/route` - 路由策略

### Crypto 模块

位于 `src-tauri/gatemate-core/src/crypto.rs`，处理加密操作：

- `encrypt()` - AES-256-GCM 加密
- `decrypt()` - AES-256-GCM 解密
- `get_or_create_master_key()` - 获取或生成主密钥

### Database 模块

位于 `src-tauri/gatemate-core/src/db.rs`，数据库抽象层：

- 连接池管理
- 迁移执行
- 事务支持

## 部署与运行

### 开发环境

```bash
pnpm install
pnpm tauri dev
```

### 生产构建

```bash
pnpm tauri build
```

## 安全考虑

1. **API Key 存储**: 绝不明文存储，始终加密
2. **主密钥**: 使用系统 Keychain 保护
3. **CORS**: 严格限制允许的来源
4. **输入验证**: 后端对所有输入进行验证
5. **SQL 注入**: 使用参数化查询

## 未来优化方向

1. **微服务化**: 将路由服务独立为微服务
2. **分布式缓存**: 添加 Redis 缓存层
3. **多数据库支持**: 支持 PostgreSQL/MariaDB
4. **实时监控**: WebSocket 实时推送用量数据
5. **插件系统**: 支持第三方插件扩展