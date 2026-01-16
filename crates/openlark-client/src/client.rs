//! OpenLark Client - 全新简化架构
//!
//! 极简设计：仅保留 meta 链式字段访问（单入口，KISS）

use crate::registry::ServiceRegistry;
use crate::{
    core_config::{build_base_core_config, build_core_config_with_default_token_provider},
    error::{with_context, with_operation_context},
    traits::LarkClient,
    Config, DefaultServiceRegistry, Result, ServiceMetadata, ServiceStatus,
};
use openlark_core::error::ErrorTrait;
use std::sync::Arc;

/// 🔐 认证 meta 入口：`client.auth.app / client.auth.user / client.auth.oauth`
#[cfg(feature = "auth")]
#[derive(Debug, Clone)]
pub struct AuthClient {
    pub app: openlark_auth::AuthService,
    pub user: openlark_auth::AuthenService,
    pub oauth: openlark_auth::OAuthService,
}

#[cfg(feature = "auth")]
impl AuthClient {
    fn new(config: openlark_core::config::Config) -> Self {
        Self {
            app: openlark_auth::AuthService::new(config.clone()),
            user: openlark_auth::AuthenService::new(config.clone()),
            oauth: openlark_auth::OAuthService::new(config),
        }
    }
}

/// 🚀 OpenLark客户端 - 极简设计
///
/// # 特性
/// - 零配置启动：`Client::from_env()`
/// - 单入口：meta 链式字段访问（`client.docs/...`）
/// - 编译时feature优化
/// - 高性能异步
/// - 现代化错误处理
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // 从环境变量创建客户端
///     let client = Client::from_env()?;
///
///     // meta 链式入口（需要对应 feature）
///     // - 通讯：client.communication.im...
///     // - 文档：client.docs.ccm...
///     // - 认证：client.auth.app / client.auth.user / client.auth.oauth
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    /// 客户端配置
    config: Arc<Config>,
    /// 服务注册表
    registry: Arc<DefaultServiceRegistry>,
    /// 底层 core 配置（供各 meta client 复用）
    core_config: openlark_core::config::Config,

    /// CardKit meta 调用链：client.cardkit.v1.card.create(...)
    #[cfg(feature = "cardkit")]
    pub cardkit: openlark_cardkit::CardkitClient,

    /// Auth meta 调用链入口：client.auth.app / client.auth.user / client.auth.oauth
    #[cfg(feature = "auth")]
    pub auth: AuthClient,

    /// Docs meta 调用链入口：client.docs.ccm / client.docs.base ...
    #[cfg(feature = "docs")]
    pub docs: openlark_docs::DocsClient,

    /// Communication meta 调用链入口：client.communication.im / client.communication.contact ...
    #[cfg(feature = "communication")]
    pub communication: openlark_communication::CommunicationClient,

    /// Meeting meta 调用链入口：client.meeting.vc.v1.room.create() ...
    #[cfg(feature = "meeting")]
    pub meeting: openlark_meeting::MeetingClient,
}

impl Client {
    /// 🔥 从环境变量创建客户端
    ///
    /// # 环境变量
    /// ```bash
    /// export OPENLARK_APP_ID=your_app_id
    /// export OPENLARK_APP_SECRET=your_app_secret
    /// export OPENLARK_BASE_URL=https://open.feishu.cn  # 可选
    /// ```
    ///
    /// # 返回值
    /// 返回配置好的客户端实例或错误
    ///
    /// # 示例
    /// ```rust,no_run
    /// use openlark_client::Client;
    ///
    /// let _client = Client::from_env();
    /// ```
    pub fn from_env() -> Result<Self> {
        Self::builder().from_env().build()
    }

    /// 🏗️ 创建构建器
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// 🔧 获取客户端配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 📋 获取服务注册表
    pub fn registry(&self) -> &DefaultServiceRegistry {
        &self.registry
    }

    /// 🔧 获取底层 core 配置（高级用法/调试用）
    pub fn core_config(&self) -> &openlark_core::config::Config {
        &self.core_config
    }

    /// ✅ 检查客户端是否已正确配置
    pub fn is_configured(&self) -> bool {
        !self.config.app_id.is_empty() && !self.config.app_secret.is_empty()
    }

    /// 🆕 创建带有自定义配置的客户端
    pub fn with_config(config: Config) -> Result<Self> {
        let validation_result = config.validate();
        if let Err(err) = validation_result {
            return with_context(Err(err), "operation", "Client::with_config");
        }

        let config = Arc::new(config);
        let mut registry = DefaultServiceRegistry::new();

        // 加载启用的服务
        let load_result = load_enabled_services(&config, &mut registry);
        if let Err(err) = load_result {
            return with_operation_context(Err(err), "Client::with_config", "service_loading");
        }

        let registry = Arc::new(registry);

        let base_core_config = build_base_core_config(config.as_ref());
        let core_config = build_core_config_with_default_token_provider(config.as_ref());

        #[cfg(feature = "cardkit")]
        let cardkit = openlark_cardkit::CardkitClient::new(core_config.clone());

        #[cfg(feature = "auth")]
        let auth = AuthClient::new(base_core_config.clone());

        #[cfg(feature = "docs")]
        let docs = openlark_docs::DocsClient::new(core_config.clone());

        #[cfg(feature = "communication")]
        let communication = openlark_communication::CommunicationClient::new(core_config.clone());

        #[cfg(feature = "meeting")]
        let meeting = openlark_meeting::MeetingClient::new(core_config.clone());

        Ok(Client {
            config,
            registry,
            core_config: core_config.clone(),
            #[cfg(feature = "cardkit")]
            cardkit,
            #[cfg(feature = "auth")]
            auth,
            #[cfg(feature = "docs")]
            docs,
            #[cfg(feature = "communication")]
            communication,
            #[cfg(feature = "meeting")]
            meeting,
        })
    }

    /// 🔧 执行带有错误上下文的操作
    pub async fn execute_with_context<F, T>(&self, operation: &str, f: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let result = f.await;
        with_operation_context(result, operation, "Client")
    }
}

/// 🔥 加载启用的服务
fn load_enabled_services(config: &Config, registry: &mut DefaultServiceRegistry) -> Result<()> {
    // 注册核心层服务
    register_core_services(config, registry)?;

    // 注册专业层服务
    register_professional_services(config, registry)?;

    // 注册企业层服务
    register_enterprise_services(config, registry)?;

    Ok(())
}

/// 注册核心层服务
fn register_core_services(_config: &Config, registry: &mut DefaultServiceRegistry) -> Result<()> {
    // #[cfg(feature = "auth")]  // auth 功能暂未启用
    // {
    //     tracing::debug!("注册认证服务");
    //     let metadata = ServiceMetadata {
    //         name: "auth".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书认证服务，提供令牌管理、身份验证等功能".to_string()),
    //         dependencies: vec![],
    //         provides: vec![
    //             "token-management".to_string(),
    //             "permission-control".to_string(),
    //         ],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 1,
    //     };
    //     registry.register_service(metadata)?;
    // }

    #[cfg(feature = "communication")]
    {
        tracing::debug!("注册通讯服务");
        let metadata = ServiceMetadata {
            name: "communication".to_string(),
            version: "1.0.0".to_string(),
            description: Some("飞书通讯服务，提供消息、联系人、群组等功能".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec![
                "im".to_string(),
                "contacts".to_string(),
                "groups".to_string(),
            ],
            status: ServiceStatus::Uninitialized,
            priority: 2,
        };
        registry
            .register_service(metadata)
            .map_err(|e| crate::error::internal_error(format!("注册通讯服务失败: {}", e)))?;
    }

    #[cfg(feature = "docs")]
    {
        tracing::debug!("注册文档服务");
        let metadata = ServiceMetadata {
            name: "docs".to_string(),
            version: "1.0.0".to_string(),
            description: Some("飞书文档服务，提供云文档、表格、知识库等功能".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec![
                "cloud-docs".to_string(),
                "sheets".to_string(),
                "wiki".to_string(),
            ],
            status: ServiceStatus::Uninitialized,
            priority: 2,
        };
        registry
            .register_service(metadata)
            .map_err(|e| crate::error::internal_error(format!("注册文档服务失败: {}", e)))?;
    }

    Ok(())
}

/// 注册专业层服务
fn register_professional_services(
    _config: &Config,
    _registry: &mut DefaultServiceRegistry,
) -> Result<()> {
    // #[cfg(feature = "hr")]  // hr 功能暂未启用
    // {
    //     tracing::debug!("注册人力资源服务");
    //     let metadata = ServiceMetadata {
    //         name: "hr".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书人力资源服务，提供员工、考勤、薪酬等功能".to_string()),
    //         dependencies: vec!["auth".to_string()],
    //         provides: vec![
    //             "attendance".to_string(),
    //             "corehr".to_string(),
    //             "ehr".to_string(),
    //         ],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 3,
    //     };
    //     registry.register_service(metadata)?;
    // }

    // #[cfg(feature = "ai")]  // ai 功能暂未启用
    // {
    //     tracing::debug!("注册AI服务");
    //     let metadata = ServiceMetadata {
    //         name: "ai".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书AI服务，提供智能助手、AI分析等功能".to_string()),
    //         dependencies: vec!["auth".to_string(), "communication".to_string()],
    //         provides: vec!["chatbot".to_string(), "smart-analysis".to_string()],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 4,
    //     };
    //     registry.register_service(metadata)?;
    // }

    // #[cfg(feature = "calendar")]  // calendar 功能暂未启用
    // {
    //     tracing::debug!("注册日历服务");
    //     let metadata = ServiceMetadata {
    //         name: "calendar".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书日历服务，提供日程管理、会议安排等功能".to_string()),
    //         dependencies: vec!["auth".to_string(), "communication".to_string()],
    //         provides: vec!["schedule".to_string(), "meetings".to_string()],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 4,
    //     };
    //     registry.register_service(metadata)?;
    // }

    Ok(())
}

/// 注册企业层服务
fn register_enterprise_services(
    _config: &Config,
    _registry: &mut DefaultServiceRegistry,
) -> Result<()> {
    // #[cfg(feature = "admin")]  // admin 功能暂未启用
    // {
    //     tracing::debug!("注册管理服务");
    //     let metadata = ServiceMetadata {
    //         name: "admin".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书管理服务，提供用户管理、系统配置等功能".to_string()),
    //         dependencies: vec!["auth".to_string(), "hr".to_string()],
    //         provides: vec!["user-management".to_string(), "system-config".to_string()],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 5,
    //     };
    //     registry.register_service(metadata)?;
    // }

    // #[cfg(feature = "approval")]  // approval 功能暂未启用
    // {
    //     tracing::debug!("注册审批服务");
    //     let metadata = ServiceMetadata {
    //         name: "approval".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书审批服务，提供审批流程、模板管理等功能".to_string()),
    //         dependencies: vec!["auth".to_string(), "communication".to_string()],
    //         provides: vec!["workflow".to_string(), "template".to_string()],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 5,
    //     };
    //     registry.register_service(metadata)?;
    // }

    // #[cfg(feature = "helpdesk")]  // helpdesk 功能暂未启用
    // {
    //     tracing::debug!("注册帮助台服务");
    //     let metadata = ServiceMetadata {
    //         name: "helpdesk".to_string(),
    //         version: "1.0.0".to_string(),
    //         description: Some("飞书帮助台服务，提供工单管理、知识库等功能".to_string()),
    //         dependencies: vec![
    //             "auth".to_string(),
    //             "communication".to_string(),
    //             "ai".to_string(),
    //         ],
    //         provides: vec!["ticket".to_string(), "knowledge-base".to_string()],
    //         status: ServiceStatus::Uninitialized,
    //         priority: 6,
    //     };
    //     registry.register_service(metadata)?;
    // }

    Ok(())
}

// 实现LarkClient trait
impl LarkClient for Client {
    fn config(&self) -> &Config {
        &self.config
    }

    fn is_configured(&self) -> bool {
        self.is_configured()
    }
}

/// 🏗️ 客户端构建器 - 流畅API
///
/// 提供链式调用的客户端构建方式
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Client;
/// use openlark_client::Result;
/// use std::time::Duration;
///
/// fn main() -> Result<()> {
///     let _client = Client::builder()
///         .app_id("your_app_id")
///         .app_secret("your_app_secret")
///         .base_url("https://open.feishu.cn")
///         .timeout(Duration::from_secs(30))
///         .build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    /// 🆕 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 🆔 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 🔑 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret = app_secret.into();
        self
    }

    /// 🏷️ 设置应用类型（自建 / 商店）
    pub fn app_type(mut self, app_type: openlark_core::constants::AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    /// 🔐 设置是否允许自动获取 token（默认 true）
    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.config.enable_token_cache = enable;
        self
    }

    /// 🌐 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    /// ⏱️ 设置请求超时时间
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// 🔄 设置重试次数
    pub fn retry_count(mut self, retry_count: u32) -> Self {
        self.config.retry_count = retry_count;
        self
    }

    /// 📝 启用或禁用日志
    pub fn enable_log(mut self, enable: bool) -> Self {
        self.config.enable_log = enable;
        self
    }

    /// 🌍 从环境变量加载配置
    pub fn from_env(mut self) -> Self {
        self.config.load_from_env();
        self
    }

    /// 🔨 构建客户端实例
    ///
    /// # 返回值
    /// 返回配置好的客户端实例或验证错误
    ///
    /// # 错误
    /// 如果配置验证失败，会返回相应的错误信息，包含用户友好的恢复建议
    pub fn build(self) -> Result<Client> {
        let result = Client::with_config(self.config);
        if let Err(ref error) = result {
            tracing::error!(
                "客户端构建失败: {}",
                error.user_message().unwrap_or("未知错误")
            );
        }
        result
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Client的便利构造函数
impl From<Config> for Result<Client> {
    fn from(config: Config) -> Self {
        Client::with_config(config)
    }
}

/// 客户端错误处理扩展特征
pub trait ClientErrorHandling {
    /// 处理错误并添加客户端上下文
    fn handle_error<T>(&self, result: Result<T>, operation: &str) -> Result<T>;
    /// 处理异步错误并添加客户端上下文
    async fn handle_async_error<T, F>(&self, f: F, operation: &str) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>;
}

impl ClientErrorHandling for Client {
    fn handle_error<T>(&self, result: Result<T>, operation: &str) -> Result<T> {
        with_operation_context(result, operation, "Client")
    }

    async fn handle_async_error<T, F>(&self, f: F, operation: &str) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let result = f.await;
        with_operation_context(result, operation, "Client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::error::ErrorTrait;
    use std::time::Duration;

    #[test]
    fn test_client_builder() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .timeout(Duration::from_secs(30))
            .build();

        assert!(client.is_ok());
    }

    #[test]
    fn test_client_config() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let client = Client::with_config(config).unwrap();
        assert_eq!(client.config().app_id, "test_app_id");
        assert_eq!(client.config().app_secret, "test_app_secret");
        assert!(client.is_configured());
    }

    #[test]
    fn test_client_not_configured() {
        let config = Config {
            app_id: String::new(),
            app_secret: String::new(),
            ..Default::default()
        };

        let client_result = Client::with_config(config);
        assert!(client_result.is_err());

        if let Err(error) = client_result {
            assert!(error.is_config_error() || error.is_validation_error());
            assert!(!error.user_message().unwrap_or("未知错误").is_empty());
        }
    }

    #[test]
    fn test_client_clone() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let cloned_client = client.clone();
        assert_eq!(client.config().app_id, cloned_client.config().app_id);
    }

    #[cfg(feature = "cardkit")]
    #[test]
    fn test_cardkit_chain_exists() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let _ = &client.cardkit.v1.card;
    }

    #[cfg(feature = "docs")]
    #[test]
    fn test_docs_chain_exists() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let _ = client.docs.config();
    }

    #[cfg(feature = "communication")]
    #[test]
    fn test_communication_chain_exists() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let _ = client.communication.config();
    }

    #[cfg(feature = "meeting")]
    #[test]
    fn test_meeting_chain_exists() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let _ = client.meeting.config();
    }

    #[test]
    fn test_client_error_handling() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        // 测试错误上下文处理
        let error_result: Result<i32> =
            Err(crate::error::validation_error("field", "validation failed"));
        let result = client.handle_error(error_result, "test_operation");

        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error.context().has_context("operation"));
            assert_eq!(
                error.context().get_context("operation"),
                Some("test_operation")
            );
            assert_eq!(error.context().get_context("component"), Some("Client"));
        }
    }

    #[tokio::test]
    async fn test_async_error_handling() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        // 测试异步错误上下文处理
        let result = client
            .handle_async_error(
                async { Err::<i32, _>(crate::error::network_error("async error")) },
                "async_test",
            )
            .await;

        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error.context().has_context("operation"));
            assert_eq!(error.context().get_context("operation"), Some("async_test"));
            assert_eq!(error.context().get_context("component"), Some("Client"));
        }
    }

    #[test]
    fn test_from_env_missing_vars() {
        // 这个测试可能在有环境变量的情况下失败，我们跳过它
        // 在实际应用中，Client::from_env() 依赖于环境变量，难以在测试中完全控制
        // 改为测试构建器的错误情况
        let builder = ClientBuilder::default();
        let result = builder.build();
        assert!(result.is_err()); // 没有app_id和app_secret应该失败
    }

    #[test]
    fn test_from_app_id_string() {
        std::env::set_var("OPENLARK_APP_ID", "test_app_id");
        std::env::set_var("OPENLARK_APP_SECRET", "test_secret");

        let result: Result<Client> = Client::from_env();
        assert!(result.is_ok());

        if let Ok(client) = result {
            assert_eq!(client.config().app_id, "test_app_id");
            assert_eq!(client.config().app_secret, "test_secret");
        }

        // 清理环境变量
        std::env::remove_var("OPENLARK_APP_ID");
        std::env::remove_var("OPENLARK_APP_SECRET");
    }

    #[test]
    fn test_builder_default() {
        let builder = ClientBuilder::default();
        assert!(builder.config.app_id.is_empty());
        assert!(builder.config.app_secret.is_empty());
    }

    #[cfg(feature = "communication")]
    #[test]
    fn test_communication_service_access() {
        let client = Client::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        // 单入口：meta 链式字段访问（这里只验证字段可用）
        let _comm = &client.communication;
    }

    // === 异步客户端功能测试 ===
    // 测试LarkClient特征和扩展特征的异步功能

    // 简化的模拟客户端，专注于异步功能测试
    struct MockAsyncClient {
        app_id: String,
        app_secret: String,
        request_count: std::sync::atomic::AtomicU64,
    }

    impl MockAsyncClient {
        fn new(app_id: &str, app_secret: &str) -> Self {
            Self {
                app_id: app_id.to_string(),
                app_secret: app_secret.to_string(),
                request_count: std::sync::atomic::AtomicU64::new(0),
            }
        }

        fn increment_request_count(&self) {
            self.request_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        fn get_request_count(&self) -> u64 {
            self.request_count.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    // 异步认证特征
    trait MockAuthenticatedClient {
        async fn get_access_token(&self) -> crate::Result<String>;
        async fn refresh_token(&self) -> crate::Result<()>;
        async fn is_token_valid(&self) -> crate::Result<bool>;
    }

    // 异步请求特征
    trait MockRequestClient {
        async fn send_request(&self, endpoint: &str) -> crate::Result<String>;
        async fn get(&self, endpoint: &str) -> crate::Result<String>;
        async fn post(&self, endpoint: &str, data: &str) -> crate::Result<String>;
    }

    impl MockAuthenticatedClient for MockAsyncClient {
        async fn get_access_token(&self) -> crate::Result<String> {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok("mock_token_123".to_string())
        }

        async fn refresh_token(&self) -> crate::Result<()> {
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            if !self.app_id.is_empty() && !self.app_secret.is_empty() {
                Ok(())
            } else {
                Err(crate::error::configuration_error("无效的配置"))
            }
        }

        async fn is_token_valid(&self) -> crate::Result<bool> {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            Ok(true)
        }
    }

    impl MockRequestClient for MockAsyncClient {
        async fn send_request(&self, endpoint: &str) -> crate::Result<String> {
            self.increment_request_count();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            if self.app_id == "error_app_id" {
                Err(crate::error::network_error("模拟网络错误"))
            } else {
                Ok(format!("Response from {}", endpoint))
            }
        }

        async fn get(&self, endpoint: &str) -> crate::Result<String> {
            self.send_request(&format!("GET {}", endpoint)).await
        }

        async fn post(&self, endpoint: &str, data: &str) -> crate::Result<String> {
            self.send_request(&format!("POST {} {}", endpoint, data))
                .await
        }
    }

    #[tokio::test]
    async fn test_async_mock_client_configuration() {
        let client = MockAsyncClient::new("test_app_id", "test_app_secret");

        // 测试基本配置
        assert_eq!(client.app_id, "test_app_id");
        assert_eq!(client.app_secret, "test_app_secret");
        assert_eq!(client.get_request_count(), 0);
    }

    #[tokio::test]
    async fn test_async_authenticated_client_operations() {
        let client = MockAsyncClient::new("auth_app", "auth_secret");

        // 测试认证操作
        let token_result = client.get_access_token().await;
        assert!(token_result.is_ok());
        assert_eq!(token_result.unwrap(), "mock_token_123");

        let is_valid_result = client.is_token_valid().await;
        assert!(is_valid_result.is_ok());
        assert!(is_valid_result.unwrap());

        let refresh_result = client.refresh_token().await;
        assert!(refresh_result.is_ok());
    }

    #[tokio::test]
    async fn test_async_authenticated_client_config_error() {
        let client = MockAsyncClient::new("", "auth_secret");

        // 测试配置错误时的认证操作
        let refresh_result = client.refresh_token().await;
        assert!(refresh_result.is_err());
        let err = refresh_result.unwrap_err();
        assert!(err.is_config_error());
        assert!(!err.is_validation_error());
    }

    #[tokio::test]
    async fn test_async_request_client_operations() {
        let client = MockAsyncClient::new("request_app", "request_secret");

        // 测试请求操作
        let get_result = client.get("test/endpoint").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().contains("test/endpoint"));

        let post_result = client.post("test/api", "test_data").await;
        assert!(post_result.is_ok());
        assert!(post_result.unwrap().contains("test/api test_data"));

        // 验证请求计数
        assert_eq!(client.get_request_count(), 2);
    }

    #[tokio::test]
    async fn test_async_request_client_error_handling() {
        let client = MockAsyncClient::new("error_app_id", "request_secret");

        // 测试错误处理
        let error_result = client.get("error/endpoint").await;
        assert!(error_result.is_err());
        assert!(error_result.clone().unwrap_err().is_network_error());
        assert!(error_result.clone().unwrap_err().is_retryable());
    }

    #[tokio::test]
    async fn test_async_concurrent_operations() {
        use tokio::task::JoinSet;

        let client =
            std::sync::Arc::new(MockAsyncClient::new("concurrent_app", "concurrent_secret"));
        let mut join_set: JoinSet<crate::Result<String>> = JoinSet::new();

        // 并发执行多个认证操作（转换为String返回）
        let client_clone = client.clone();
        join_set.spawn(async move { client_clone.get_access_token().await });

        let client_clone = client.clone();
        join_set.spawn(async move {
            match client_clone.is_token_valid().await {
                Ok(valid) => Ok(format!("valid: {}", valid)),
                Err(e) => Err(e),
            }
        });

        let client_clone = client.clone();
        join_set.spawn(async move {
            match client_clone.refresh_token().await {
                Ok(_) => Ok("refreshed".to_string()),
                Err(e) => Err(e),
            }
        });

        // 并发执行多个请求操作
        for i in 0..3 {
            let client_clone = client.clone();
            join_set.spawn(async move { client_clone.get(&format!("endpoint/{}", i)).await });
        }

        // 等待所有操作完成
        let mut results = vec![];
        while let Some(result) = join_set.join_next().await {
            results.push(result);
        }

        assert_eq!(results.len(), 6); // 3个认证 + 3个请求

        // 验证所有操作都成功
        for result in results {
            assert!(result.is_ok());
            let inner_result = result.unwrap();
            assert!(inner_result.is_ok());
            let result_str = inner_result.unwrap();
            assert!(!result_str.is_empty());
        }
    }

    #[tokio::test]
    async fn test_async_timing_behavior() {
        let client = MockAsyncClient::new("timing_app", "timing_secret");

        let start = std::time::Instant::now();

        // 执行一系列异步操作
        let _ = client.get_access_token().await;
        let _ = client.is_token_valid().await;
        let _ = client.refresh_token().await;
        let _ = client.get("test/endpoint").await;

        let elapsed = start.elapsed();

        // 验证总时间符合预期（每个操作都有延迟）
        assert!(elapsed >= tokio::time::Duration::from_millis(400)); // 4个操作 * 100ms + 1个 * 50ms + 1个 * 200ms
        assert!(elapsed <= tokio::time::Duration::from_millis(600)); // 允许一些误差
    }

    #[tokio::test]
    async fn test_async_client_state_mutation() {
        let client = MockAsyncClient::new("state_app", "state_secret");

        // 初始状态
        assert_eq!(client.get_request_count(), 0);

        // 执行操作改变状态
        let _ = client.get("endpoint1").await;
        assert_eq!(client.get_request_count(), 1);

        let _ = client.post("endpoint2", "data").await;
        assert_eq!(client.get_request_count(), 2);

        let _ = client.get("endpoint3").await;
        assert_eq!(client.get_request_count(), 3);
    }
}
