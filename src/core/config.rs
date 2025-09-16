use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::core::{
    app_ticket_manager::AppTicketManager,
    constants::{AppType, FEISHU_BASE_URL},
    token_manager::TokenManager,
};

#[derive(Debug, Clone)]
pub struct Config {
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

impl Default for Config {
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

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
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

    pub fn req_timeout(mut self, timeout: Duration) -> Self {
        self.req_timeout = Some(timeout);
        self
    }

    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn build(self) -> Config {
        let default = Config::default();
        Config {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{AppType, FEISHU_BASE_URL};
    use std::time::Duration;

    #[test]
    fn test_config_creation() {
        let config = Config {
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
        };

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
        let config = Config {
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
        };

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
        let mut config = Config::default();
        config
            .header
            .insert("Authorization".to_string(), "Bearer token".to_string());
        config
            .header
            .insert("Content-Type".to_string(), "application/json".to_string());

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
        let self_build_config = Config {
            app_type: AppType::SelfBuild,
            ..Default::default()
        };

        let marketplace_config = Config {
            app_type: AppType::Marketplace,
            ..Default::default()
        };

        assert_eq!(self_build_config.app_type, AppType::SelfBuild);
        assert_eq!(marketplace_config.app_type, AppType::Marketplace);
        assert_ne!(self_build_config.app_type, marketplace_config.app_type);
    }

    #[test]
    fn test_config_with_timeout_variations() {
        let no_timeout_config = Config {
            req_timeout: None,
            ..Default::default()
        };

        let short_timeout_config = Config {
            req_timeout: Some(Duration::from_secs(5)),
            ..Default::default()
        };

        let long_timeout_config = Config {
            req_timeout: Some(Duration::from_secs(300)),
            ..Default::default()
        };

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
    fn test_config_token_cache_settings() {
        let cache_enabled_config = Config {
            enable_token_cache: true,
            ..Default::default()
        };

        let cache_disabled_config = Config {
            enable_token_cache: false,
            ..Default::default()
        };

        assert!(cache_enabled_config.enable_token_cache);
        assert!(!cache_disabled_config.enable_token_cache);
    }

    #[test]
    fn test_config_base_url_variations() {
        let default_config = Config::default();
        assert_eq!(default_config.base_url, FEISHU_BASE_URL);

        let custom_config = Config {
            base_url: "https://custom.feishu.com".to_string(),
            ..Default::default()
        };
        assert_eq!(custom_config.base_url, "https://custom.feishu.com");

        let localhost_config = Config {
            base_url: "http://localhost:8080".to_string(),
            ..Default::default()
        };
        assert_eq!(localhost_config.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_config_empty_credentials() {
        let empty_config = Config::default();
        assert!(empty_config.app_id.is_empty());
        assert!(empty_config.app_secret.is_empty());
    }

    #[test]
    fn test_config_long_credentials() {
        let long_app_id = "a".repeat(100);
        let long_app_secret = "b".repeat(200);

        let config = Config {
            app_id: long_app_id.clone(),
            app_secret: long_app_secret.clone(),
            ..Default::default()
        };

        assert_eq!(config.app_id.len(), 100);
        assert_eq!(config.app_secret.len(), 200);
        assert_eq!(config.app_id, long_app_id);
        assert_eq!(config.app_secret, long_app_secret);
    }

    #[test]
    fn test_config_special_characters_in_credentials() {
        let special_app_id = "app-id_with.special@chars#123";
        let special_app_secret = "secret$with%special&chars*()+={}[]|\\:;\"'<>?/~`";

        let config = Config {
            app_id: special_app_id.to_string(),
            app_secret: special_app_secret.to_string(),
            ..Default::default()
        };

        assert_eq!(config.app_id, special_app_id);
        assert_eq!(config.app_secret, special_app_secret);
    }

    #[test]
    fn test_config_unicode_credentials() {
        let unicode_app_id = "应用标识符_🚀";
        let unicode_app_secret = "密钥_🔐_测试";

        let config = Config {
            app_id: unicode_app_id.to_string(),
            app_secret: unicode_app_secret.to_string(),
            ..Default::default()
        };

        assert_eq!(config.app_id, unicode_app_id);
        assert_eq!(config.app_secret, unicode_app_secret);
    }

    #[test]
    fn test_config_http_client_creation() {
        let config = Config::default();

        // Test that the HTTP client is created successfully
        assert!(format!("{:?}", config.http_client).contains("Client"));
    }

    #[test]
    fn test_config_managers_creation() {
        let config = Config::default();

        // Test that managers are properly initialized as Arc<Mutex<T>>
        assert!(config.token_manager.try_lock().is_ok());
        assert!(config.app_ticket_manager.try_lock().is_ok());
    }

    #[test]
    fn test_config_concurrent_access() {
        let config = Config::default();
        let config_clone = config.clone();

        // Test that cloned config can access managers independently
        let token_manager1 = config.token_manager.clone();
        let token_manager2 = config_clone.token_manager.clone();

        assert!(token_manager1.try_lock().is_ok());
        assert!(token_manager2.try_lock().is_ok());
    }

    #[test]
    fn test_config_header_manipulation() {
        let mut config = Config::default();

        // Start with empty headers
        assert!(config.header.is_empty());

        // Add headers
        config
            .header
            .insert("User-Agent".to_string(), "open-lark-sdk".to_string());
        config
            .header
            .insert("Accept".to_string(), "application/json".to_string());
        assert_eq!(config.header.len(), 2);

        // Update existing header
        config
            .header
            .insert("User-Agent".to_string(), "open-lark-sdk-v2".to_string());
        assert_eq!(config.header.len(), 2);
        assert_eq!(
            config.header.get("User-Agent"),
            Some(&"open-lark-sdk-v2".to_string())
        );

        // Remove header
        config.header.remove("Accept");
        assert_eq!(config.header.len(), 1);
        assert!(!config.header.contains_key("Accept"));
    }

    #[test]
    fn test_config_timeout_duration_precision() {
        let millis_config = Config {
            req_timeout: Some(Duration::from_millis(1500)), // 1.5 seconds
            ..Default::default()
        };

        let nanos_config = Config {
            req_timeout: Some(Duration::from_nanos(2_500_000_000)), // 2.5 seconds
            ..Default::default()
        };

        assert_eq!(millis_config.req_timeout, Some(Duration::from_millis(1500)));
        assert_eq!(
            nanos_config.req_timeout,
            Some(Duration::from_nanos(2_500_000_000))
        );
    }

    #[test]
    fn test_config_extreme_timeout_values() {
        let zero_timeout_config = Config {
            req_timeout: Some(Duration::from_secs(0)),
            ..Default::default()
        };

        let max_timeout_config = Config {
            req_timeout: Some(Duration::from_secs(u64::MAX)),
            ..Default::default()
        };

        assert_eq!(
            zero_timeout_config.req_timeout,
            Some(Duration::from_secs(0))
        );
        assert_eq!(
            max_timeout_config.req_timeout,
            Some(Duration::from_secs(u64::MAX))
        );
    }

    #[test]
    fn test_config_memory_efficiency() {
        // Test that Config doesn't consume excessive memory
        let configs: Vec<Config> = (0..100)
            .map(|i| Config {
                app_id: format!("app_{}", i),
                app_secret: format!("secret_{}", i),
                ..Default::default()
            })
            .collect();

        assert_eq!(configs.len(), 100);

        // Verify each config is properly initialized
        for (i, config) in configs.iter().enumerate() {
            assert_eq!(config.app_id, format!("app_{}", i));
            assert_eq!(config.app_secret, format!("secret_{}", i));
        }
    }
}
