# OpenLark Auth API 修复实施计划

> **计划日期**: 2025-11-25
> **目标**: 补充passport/v1项目，实现100%认证API覆盖
> **工期**: 3天
> **优先级**: 高

---

## 🎯 修复目标

基于差距分析结果，需要补充**passport/v1**项目的2个API：

1. `session/query` - 批量获取用户登录信息
2. `session/logout` - 用户退出登录

这将使核心认证API覆盖率从当前的**83%** (10/12)提升到**100%** (12/12)。

---

## 📋 详细实施计划

### 阶段1：数据模型定义 (第1天)

#### 1.1 创建passport数据模型

**文件**: `crates/openlark-auth/src/models/passport.rs`

```rust
//! Passport相关数据模型 - 用户登录状态管理

use serde::{Deserialize, Serialize};

/// 会话查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionQueryRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 是否包含已离职用户
    pub include_resigned: Option<bool>,
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 用户ID
    pub user_id: String,
    /// 会话状态
    pub status: SessionStatus,
    /// 登录时间
    pub login_time: i64,
    /// 最后活跃时间
    pub last_active_time: i64,
    /// 登录IP
    pub login_ip: String,
    /// 用户代理
    pub user_agent: String,
    /// 设备信息
    pub device_info: Option<DeviceInfo>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型
    pub device_type: String,
    /// 设备ID
    pub device_id: String,
    /// 操作系统
    pub os: String,
    /// 浏览器
    pub browser: String,
}

/// 会话状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    /// 在线
    Online,
    /// 离线
    Offline,
    /// 已退出
    LoggedOut,
}

/// 会话查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionQueryResponse {
    /// 会话信息列表
    pub session_info: Vec<SessionInfo>,
    /// 总数
    pub total: i32,
}

/// 退出登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// 用户ID
    pub user_id: String,
    /// 是否退出所有设备
    pub logout_all_devices: Option<bool>,
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 结果码
    pub code: i32,
    /// 结果消息
    pub msg: String,
}
```

#### 1.2 更新模型导出

**文件**: `crates/openlark-auth/src/models/mod.rs`

```rust
// 添加passport模型导出
pub use passport::{
    DeviceInfo, LogoutRequest, LogoutResponse,
    SessionInfo, SessionQueryRequest, SessionQueryResponse, SessionStatus
};

// 子模块
pub mod passport;  // 新增
```

### 阶段2：passport项目架构 (第1-2天)

#### 2.1 创建passport项目入口

**文件**: `crates/openlark-auth/src/passport/mod.rs`

```rust
//! Passport项目 - 用户登录状态管理
//!
//! 提供用户登录状态查询和退出登录功能。

use std::sync::Arc;
use crate::models::{AuthConfig, AuthResult};

/// Passport项目
#[derive(Debug)]
pub struct PassportProject {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl PassportProject {
    /// 创建新的Passport项目实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取v1版本服务
    pub fn v1(&self) -> PassportV1Service {
        PassportV1Service::new(self.config.clone())
    }
}

/// Passport服务统一入口
pub struct PassportServices {
    pub config: Arc<AuthConfig>,
    pub passport: PassportProject,
}

impl PassportServices {
    pub fn new(config: crate::models::AuthConfig) -> Self {
        let config = Arc::new(config);
        Self {
            passport: PassportProject::new(config.clone()),
            config,
        }
    }
}

// 重新导出
pub use v1::PassportV1Service;

// v1版本模块
pub mod v1;
```

#### 2.2 实现v1版本服务

**文件**: `crates/openlark-auth/src/passport/v1/mod.rs`

```rust
//! Passport v1版本 - 用户登录状态管理

use std::sync::Arc;
use crate::models::{AuthConfig, AuthResult};

/// Passport v1服务
#[derive(Debug)]
pub struct PassportV1Service {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl PassportV1Service {
    /// 创建新的Passport v1服务实例
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 会话查询服务
    pub fn session(&self) -> SessionService {
        SessionService::new(self.config.clone(), self.client.clone())
    }
}

// 重新导出
pub use session::SessionService;

// 会话管理模块
pub mod session;
```

#### 2.3 实现Session服务

**文件**: `crates/openlark-auth/src/passport/v1/session.rs`

```rust
//! 会话管理服务 - Session Resource
//!
//! 提供用户登录状态查询和退出登录功能。

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{
    AuthConfig, AuthError, AuthResult,
    SessionQueryRequest, SessionQueryResponse,
    LogoutRequest, LogoutResponse
};

/// 会话管理服务
#[derive(Debug)]
pub struct SessionService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl SessionService {
    /// 创建新的会话管理服务
    pub fn new(config: Arc<AuthConfig>, client: reqwest::Client) -> Self {
        Self {
            config,
            client,
        }
    }

    /// 批量查询用户登录信息
    pub fn query(&self) -> SessionQueryBuilder {
        SessionQueryBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            user_ids: Vec::new(),
            include_resigned: None,
        }
    }

    /// 用户退出登录
    pub fn logout(&self) -> LogoutBuilder {
        LogoutBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            user_id: String::new(),
            logout_all_devices: None,
        }
    }
}

/// 会话查询构建器
#[derive(Debug)]
pub struct SessionQueryBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    user_ids: Vec<String>,
    include_resigned: Option<bool>,
}

impl SessionQueryBuilder {
    /// 添加用户ID
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置是否包含已离职用户
    pub fn include_resigned(mut self, include_resigned: bool) -> Self {
        self.include_resigned = Some(include_resigned);
        self
    }

    /// 发送查询请求
    pub async fn send(self) -> AuthResult<SessionQueryResponse> {
        let url = format!("{}/open-apis/passport/v1/sessions/query", self.config.base_url);

        let request_body = SessionQueryRequest {
            user_ids: self.user_ids,
            include_resigned: self.include_resigned,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let session_response: SessionQueryResponse = response.json().await?;
            Ok(session_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AuthError::APIError {
                code: status.as_u16() as i32,
                message: format!("HTTP {}: {}", status, error_text),
            })
        }
    }
}

/// 退出登录构建器
#[derive(Debug)]
pub struct LogoutBuilder {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
    user_id: String,
    logout_all_devices: Option<bool>,
}

impl LogoutBuilder {
    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置是否退出所有设备
    pub fn logout_all_devices(mut self, logout_all_devices: bool) -> Self {
        self.logout_all_devices = Some(logout_all_devices);
        self
    }

    /// 发送退出登录请求
    pub async fn send(self) -> AuthResult<LogoutResponse> {
        let url = format!("{}/open-apis/passport/v1/sessions/logout", self.config.base_url);

        let request_body = LogoutRequest {
            user_id: self.user_id,
            logout_all_devices: self.logout_all_devices,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "openlark-rust-sdk/0.1.0")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let logout_response: LogoutResponse = response.json().await?;
            Ok(logout_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AuthError::APIError {
                code: status.as_u16() as i32,
                message: format!("HTTP {}: {}", status, error_text),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_service_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = PassportV1Service::new(std::sync::Arc::new(config));

        // 测试服务创建
        let _session_service = service.session();
    }

    #[test]
    fn test_session_query_builder() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let service = PassportV1Service::new(std::sync::Arc::new(config));

        let builder = service.session()
            .query()
            .user_ids(vec!["user1".to_string(), "user2".to_string()])
            .include_resigned(true);

        // 构建器应该正确设置参数
        assert_eq!(builder.user_ids.len(), 2);
        assert_eq!(builder.include_resigned, Some(true));
    }
}
```

### 阶段3：集成到主服务 (第2天)

#### 3.1 更新主库导出

**文件**: `crates/openlark-auth/src/lib.rs`

```rust
// 在现有导入中添加
pub mod passport;

// 重新导出
pub use passport::{PassportProject, PassportServices};

// 在AuthServices中添加passport
impl AuthServices {
    pub fn new(config: crate::models::AuthConfig) -> Self {
        let config = std::sync::Arc::new(config);

        Self {
            auth: AuthProject::new(config.clone()),
            authen: AuthenProject::new(config.clone()),
            oauth: OauthProject::new(config.clone()),
            passport: PassportProject::new(config.clone()),  // 新增
            config,
        }
    }
}
```

#### 3.2 更新prelude

**文件**: `crates/openlark-auth/src/lib.rs`

```rust
/// 预导出模块
pub mod prelude {
    pub use super::{AuthProject, AuthResult, AuthServices, AuthenProject, OauthProject};

    // 新增passport导出
    pub use super::passport::{PassportProject, PassportServices};

    pub use super::auth::*;
    pub use super::authen::*;
    pub use super::models::*;
    pub use super::oauth::*;
    pub use super::passport::*;  // 新增
}
```

### 阶段4：测试验证 (第3天)

#### 4.1 创建集成测试

**文件**: `tests/passport_integration_tests.rs`

```rust
//! Passport集成测试

use openlark_auth::prelude::*;

#[tokio::test]
async fn test_passport_session_query() {
    let config = AuthConfig::new("test_app_id", "test_app_secret");
    let passport_services = PassportServices::new(config);

    // 测试会话查询
    let result = passport_services.passport.v1().session()
        .query()
        .user_ids(vec!["user1".to_string()])
        .include_resigned(false)
        .send()
        .await;

    // 验证结果结构
    match result {
        Ok(response) => {
            assert!(!response.session_info.is_empty());
            assert!(response.total >= 0);
        }
        Err(_) => {
            // 在没有真实token的情况下，网络错误是预期的
        }
    }
}

#[tokio::test]
async fn test_passport_logout() {
    let config = AuthConfig::new("test_app_id", "test_app_secret");
    let passport_services = PassportServices::new(config);

    // 测试退出登录
    let result = passport_services.passport.v1().session()
        .logout()
        .user_id("test_user")
        .logout_all_devices(false)
        .send()
        .await;

    // 验证结果结构
    match result {
        Ok(response) => {
            assert_eq!(response.code, 0);
        }
        Err(_) => {
            // 在没有真实token的情况下，网络错误是预期的
        }
    }
}
```

#### 4.2 创建示例代码

**文件**: `examples/passport_demo.rs`

```rust
//! Passport功能演示
//!
//! 展示用户登录状态管理功能

use openlark_auth::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Passport功能演示");
    println!("=================");

    // 配置应用信息
    let config = AuthConfig::new("app_id", "app_secret");
    let passport = PassportServices::new(config);

    // 演示会话查询
    println!("\n📋 会话查询演示");
    println!("================");

    let user_ids = vec![
        "user_001".to_string(),
        "user_002".to_string(),
        "user_003".to_string(),
    ];

    match passport.passport.v1().session()
        .query()
        .user_ids(user_ids.clone())
        .include_resigned(true)
        .send()
        .await
    {
        Ok(response) => {
            println!("✅ 会话查询成功");
            println!("   查询用户数: {}", user_ids.len());
            println!("   返回会话数: {}", response.session_info.len());
            println!("   总计: {}", response.total);

            // 显示每个会话信息
            for (index, session) in response.session_info.iter().enumerate() {
                println!("   会话 {}: 用户ID={}, 状态={:?}",
                         index + 1,
                         session.user_id,
                         session.status);
                println!("     登录时间: {}", session.login_time);
                println!("     最后活跃: {}", session.last_active_time);
                println!("     登录IP: {}", session.login_ip);
            }
        }
        Err(e) => {
            println!("❌ 会话查询失败: {}", e);
        }
    }

    // 演示退出登录
    println!("\n📋 退出登录演示");
    println!("================");

    match passport.passport.v1().session()
        .logout()
        .user_id("user_001")
        .logout_all_devices(false)
        .send()
        .await
    {
        Ok(response) => {
            println!("✅ 退出登录成功");
            println!("   结果码: {}", response.code);
            println!("   结果消息: {}", response.msg);
        }
        Err(e) => {
            println!("❌ 退出登录失败: {}", e);
        }
    }

    Ok(())
}
```

---

## 🔍 质量保证检查

### 编译检查
```bash
cargo check -p openlark-auth
cargo test -p openlark-auth
cargo clippy -p openlark-auth
```

### 文档检查
- 确保所有公共API有文档注释
- 验证示例代码可正常运行
- 检查API文档完整性

### 性能验证
- 确保新API不影响现有性能
- 验证异步操作正确性
- 检查内存使用情况

---

## 📅 时间安排

| 阶段 | 任务 | 预估时间 | 状态 |
|------|------|----------|------|
| 第1天 | 数据模型定义 | 0.5天 | 计划 |
| 第1天 | passport项目架构 | 0.5天 | 计划 |
| 第2天 | Session服务实现 | 1天 | 计划 |
| 第2天 | 主服务集成 | 0.5天 | 计划 |
| 第3天 | 测试验证 | 0.5天 | 计划 |
| 第3天 | 示例代码 | 0.5天 | 计划 |

**总工期**: 3天

---

## 🎯 预期成果

1. **API覆盖率**: 从83%提升到100%
2. **功能完整性**: 实现所有认证API功能
3. **代码质量**: 保持现有高质量标准
4. **向后兼容**: 不影响现有API使用
5. **文档完整**: 提供完整的使用示例

---

## ✅ 完成标准

- [ ] 所有passport API正常工作
- [ ] 测试覆盖率达到95%+
- [ ] 示例代码可以正常运行
- [ ] 文档更新完整
- [ ] 代码编译无警告
- [ ] 性能测试通过