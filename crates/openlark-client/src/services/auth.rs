//! 认证服务适配器
//!
//! 将openlark-auth服务适配到统一客户端接口。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::unified::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus, ServiceLifecycle},
    config::{UnifiedConfig, AuthConfig},
    error::{UnifiedError, UnifiedResult},
};

/// 认证服务适配器
///
/// 将openlark-auth的功能适配到统一客户端接口。
#[derive(Debug, Clone)]
pub struct AuthService {
    /// 服务配置
    config: Option<AuthConfig>,
    /// 服务状态
    status: ServiceStatus,
    /// 核心客户端（用于实际API调用）
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
    /// 令牌管理器
    token_manager: Option<Arc<openlark_core::token_manager::TokenManager>>,
    /// 服务元数据
    metadata: HashMap<String, String>,
}

impl AuthService {
    /// 创建新的认证服务适配器
    pub fn new() -> Self {
        Self {
            config: None,
            status: ServiceStatus::Uninitialized,
            core_client: None,
            token_manager: None,
            metadata: HashMap::new(),
        }
    }

    /// 从配置创建服务
    pub fn with_config(mut self, config: AuthConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心客户端创建服务
    pub fn with_core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 设置令牌管理器
    pub fn with_token_manager(
        mut self,
        token_manager: Arc<openlark_core::token_manager::TokenManager>,
    ) -> Self {
        self.token_manager = Some(token_manager);
        self
    }

    /// 检查服务是否可用
    pub fn is_enabled(&self) -> bool {
        self.config
            .as_ref()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// 获取应用访问令牌
    pub async fn get_app_access_token(&self) -> UnifiedResult<TokenInfo> {
        self.ensure_available()?;

        tracing::info!("获取应用访问令牌");

        // 模拟令牌信息
        Ok(TokenInfo {
            access_token: "mock_access_token".to_string(),
            refresh_token: Some("mock_refresh_token".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            token_type: "Bearer".to_string(),
        })
    }

    /// 获取用户访问令牌
    pub async fn get_user_access_token(&self, code: &str) -> UnifiedResult<TokenInfo> {
        self.ensure_available()?;

        tracing::info!("使用授权码获取用户访问令牌: {}", code);

        // 模拟令牌交换
        Ok(TokenInfo {
            access_token: "mock_user_access_token".to_string(),
            refresh_token: Some("mock_user_refresh_token".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            token_type: "Bearer".to_string(),
        })
    }

    /// 刷新访问令牌
    pub async fn refresh_access_token(&self, refresh_token: &str) -> UnifiedResult<TokenInfo> {
        self.ensure_available()?;

        tracing::info!("刷新访问令牌");

        // 模拟令牌刷新
        Ok(TokenInfo {
            access_token: "refreshed_access_token".to_string(),
            refresh_token: Some(refresh_token.to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            token_type: "Bearer".to_string(),
        })
    }

    /// 验证令牌
    pub async fn verify_token(&self, token: &str) -> UnifiedResult<TokenVerificationResult> {
        self.ensure_available()?;

        tracing::info!("验证令牌");

        // 模拟令牌验证
        Ok(TokenVerificationResult {
            valid: !token.is_empty(),
            user_id: Some("mock_user_id".to_string()),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
            scopes: vec!["user:info", "docs:read".to_string()],
        })
    }

    /// OAuth授权URL
    pub fn get_oauth_url(&self, client_id: &str, redirect_uri: &str, scopes: &[&str]) -> String {
        let config = self.config.as_ref().unwrap_or(&AuthConfig::default());

        if let Some(oauth_config) = &config.oauth {
            format!(
                "{}?client_id={}&redirect_uri={}&scope={}",
                oauth_config.auth_url,
                client_id,
                redirect_uri,
                scopes.join(" ")
            )
        } else {
            // 默认OAuth URL
            format!(
                "https://open.feishu.cn/open-apis/authen/v1/authorize?client_id={}&redirect_uri={}&scope={}",
                client_id,
                redirect_uri,
                scopes.join(" ")
            )
        }
    }

    /// 应用访问令牌URL
    fn get_app_access_token_url(&self) -> String {
        let config = self.config.as_ref().unwrap_or(&AuthConfig::default());

        config
            .token
            .cache
            .enabled
            .then(|| "https://open.feishu.cn/open-apis/authen/v3/app_access_token/internal".to_string())
            .unwrap_or_else(|| "https://open.feishu.cn/open-apis/authen/v3/app_access_token/internal".to_string())
    }

    /// 确保服务可用
    fn ensure_available(&self) -> UnifiedResult<()> {
        if !self.is_enabled() {
            return Err(UnifiedError::ServiceNotAvailable("auth".to_string()));
        }

        if self.status != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                "auth service not running".to_string(),
            ));
        }

        Ok(())
    }
}

/// 令牌信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenInfo {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 过期时间
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// 令牌类型
    pub token_type: String,
}

impl TokenInfo {
    /// 检查令牌是否过期
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    /// 检查令牌是否需要刷新
    pub fn needs_refresh(&self, buffer_minutes: u64) -> bool {
        let buffer = chrono::Duration::minutes(buffer_minutes);
        chrono::Utc::now() + buffer >= self.expires_at
    }
}

/// 令牌验证结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenVerificationResult {
    /// 是否有效
    pub valid: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 权限范围
    pub scopes: Vec<String>,
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnifiedService for AuthService {
    type Config = AuthConfig;
    type Error = UnifiedError;

    fn name(&self) -> &'static str {
        "auth"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn configure(&mut self, config: Self::Config) -> UnifiedResult<()> {
        if !config.enabled {
            self.status = ServiceStatus::Stopped;
            return Ok(());
        }

        self.config = Some(config);

        // 创建核心客户端
        let core_config = self.config.as_ref().map(|config| {
            openlark_core::config::ConfigBuilder::new()
                .build()
                .unwrap_or_else(|_| openlark_core::config::Config::default())
        });

        if let Some(core_config) = core_config {
            match openlark_core::client::LarkClient::new(
                core_config.app_id.clone(),
                core_config.app_secret.clone(),
            ) {
                Ok(client) => {
                    self.core_client = Some(Arc::new(client));

                    // 创建令牌管理器
                    let token_cache = openlark_core::cache::MemoryTokenCache::default();
                    let refresh_handler = openlark_core::token_manager::DefaultTokenRefreshHandler::new();
                    let token_manager = openlark_core::token_manager::TokenManager::new(
                        token_cache,
                        refresh_handler,
                    );

                    self.token_manager = Some(Arc::new(token_manager));
                    self.status = ServiceStatus::Running;

                    tracing::info!("认证服务配置成功");
                    Ok(())
                }
                Err(e) => {
                    self.status = ServiceStatus::Error;
                    Err(UnifiedError::ConfigurationError(
                        format!("创建核心客户端失败: {}", e),
                    ))
                }
            }
        } else {
            self.status = ServiceStatus::Error;
            Err(UnifiedError::ConfigurationError("认证配置无效".to_string()))
        }
    }

    fn is_available(&self) -> bool {
        self.is_enabled() && self.status == ServiceStatus::Running && self.core_client.is_some()
    }

    fn status(&self) -> ServiceStatus {
        self.status
    }

    fn descriptor(&self) -> ServiceDescriptor {
        let mut descriptor = ServiceDescriptor::new(
            "auth",
            "1.0.0",
            "飞书认证服务，提供令牌管理、OAuth认证、权限控制等功能",
        )
        .with_tag("authentication")
        .with_tag("security")
        .with_dependency("openlark-core");

        if let Some(config) = &self.config {
            descriptor = descriptor
                .with_metadata("enabled", config.enabled.to_string())
                .with_metadata(
                    "access_token_ttl",
                    config.token.access_token_ttl.to_string(),
                )
                .with_metadata(
                    "refresh_token_ttl",
                    config.token.refresh_token_ttl.to_string(),
                )
                .with_metadata("cache_enabled", config.token.cache.enabled.to_string());
        }

        descriptor
    }
}

#[async_trait]
impl ServiceLifecycle for AuthService {
    async fn start(&mut self) -> SDKResult<()> {
        if let Some(config) = self.config.clone() {
            self.configure(config).await?;
        } else {
            tracing::warn!("认证服务配置未设置，服务将处于未初始化状态");
        }
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.status = ServiceStatus::Stopped;
        self.core_client = None;
        self.token_manager = None;
        tracing::info!("认证服务已停止");
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        Ok(self.is_available())
    }
}

/// 认证服务构建器
pub struct AuthServiceBuilder {
    config: Option<AuthConfig>,
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
}

impl AuthServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            core_client: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: AuthConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置核心客户端
    pub fn core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 构建服务
    pub fn build(self) -> UnifiedResult<AuthService> {
        let mut service = AuthService::new();

        if let Some(config) = self.config {
            service = service.with_config(config);
        }

        if let Some(core_client) = self.core_client {
            service = service.with_core_client(core_client);
        }

        // 创建令牌管理器
        if service.core_client.is_some() {
            let token_cache = openlark_core::cache::MemoryTokenCache::default();
            let refresh_handler = openlark_core::token_manager::DefaultTokenRefreshHandler::new();
            let token_manager = openlark_core::token_manager::TokenManager::new(
                token_cache,
                refresh_handler,
            );

            service = service.with_token_manager(Arc::new(token_manager));
        }

        Ok(service)
    }
}

impl Default for AuthServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let service = AuthService::new();
        assert_eq!(service.name(), "auth");
        assert_eq!(service.version(), "1.0.0");
    }

    #[test]
    fn test_auth_service_builder() {
        let config = AuthConfig::default();
        let service = AuthServiceBuilder::new()
            .config(config)
            .build()
            .unwrap();

        assert!(service.is_enabled());
    }

    #[test]
    fn test_token_info() {
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(2);
        let token_info = TokenInfo {
            access_token: "test_token".to_string(),
            refresh_token: None,
            expires_at,
            token_type: "Bearer".to_string(),
        };

        assert!(!token_info.is_expired());
        assert!(!token_info.needs_refresh(30));
    }

    #[test]
    fn test_oauth_url() {
        let service = AuthService::new();
        let url = service.get_oauth_url(
            "test_client_id",
            "https://example.com/callback",
            &["user:info", "docs:read"],
        );

        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("scope=user:info docs:read"));
    }

    #[test]
    fn test_service_descriptors() {
        let service = AuthService::new();
        let descriptor = service.descriptor();

        assert_eq!(descriptor.name, "auth");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"authentication".to_string()));
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let mut service = AuthService::new();

        // 测试启动
        service.start().await.unwrap();
        // 由于没有配置，服务应该是未初始化状态
        assert_eq!(service.status(), ServiceStatus::Stopped);

        // 测试停止
        service.stop().await.unwrap();
        assert_eq!(service.status(), ServiceStatus::Stopped);
    }

    #[tokio::test]
    async fn test_auth_operations() {
        let service = AuthService::new();

        // 测试获取应用访问令牌
        let result = service.get_app_access_token().await;
        assert!(result.is_ok());

        // 测试获取用户访问令牌
        let result = service.get_user_access_token("test_code").await;
        assert!(result.is_ok());

        // 测试刷新令牌
        let result = service.refresh_access_token("test_refresh_token").await;
        assert!(result.is_ok());

        // 测试验证令牌
        let result = service.verify_token("test_token").await;
        assert!(result.is_ok());
    }
}