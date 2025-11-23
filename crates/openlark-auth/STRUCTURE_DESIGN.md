# openlark-auth 目录结构设计

> **设计目标**: 基于 openlark-core/auth 建立完整的基础认证模块
> **职责**: 提供令牌管理、缓存、验证、刷新等核心认证功能
> **依赖**: openlark-core (基础设施)

## 目录结构

```
crates/openlark-auth/
├── Cargo.toml                    # 项目配置
├── README.md                     # 模块说明
├── CHANGELOG.md                  # 变更记录
├── LICENSE                       # 许可证
├── CLAUDE.md                     # AI 辅助文档
├── src/                          # 源代码目录
│   ├── lib.rs                    # 库入口点
│   ├── prelude.rs                # 预导出模块
│   │
│   ├── auth/                     # 核心认证模块 (从 core 迁移)
│   │   ├── mod.rs                # 模块导出
│   │   ├── token.rs              # 令牌类型和管理
│   │   ├── cache.rs              # 缓存实现
│   │   ├── refresh.rs            # 令牌刷新机制
│   │   ├── validator.rs          # 令牌验证
│   │   └── types.rs              # 类型定义 (新增)
│   │
│   ├── client/                   # 客户端层
│   │   ├── mod.rs                # 客户端模块
│   │   ├── auth_client.rs        # 认证客户端
│   │   ├── token_client.rs       # 令牌客户端
│   │   └── config.rs             # 认证配置
│   │
│   ├── endpoints/                # API端点
│   │   ├── mod.rs                # 端点模块
│   │   ├── auth.rs              # 认证端点 (从 core 迁移)
│   │   ├── token.rs             # 令牌端点
│   │   └── oauth.rs              # OAuth端点 (新增)
│   │
│   ├── managers/                 # 管理器层
│   │   ├── mod.rs                # 管理器模块
│   │   ├── token_manager.rs      # 令牌管理器 (从 core 迁移)
│   │   ├── cache_manager.rs      # 缓存管理器
│   │   └── refresh_manager.rs    # 刷新管理器
│   │
│   ├── utils/                    # 工具模块
│   │   ├── mod.rs                # 工具模块
│   │   ├── crypto.rs             # 加密工具
│   │   ├── validator.rs          # 验证工具
│   │   └── time.rs               # 时间工具
│   │
│   ├── error.rs                  # 错误定义
│   └── config.rs                 # 配置管理
│
├── tests/                        # 测试目录
│   ├── mod.rs                    # 测试模块导出
│   ├── unit/                     # 单元测试
│   │   ├── auth_tests.rs         # 认证功能测试
│   │   ├── token_tests.rs        # 令牌功能测试
│   │   ├── cache_tests.rs        # 缓存功能测试
│   │   ├── refresh_tests.rs      # 刷新功能测试
│   │   └── validator_tests.rs    # 验证功能测试
│   │
│   ├── integration/              # 集成测试
│   │   ├── auth_integration_tests.rs      # 认证集成测试
│   │   ├── token_integration_tests.rs     # 令牌集成测试
│   │   └── end_to_end_tests.rs              # 端到端测试
│   │
│   └── fixtures/                # 测试数据
│       ├── mod.rs
│       ├── auth_fixtures.rs
│       └── token_fixtures.rs
│
├── examples/                     # 示例代码
│   ├── basic_auth.rs             # 基础认证示例
│   ├── token_management.rs       # 令牌管理示例
│   ├── oauth_flow.rs             # OAuth流程示例
│   └── custom_cache.rs           # 自定义缓存示例
│
└── benches/                      # 性能基准测试
    ├── auth_bench.rs              # 认证性能测试
    ├── token_bench.rs             # 令牌性能测试
    └── cache_bench.rs             # 缓存性能测试
```

## 模块职责说明

### 核心认证模块 (`src/auth/`)
从 `openlark-core/auth` 迁移的核心功能：
- **token.rs**: 令牌类型定义、管理接口
- **cache.rs**: 缓存接口和实现
- **refresh.rs**: 自动刷新机制
- **validator.rs**: 令牌验证逻辑
- **types.rs**: 公共类型定义

### 客户端层 (`src/client/`)
提供高级客户端接口：
- **auth_client.rs**: 认证服务客户端
- **token_client.rs**: 令牌管理客户端
- **config.rs**: 认证配置管理

### 端点层 (`src/endpoints/`)
API端点定义和常量：
- **auth.rs**: 从 core 迁移的认证端点
- **token.rs**: 令牌相关端点
- **oauth.rs**: 新增 OAuth 端点

### 管理器层 (`src/managers/`)
业务逻辑管理：
- **token_manager.rs**: 从 core 迁移的令牌管理器
- **cache_manager.rs**: 缓存管理器
- **refresh_manager.rs**: 刷新管理器

## 依赖关系

### 外部依赖
```toml
[dependencies]
# 核心基础设施
openlark-core = { workspace = true }

# 异步运行时
tokio = { workspace = true, features = ["full"] }
async-trait = { workspace = true }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 时间处理
chrono = { workspace = true, features = ["serde"] }

# 错误处理
thiserror = { workspace = true }
anyhow = { workspace = true }

# 日志
tracing = { workspace = true }

# 加密和验证
ring = { version = "0.17", optional = true }
sha2 = { version = "0.10", optional = true }
hmac = { version = "0.12", optional = true }

# UUID
uuid = { workspace = true, features = ["v4", "serde"] }

[features]
default = ["token-management", "cache"]
token-management = []
cache = ["token-management"]
oauth = []
encryption = ["ring", "sha2", "hmac"]
```

### 内部依赖关系
```
openlark-security (业务层) → openlark-auth (基础认证层) → openlark-core (基础设施)
```

## API 接口设计

### 核心接口
```rust
// 令牌管理接口
#[async_trait]
pub trait TokenManager: Send + Sync {
    async fn get_access_token(&self, request: GetTokenRequest) -> AuthResult<AccessToken>;
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<AccessToken>;
    async fn validate_token(&self, token: &str) -> AuthResult<TokenValidationResult>;
}

// 缓存接口
#[async_trait]
pub trait TokenCache: Send + Sync {
    async fn get(&self, key: &str) -> Option<AccessToken>;
    async fn set(&self, key: &str, token: &AccessToken, ttl: Duration);
    async fn invalidate(&self, key: &str);
    async fn clear(&self);
}

// 认证客户端
pub struct AuthClient {
    token_manager: Arc<dyn TokenManager>,
    cache: Arc<dyn TokenCache>,
    config: AuthConfig,
}
```

## 向后兼容策略

### 在 openlark-core 中保留兼容性导出
```rust
// openlark-core/src/lib.rs
#[deprecated(note = "Use openlark-auth instead")]
pub use openlark_auth::*;

// 提供过渡期的预导出
pub mod prelude {
    #[deprecated(note = "Use openlark_auth::prelude instead")]
    pub use openlark_auth::*;
}
```

## 测试策略

### 测试覆盖率目标
- 单元测试：90%+
- 集成测试：80%+
- 性能测试：关键路径100%

### 测试重点
- 令牌管理功能正确性
- 缓存机制可靠性
- 自动刷新机制稳定性
- 错误处理完整性
- 性能指标达标

## 迁移策略

### 从 openlark-core 迁移的文件
1. **直接迁移**：保持功能完全一致
2. **依赖适配**：更新模块内依赖
3. **接口优化**：改进公共接口设计
4. **测试适配**：更新测试用例

### 新增功能
1. **OAuth支持**：完整的OAuth流程
2. **高级缓存**：多级缓存策略
3. **性能优化**：批量操作和并发处理
4. **监控集成**：指标和日志

---

**设计原则**：
- 单一职责：每个模块职责清晰
- 开放封闭：易于扩展新功能
- 依赖倒置：依赖抽象而非具体实现
- 接口隔离：稳定的公共接口

**更新时间**: 2025-11-23
**设计者**: OpenLark Team
**版本**: 1.0