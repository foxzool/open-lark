//! OpenLark Client - 全新简化架构
//!
//! 极简设计：仅保留 meta 链式字段访问（单入口，KISS）

use crate::{
    error::{with_context, with_operation_context},
    traits::LarkClient,
    Config, DefaultServiceRegistry, Result,
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
        if let Err(err) = crate::registry::bootstrap::register_compiled_services(&mut registry) {
            return with_operation_context(Err(err), "Client::with_config", "service_loading");
        }

        let registry = Arc::new(registry);

        // 从 client Config 获取 core Config
        #[cfg(feature = "auth")]
        let base_core_config = config.as_ref().build_core_config();
        #[cfg(feature = "auth")]
        let core_config = config
            .as_ref()
            .get_or_build_core_config_with_token_provider();
        #[cfg(not(feature = "auth"))]
        let core_config = config.as_ref().get_or_build_core_config();

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
        // 验证默认构建器未配置 app_id/app_secret 时会失败（不依赖环境变量）。
        let builder = ClientBuilder::default();
        let result = builder.build();
        assert!(result.is_err()); // 没有app_id和app_secret应该失败
    }

    #[test]
    fn test_from_app_id_string() {
        crate::test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_secret")),
            ],
            || {
                let result: Result<Client> = Client::from_env();
                assert!(result.is_ok());

                if let Ok(client) = result {
                    assert_eq!(client.config().app_id, "test_app_id");
                    assert_eq!(client.config().app_secret, "test_secret");
                }
            },
        );
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
}
