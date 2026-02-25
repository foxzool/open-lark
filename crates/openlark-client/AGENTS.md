# openlark-client Knowledge Base

**Crate**: High-Level Client  
**Purpose**: 统一客户端接口、服务注册表、meta 调用链

## OVERVIEW

OpenLark 高级客户端，提供统一的 `Client` 入口点和 ServiceRegistry 服务注册表模式。支持 feature 条件编译和流畅的链式调用体验。

## STRUCTURE

```
src/
├── lib.rs                    # Crate 入口，导出 prelude
├── client.rs                 # 主 Client 实现
├── builder.rs                # ClientBuilder 构建器
├── config.rs                 # 客户端配置
├── registry.rs               # ServiceRegistry 服务注册表
├── error.rs                  # 客户端错误处理
├── prelude.rs                # 常用类型预导入
├── services/                 # 服务管理
│   ├── manager.rs           # ServiceManager
│   └── types.rs             # 服务类型定义
├── ws_client/                # WebSocket 客户端（可选 feature）
└── compat/                   # 向后兼容层
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加新服务入口 | `src/client.rs` | 在 Client 结构体添加字段 |
| 修改构建器 | `src/builder.rs` | ClientBuilder 配置选项 |
| 服务注册 | `src/registry.rs` | ServiceRegistry 类型安全存储 |
| WebSocket | `src/ws_client/` | 需要 `websocket` feature |
| 错误处理 | `src/error.rs` | 客户端级别错误 |

## CONVENTIONS

### Meta 调用链
```rust
// 推荐的 API 访问方式
client.docs.ccm.drive.v1().file().upload(...).execute().await?;
client.communication.im.v1().message().send(...).execute().await?;
client.auth.app().access_token().get().execute().await?;
```

### Feature 条件编译
```rust
#[cfg(feature = "docs")]
pub docs: DocsService,

#[cfg(feature = "communication")]
pub communication: CommunicationService,
```

### ServiceRegistry 使用
```rust
// 检查服务是否可用
if client.registry().has_service("docs") { ... }

// 列出所有服务
for entry in client.registry().list_services() { ... }
```

## ANTI-PATTERNS

- ❌ 不要直接实例化业务 crate 的服务（使用 registry 或 meta 链）
- ❌ 不要在 client 中硬编码业务逻辑
- ❌ 不要破坏向后兼容性（compat/ 中的类型）

## NOTES

- **默认 Features**: `core-services` = auth + communication + docs + workflow
- **Prelude**: `use openlark_client::prelude::*;` 导入常用类型
- **环境变量**: 支持 `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET` 自动配置
