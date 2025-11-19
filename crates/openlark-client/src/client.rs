//! OpenLark Client - 全新简化架构
//!
//! 极简设计，1行代码创建客户端，类型安全的服务访问

use crate::{traits::LarkClient, Config, Result, ServiceRegistry};
use std::sync::Arc;

/// 🚀 OpenLark客户端 - 极简设计
///
/// # 特性
/// - 零配置启动：`Client::from_env()`
/// - 类型安全的服务访问
/// - 编译时feature优化
/// - 高性能异步
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // 从环境变量创建客户端
///     let client = Client::from_env()?;
///
///     // 发送消息（需要communication feature）
///     #[cfg(feature = "communication")]
///     {
///         let result = client.communication()
///             .send_text("user_id", "open_id", "Hello!")
///             .await?;
///         println!("消息发送成功: {}", result.message_id);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    /// 客户端配置
    config: Arc<Config>,
    /// 服务注册表
    registry: Arc<ServiceRegistry>,
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
    /// let client = Client::from_env()?;
    /// ```
    pub fn from_env() -> Result<Self> {
        Self::builder().from_env().build()
    }

    /// 🏗️ 创建构建器
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// 📡 访问通讯服务
    ///
    /// 需要 `communication` feature
    #[cfg(feature = "communication")]
    pub fn communication(&self) -> crate::services::CommunicationService<'_> {
        crate::services::CommunicationService::new(&self.config, &self.registry)
    }

    /// 👥 访问HR服务
    ///
    /// 需要 `hr` feature
    #[cfg(feature = "hr")]
    pub fn hr(&self) -> crate::services::HRService<'_> {
        crate::services::HRService::new(&self.config, &self.registry)
    }

    /// 📄 访问文档服务
    ///
    /// 需要 `docs` feature
    #[cfg(feature = "docs")]
    pub fn docs(&self) -> crate::services::DocsService<'_> {
        crate::services::DocsService::new(&self.config, &self.registry)
    }

    /// 🤖 访问AI服务
    ///
    /// 需要 `ai` feature
    #[cfg(feature = "ai")]
    pub fn ai(&self) -> crate::services::AIService<'_> {
        crate::services::AIService::new(&self.config)
    }

    /// 🔐 访问认证服务
    ///
    /// 需要 `auth` feature
    #[cfg(feature = "auth")]
    pub fn auth(&self) -> crate::services::AuthService<'_> {
        crate::services::AuthService::new(&self.config)
    }

    /// 🔧 获取客户端配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 📋 获取服务注册表
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// ✅ 检查客户端是否已正确配置
    pub fn is_configured(&self) -> bool {
        !self.config.app_id.is_empty() && !self.config.app_secret.is_empty()
    }

    /// 🆕 创建带有自定义配置的客户端
    pub fn with_config(config: Config) -> Result<Self> {
        config.validate()?;
        let config = Arc::new(config);
        let registry = Arc::new(ServiceRegistry::new(&config));

        Ok(Client { config, registry })
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
/// use std::time::Duration;
///
/// let client = Client::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("https://open.feishu.cn")
///     .timeout(Duration::from_secs(30))
///     .build()?;
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
    /// 如果配置验证失败，会返回相应的错误信息
    pub fn build(self) -> Result<Client> {
        Client::with_config(self.config)
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

#[cfg(test)]
mod tests {
    use super::*;
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

        let client = Client::with_config(config).unwrap();
        assert!(!client.is_configured());
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

    #[test]
    fn test_from_env_missing_vars() {
        // 清理环境变量
        std::env::remove_var("OPENLARK_APP_ID");
        std::env::remove_var("OPENLARK_APP_SECRET");

        let result = Client::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_app_id_string() {
        std::env::set_var("OPENLARK_APP_SECRET", "test_secret");

        let result: Result<Client> = "test_app_id".into();
        assert!(result.is_ok());

        if let Ok(client) = result {
            assert_eq!(client.config().app_id, "test_app_id");
            assert_eq!(client.config().app_secret, "test_secret");
        }

        // 清理环境变量
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

        // 这个测试只验证服务访问器可以正常创建
        // 实际的API调用需要mock服务器
        let _service = client.communication();
    }
}
