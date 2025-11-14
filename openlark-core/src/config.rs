use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::{
    app_ticket_manager::AppTicketManager,
    constants::{AppType, FEISHU_BASE_URL},
    performance::OptimizedHttpConfig,
    token_manager::TokenManager,
};

#[derive(Debug, Clone)]
pub struct Config {
    /// 包装在 Arc 中的共享配置数据
    inner: Arc<ConfigInner>,
}

/// 内部配置数据，被多个服务共享
#[derive(Debug)]
pub struct ConfigInner {
    pub app_id: String,
    pub app_secret: String,
    /// 域名, 默认为 <https://open.feishu.cn>
    pub base_url: String,
    pub enable_token_cache: bool,
    /// 应用类型, 默认为自建应用
    pub app_type: AppType,
    pub http_client: reqwest::Client,
    /// 客户端超时时间, 默认永不超时
    pub req_timeout: Option<Duration>,
    pub header: HashMap<String, String>,
    /// Token 管理器
    pub token_manager: Arc<Mutex<TokenManager>>,
    /// App Ticket 管理器
    pub app_ticket_manager: Arc<Mutex<AppTicketManager>>,
}

impl Default for ConfigInner {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: FEISHU_BASE_URL.to_string(),
            enable_token_cache: true,
            app_type: AppType::SelfBuild,
            http_client: reqwest::Client::new(),
            req_timeout: None,
            header: Default::default(),
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            app_ticket_manager: Arc::new(Mutex::new(AppTicketManager::new())),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: Arc::new(ConfigInner::default()),
        }
    }
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// 创建新的 Config 实例，直接从 ConfigInner
    pub fn new(inner: ConfigInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    /// 获取内部 Arc 的引用计数
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

#[derive(Default, Clone)]
pub struct ConfigBuilder {
    app_id: Option<String>,
    app_secret: Option<String>,
    base_url: Option<String>,
    enable_token_cache: Option<bool>,
    app_type: Option<AppType>,
    http_client: Option<reqwest::Client>,
    req_timeout: Option<Duration>,
    header: Option<HashMap<String, String>>,
}

impl ConfigBuilder {
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = Some(app_secret.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.enable_token_cache = Some(enable);
        self
    }

    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.app_type = Some(app_type);
        self
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// 使用优化的HTTP配置构建客户端
    pub fn optimized_http_client(
        mut self,
        config: OptimizedHttpConfig,
    ) -> Result<Self, reqwest::Error> {
        let client = config.build_client()?;
        self.http_client = Some(client);
        Ok(self)
    }

    /// 使用生产环境优化配置
    pub fn production_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::production();
        self.optimized_http_client(config)
    }

    /// 使用高吞吐量配置
    pub fn high_throughput_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::high_throughput();
        self.optimized_http_client(config)
    }

    /// 使用低延迟配置
    pub fn low_latency_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::low_latency();
        self.optimized_http_client(config)
    }

    pub fn req_timeout(mut self, timeout: Duration) -> Self {
        self.req_timeout = Some(timeout);
        self
    }

    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn build(self) -> Config {
        let default = ConfigInner::default();
        Config::new(ConfigInner {
            app_id: self.app_id.unwrap_or(default.app_id),
            app_secret: self.app_secret.unwrap_or(default.app_secret),
            base_url: self.base_url.unwrap_or(default.base_url),
            enable_token_cache: self
                .enable_token_cache
                .unwrap_or(default.enable_token_cache),
            app_type: self.app_type.unwrap_or(default.app_type),
            http_client: self.http_client.unwrap_or(default.http_client),
            req_timeout: self.req_timeout.or(default.req_timeout),
            header: self.header.unwrap_or(default.header),
            token_manager: default.token_manager,
            app_ticket_manager: default.app_ticket_manager,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{AppType, FEISHU_BASE_URL};
    use std::time::Duration;

    #[test]
    fn test_config_creation() {
        let config = Config::new(ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://test.api.com".to_string(),
            enable_token_cache: true,
            app_type: AppType::SelfBuild,
            http_client: reqwest::Client::new(),
            req_timeout: Some(Duration::from_secs(30)),
            header: HashMap::new(),
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            app_ticket_manager: Arc::new(Mutex::new(AppTicketManager::new())),
        });

        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://test.api.com");
        assert!(config.enable_token_cache);
        assert_eq!(config.req_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.app_id, "");
        assert_eq!(config.app_secret, "");
        assert_eq!(config.base_url, FEISHU_BASE_URL);
        assert!(config.enable_token_cache);
        assert_eq!(config.app_type, AppType::SelfBuild);
        assert!(config.req_timeout.is_none());
        assert!(config.header.is_empty());
    }

    #[test]
    fn test_config_clone() {
        let config = Config::new(ConfigInner {
            app_id: "clone_test".to_string(),
            app_secret: "clone_secret".to_string(),
            base_url: "https://clone.test.com".to_string(),
            enable_token_cache: false,
            app_type: AppType::Marketplace,
            http_client: reqwest::Client::new(),
            req_timeout: Some(Duration::from_secs(60)),
            header: {
                let mut header = HashMap::new();
                header.insert("Test-Header".to_string(), "test-value".to_string());
                header
            },
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            app_ticket_manager: Arc::new(Mutex::new(AppTicketManager::new())),
        });

        let cloned_config = config.clone();

        assert_eq!(config.app_id, cloned_config.app_id);
        assert_eq!(config.app_secret, cloned_config.app_secret);
        assert_eq!(config.base_url, cloned_config.base_url);
        assert_eq!(config.enable_token_cache, cloned_config.enable_token_cache);
        assert_eq!(config.app_type, cloned_config.app_type);
        assert_eq!(config.req_timeout, cloned_config.req_timeout);
        assert_eq!(config.header.len(), cloned_config.header.len());
        assert_eq!(
            config.header.get("Test-Header"),
            cloned_config.header.get("Test-Header")
        );

        // Verify Arc clone efficiency - both should point to same memory
        assert!(Arc::ptr_eq(&config.inner, &cloned_config.inner));

        // Verify reference counting works
        assert_eq!(config.reference_count(), 2);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("app_id"));
        assert!(debug_str.contains("app_secret"));
        assert!(debug_str.contains("base_url"));
    }

    #[test]
    fn test_config_with_custom_header() {
        let mut header = HashMap::new();
        header.insert("Authorization".to_string(), "Bearer token".to_string());
        header.insert("Content-Type".to_string(), "application/json".to_string());

        let config = Config::new(ConfigInner {
            header,
            ..ConfigInner::default()
        });

        assert_eq!(config.header.len(), 2);
        assert_eq!(
            config.header.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(
            config.header.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_config_with_different_app_types() {
        let self_build_config = Config::new(ConfigInner {
            app_type: AppType::SelfBuild,
            ..ConfigInner::default()
        });

        let marketplace_config = Config::new(ConfigInner {
            app_type: AppType::Marketplace,
            ..ConfigInner::default()
        });

        assert_eq!(self_build_config.app_type, AppType::SelfBuild);
        assert_eq!(marketplace_config.app_type, AppType::Marketplace);
        assert_ne!(self_build_config.app_type, marketplace_config.app_type);
    }

    #[test]
    fn test_config_with_timeout_variations() {
        let no_timeout_config = Config::default();

        let short_timeout_config = Config::new(ConfigInner {
            req_timeout: Some(Duration::from_secs(5)),
            ..ConfigInner::default()
        });

        let long_timeout_config = Config::new(ConfigInner {
            req_timeout: Some(Duration::from_secs(300)),
            ..ConfigInner::default()
        });

        assert!(no_timeout_config.req_timeout.is_none());
        assert_eq!(
            short_timeout_config.req_timeout,
            Some(Duration::from_secs(5))
        );
        assert_eq!(
            long_timeout_config.req_timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_config_builders() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        assert_eq!(config.app_id, "test_app");
        assert_eq!(config.app_secret, "test_secret");
    }

    #[test]
    fn test_config_arc_efficiency() {
        let config = Config::default();
        assert_eq!(config.reference_count(), 1);

        let config_clone = config.clone();
        assert_eq!(config.reference_count(), 2);
        assert_eq!(config_clone.reference_count(), 2);

        // Both configs should point to the same inner data
        assert!(Arc::ptr_eq(&config.inner, &config_clone.inner));
    }

    #[test]
    fn test_arc_efficiency_simulation() {
        // 模拟服务模块中的多次克隆
        let config = Config::default();

        // 模拟 PerformanceService::new() 中的4次clone
        let service1_config = config.clone();
        let service2_config = config.clone();
        let service3_config = config.clone();
        let service4_config = config.clone();

        // 所有配置应该指向同一个内存位置
        assert!(Arc::ptr_eq(&config.inner, &service1_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service2_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service3_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service4_config.inner));

        // 引用计数应该是5（原始 + 4个克隆）
        assert_eq!(config.reference_count(), 5);

        println!("Arc<Config> 改造成功：5个配置实例共享同一份内存！");
    }
}
