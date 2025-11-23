# openlark-auth 重构计划

> **目标**: 将当前 47 行的基础模块重构为符合 P0 标准的安全认证模块
> **优先级**: P0 - 核心安全模块
> **当前覆盖率**: 13.6% (6/44 APIs)
> **目标覆盖率**: 100% (44/44 APIs)
> **预计工期**: 4-6 周

## 1. 现状分析

### 1.1 当前模块状态

**文件结构**:
```
crates/openlark-auth/
├── src/
│   └── lib.rs          # 47 行，仅重新导出 openlark_core 功能
├── Cargo.toml         # 基础配置，无功能标志
└── README.md          # 基础文档，标记为"开发中"
```

**主要问题**:
- ❌ 仅为基础重新导出模块，缺乏独立功能
- ❌ API 覆盖率严重不足 (6/44 = 13.6%)
- ❌ 缺少标准的模块目录结构
- ❌ 无独立测试框架
- ❌ 缺少性能基准和文档
- ❌ 不符合 P0 安全模块标准

### 1.2 合规性缺口

基于 `docs/module-mapping.md` 要求：

| 模块类别 | 需要API数量 | 当前实现 | 缺口 | 优先级 |
|---------|------------|---------|------|--------|
| 认证服务 (Authentication) | 11 | 4 | 7 | P0 |
| 访问控制 (ACS) | 17 | 0 | 17 | P0 |
| 安全与合规 (Security) | 8 | 2 | 6 | P0 |
| 权限验证 (Permission) | 8 | 0 | 8 | P0 |
| **总计** | **44** | **6** | **38** | **P0** |

## 2. 重构目标

### 2.1 模块重命名

**从**: `openlark-auth`
**到**: `openlark-security`

**理由**:
- 更好地反映完整的安全功能范围
- 包含认证、授权、合规等多个维度
- 符合企业级安全模块命名规范

### 2.2 目录结构重设计

```
crates/openlark-security/
├── src/
│   ├── lib.rs                    # 模块导出
│   ├── prelude.rs                # 常用导出
│   ├── config.rs                 # 配置管理
│   ├── error.rs                  # 错误定义
│   ├── client.rs                 # 客户端实现
│   │
│   ├── auth/                     # 认证服务
│   │   ├── mod.rs
│   │   ├── models.rs             # 认证数据模型
│   │   ├── v1/                   # API v1 版本
│   │   │   ├── mod.rs
│   │   │   ├── app_access_token.rs
│   │   │   ├── user_access_token.rs
│   │   │   ├── app_ticket.rs
│   │   │   ├── tenant_access_token.rs
│   │   │   └── token_info.rs
│   │   └── v2/                   # API v2 版本
│   │       ├── mod.rs
│   │       └── ...
│   │
│   ├── acs/                      # 访问控制服务
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── v1/
│   │   │   ├── mod.rs
│   │   │   ├── permission.rs
│   │   │   ├── role.rs
│   │   │   ├── policy.rs
│   │   │   └── resource_access.rs
│   │   └── v2/
│   │
│   ├── security/                 # 安全与合规
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── v1/
│   │   │   ├── mod.rs
│   │   │   ├── audit_log.rs
│   │   │   ├── security_event.rs
│   │   │   ├── compliance.rs
│   │   │   └── risk_assessment.rs
│   │   └── v2/
│   │
│   ├── permission/               # 权限验证
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── v1/
│   │   │   ├── mod.rs
│   │   │   ├── check_permission.rs
│   │   │   ├── batch_permission.rs
│   │   │   ├── permission_list.rs
│   │   │   └── user_permission.rs
│   │   └── v2/
│   │
│   └── utils/                    # 工具模块
│       ├── mod.rs
│       ├── crypto.rs             # 加密工具
│       ├── validator.rs          # 验证工具
│       └── cache.rs              # 缓存工具
│
├── tests/                        # 集成测试
│   ├── mod.rs
│   ├── auth_tests.rs
│   ├── acs_tests.rs
│   ├── security_tests.rs
│   └── permission_tests.rs
│
├── benches/                      # 性能基准
│   ├── auth_bench.rs
│   ├── acs_bench.rs
│   └── security_bench.rs
│
├── examples/                     # 示例代码
│   ├── auth_example.rs
│   ├── acs_example.rs
│   ├── security_example.rs
│   └── full_integration.rs
│
├── Cargo.toml                    # 依赖配置
├── README.md                     # 模块文档
├── CHANGELOG.md                  # 变更记录
└── CLAUDE.md                     # AI 辅助文档
```

## 3. API 实现计划

### 3.1 认证服务 (Authentication) - 11 APIs

**已实现 (4 APIs)**:
- ✅ `app_access_token` - 应用访问令牌
- ✅ `user_access_token` - 用户访问令牌
- ✅ `app_ticket` - 应用票据
- ✅ `tenant_access_token` - 租户访问令牌

**待实现 (7 APIs)**:
- [ ] `token_info` - 令牌信息查询
- [ ] `token_refresh` - 令牌刷新
- [ ] `token_revoke` - 令牌撤销
- [ ] `token_validation` - 令牌验证
- [ ] `pre_auth_code` - 预授权码
- [ ] `oauth_redirect` - OAuth 重定向
- [ ] `auto_login` - 自动登录

### 3.2 访问控制 (Access Control) - 17 APIs

**待实现 (17 APIs)**:
- [ ] `permission_check` - 权限检查
- [ ] `permission_batch_check` - 批量权限检查
- [ ] `permission_list` - 权限列表
- [ ] `permission_create` - 权限创建
- [ ] `permission_update` - 权限更新
- [ ] `permission_delete` - 权限删除
- [ ] `role_create` - 角色创建
- [ ] `role_update` - 角色更新
- [ ] `role_delete` - 角色删除
- [ ] `role_list` - 角色列表
- [ ] `role_assign` - 角色分配
- [ ] `role_unassign` - 角色取消分配
- [ ] `policy_create` - 策略创建
- [ ] `policy_update` - 策略更新
- [ ] `policy_delete` - 策略删除
- [ ] `resource_access_grant` - 资源访问授权
- [ ] `resource_access_revoke` - 资源访问撤销

### 3.3 安全与合规 (Security & Compliance) - 8 APIs

**已实现 (2 APIs)**:
- ✅ `audit_log` - 审计日志
- ✅ `security_event` - 安全事件

**待实现 (6 APIs)**:
- [ ] `compliance_check` - 合规性检查
- [ ] `risk_assessment` - 风险评估
- [ ] `security_policy` - 安全策略
- [ ] `data_classification` - 数据分类
- [ ] `privacy_setting` - 隐私设置
- [ ] `security_dashboard` - 安全仪表板

### 3.4 权限验证 (Permission Validation) - 8 APIs

**待实现 (8 APIs)**:
- [ ] `user_permission_check` - 用户权限检查
- [ ] `batch_user_permission` - 批量用户权限
- [ ] `resource_permission` - 资源权限
- [ ] `permission_hierarchy` - 权限层级
- [ ] `permission_inheritance` - 权限继承
- [ ] `temporary_permission` - 临时权限
- [ ] `permission_expiry` - 权限过期
- [ ] `permission_audit` - 权限审计

## 4. 技术实现方案

### 4.1 Cargo.toml 配置

```toml
[package]
name = "openlark-security"
version = "0.1.0"
edition = "2021"
authors = ["OpenLark Team"]
description = "飞书开放平台安全认证服务模块"
license = "MIT"
repository = "https://github.com/foxzool/open-lark"
keywords = ["lark", "feishu", "security", "auth", "acs"]
categories = ["api-bindings", "authentication"]

[dependencies]
# 核心依赖
openlark-core = { workspace = true }
openlark-protocol = { workspace = true, optional = true }

# 序列化
serde = { workspace = true }
serde_json = { workspace = true }

# 异步运行时
tokio = { workspace = true, features = ["full"] }
async-trait = { workspace = true }

# 加密和安全
ring = "0.17"
sha2 = "0.10"
hmac = "0.12"

# 错误处理
thiserror = { workspace = true }
anyhow = { workspace = true }

# 日志和追踪
tracing = { workspace = true }

# 时间处理
chrono = { workspace = true, features = ["serde"] }

# 缓存
redis = { version = "0.24", optional = true }

# HTTP客户端 (特定需求)
reqwest = { workspace = true, optional = true }

# UUID
uuid = { workspace = true, features = ["v4", "serde"] }

[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"

[features]
default = ["auth", "token-management"]

# 核心功能模块
auth = []
acs = []
security = []
permission = []

# 令牌管理
token-management = ["auth"]
token-cache = ["redis"]

# OAuth 支持
oauth = ["auth", "reqwest"]

# WebSocket 事件支持
websocket = ["openlark-protocol"]

# 合规性支持
compliance = ["security"]
audit = ["security"]

# 完整功能集
full = ["auth", "acs", "security", "permission", "token-management", "oauth", "compliance", "audit"]

# 企业级功能
enterprise = ["full", "compliance", "audit"]
```

### 4.2 核心接口设计

```rust
// src/lib.rs
//! # OpenLark Security 安全认证模块
//!
//! 提供飞书开放平台的完整安全认证服务，包括：
//! - 身份认证 (Authentication)
//! - 访问控制 (Access Control)
//! - 安全合规 (Security & Compliance)
//! - 权限验证 (Permission Validation)

pub mod auth;
pub mod acs;
pub mod security;
pub mod permission;
pub mod config;
pub mod error;
pub mod client;
pub mod utils;

// 重新导出核心类型
pub use openlark_core::{
    app_ticket_manager::{AppTicketManager, apply_app_ticket},
    token_manager::{TokenManager, TokenMetrics, PreheatingConfig},
    cache::{CacheEntry, QuickCache},
    constants::{AppType, FEISHU_BASE_URL, LARK_BASE_URL},
    req_option::RequestOption,
};

// 本地类型导出
pub use config::{SecurityConfig, AuthConfig, ACSConfig};
pub use error::{SecurityError, SecurityResult};
pub use client::{SecurityClient, SecurityClientBuilder};

/// 预导出模块
pub mod prelude {
    pub use crate::{
        SecurityClient, SecurityClientBuilder, SecurityConfig, SecurityResult,
        auth::AuthService,
        acs::AccessControlService,
        security::SecurityService,
        permission::PermissionService,
    };
}

/// 安全服务客户端
pub struct SecurityService {
    client: SecurityClient,
    config: SecurityConfig,
}

impl SecurityService {
    /// 创建新的安全服务实例
    pub fn new(config: SecurityConfig) -> SecurityResult<Self> {
        let client = SecurityClient::builder()
            .config(config.clone())
            .build()?;

        Ok(Self { client, config })
    }

    /// 获取认证服务
    #[cfg(feature = "auth")]
    pub fn auth(&self) -> auth::AuthService {
        auth::AuthService::new(self.client.clone())
    }

    /// 获取访问控制服务
    #[cfg(feature = "acs")]
    pub fn acs(&self) -> acs::AccessControlService {
        acs::AccessControlService::new(self.client.clone())
    }

    /// 获取安全服务
    #[cfg(feature = "security")]
    pub fn security(&self) -> security::SecurityService {
        security::SecurityService::new(self.client.clone())
    }

    /// 获取权限验证服务
    #[cfg(feature = "permission")]
    pub fn permission(&self) -> permission::PermissionService {
        permission::PermissionService::new(self.client.clone())
    }
}
```

### 4.3 客户端实现

```rust
// src/client.rs
use async_trait::async_trait;
use openlark_core::{CoreClient, SDKResult};
use std::sync::Arc;

/// 安全服务客户端
#[derive(Debug, Clone)]
pub struct SecurityClient {
    core_client: Arc<CoreClient>,
    config: SecurityConfig,
}

/// 安全客户端构建器
#[derive(Debug)]
pub struct SecurityClientBuilder {
    config: Option<SecurityConfig>,
}

impl SecurityClientBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn config(mut self, config: SecurityConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> SecurityResult<SecurityClient> {
        let config = self.config.ok_or_else(|| {
            SecurityError::InvalidConfig("缺少配置信息".to_string())
        })?;

        let core_client = Arc::new(CoreClient::new(config.into_core_config())?);

        Ok(SecurityClient {
            core_client,
            config,
        })
    }
}

impl Default for SecurityClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityClient {
    /// 创建新的构建器
    pub fn builder() -> SecurityClientBuilder {
        SecurityClientBuilder::new()
    }

    /// 获取核心客户端引用
    pub fn core_client(&self) -> &Arc<CoreClient> {
        &self.core_client
    }

    /// 获取配置引用
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }
}
```

## 5. 实现时间线

### 第1周：基础架构
- [x] 创建新的目录结构
- [x] 配置 Cargo.toml
- [x] 实现核心客户端和配置
- [x] 建立错误处理体系
- [x] 创建基础测试框架

### 第2周：认证服务实现
- [ ] 完成认证服务 7 个缺失 API
- [ ] 实现令牌管理和缓存
- [ ] 添加 OAuth 支持
- [ ] 编写认证服务测试

### 第3周：访问控制实现
- [ ] 实现 17 个 ACS API
- [ ] 角色和策略管理
- [ ] 权限检查机制
- [ ] 编写 ACS 测试

### 第4周：安全与合规实现
- [ ] 完成安全服务 6 个缺失 API
- [ ] 实现合规性检查
- [ ] 风险评估功能
- [ ] 编写安全测试

### 第5周：权限验证实现
- [ ] 实现 8 个权限验证 API
- [ ] 权限层级和继承
- [ ] 批量权限处理
- [ ] 编写权限测试

### 第6周：集成和优化
- [ ] 性能基准测试
- [ ] 文档完善
- [ ] 示例代码
- [ ] 最终集成测试

## 6. 质量保证

### 6.1 测试覆盖率目标

| 模块 | 目标覆盖率 | 测试类型 |
|------|-----------|----------|
| auth | 90%+ | 单元测试 + 集成测试 |
| acs | 85%+ | 单元测试 + 集成测试 |
| security | 85%+ | 单元测试 + 集成测试 |
| permission | 85%+ | 单元测试 + 集成测试 |
| 整体 | 87%+ | 全覆盖 |

### 6.2 性能指标

| 指标 | 目标 | 测试方法 |
|------|------|----------|
| API 响应时间 | < 100ms (P95) | 基准测试 |
| 内存使用 | < 50MB (常驻) | 内存分析 |
| 并发处理 | > 1000 QPS | 负载测试 |
| 错误率 | < 0.1% | 压力测试 |

### 6.3 安全要求

- ✅ 所有敏感数据加密存储
- ✅ 令牌安全传输 (HTTPS)
- ✅ 防止重放攻击
- ✅ 输入验证和清理
- ✅ 审计日志记录
- ✅ 权限最小化原则

## 7. 迁移指南

### 7.1 从 openlark-auth 迁移

**旧代码**:
```rust
use openlark_auth::{TokenManager, AuthService};

let auth_service = AuthService::new(config)?;
let token = auth_service.get_app_access_token().await?;
```

**新代码**:
```rust
use openlark_security::prelude::*;

let security_service = SecurityService::new(config)?;
let auth_service = security_service.auth();
let token = auth_service.app_access_token().await?;
```

### 7.2 向后兼容性

为了保证平滑迁移，新模块将：
1. 保留原有 API 的兼容性包装
2. 提供迁移工具和文档
3. 支持渐进式升级路径
4. 维护 6 个月的兼容期

## 8. 风险评估与缓解

### 8.1 技术风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| API 变更导致不兼容 | 高 | 中 | 严格版本控制，兼容性测试 |
| 性能回归 | 中 | 低 | 性能基准，持续监控 |
| 安全漏洞 | 高 | 低 | 安全审计，渗透测试 |

### 8.2 项目风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| 工期延误 | 中 | 中 | 分阶段交付，优先核心功能 |
| 资源不足 | 高 | 低 | 合理分配，外部支持 |
| 需求变更 | 中 | 中 | 敏捷开发，快速响应 |

## 9. 成功标准

### 9.1 功能完整性
- ✅ 44 个 API 100% 实现
- ✅ 所有功能模块正常工作
- ✅ 通过全量测试套件

### 9.2 性能标准
- ✅ 响应时间 < 100ms (P95)
- ✅ 内存使用 < 50MB (常驻)
- ✅ 并发处理 > 1000 QPS

### 9.3 质量标准
- ✅ 测试覆盖率 > 87%
- ✅ 零编译警告
- ✅ 完整的 API 文档
- ✅ 通过安全审计

### 9.4 用户体验
- ✅ 简洁易用的 API
- ✅ 完整的示例代码
- ✅ 详细的迁移指南
- ✅ 活跃的社区支持

---

**文档版本**: 1.0
**最后更新**: 2025-11-23
**负责人**: OpenLark 开发团队
**审核状态**: 待审核

---

> 🎯 **下一步行动**: 开始创建新的 `openlark-security` 模块结构，并实现第一个认证 API。