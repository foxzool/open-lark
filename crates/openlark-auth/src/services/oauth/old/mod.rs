//! OAuth old 版本授权服务

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client::TokenClient;
use crate::endpoints::AuthEndpoints;
use crate::error::{AuthError, AuthResult};

/// OAuth old 版本授权服务
#[derive(Debug, Clone)]
pub struct OAuthServiceOld {
    token_client: TokenClient,
}

impl OAuthServiceOld {
    /// 创建新的 OAuth old 服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 获取登录预授权码服务
    pub fn index(&self) -> IndexService {
        IndexService::new(self.token_client.clone())
    }
}

/// 登录预授权码服务
#[derive(Debug, Clone)]
pub struct IndexService {
    token_client: TokenClient,
}

impl IndexService {
    /// 创建新的登录预授权码服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 获取登录预授权码
    pub async fn get(
        &self,
        app_id: &str,
        redirect_uri: &str,
        scope: &str,
    ) -> AuthResult<IndexResponse> {
        // TODO: 实现登录预授权码获取逻辑
        todo!("Implement index pre-authorization code retrieval")
    }

    /// 构建授权URL
    pub fn build_authorization_url(
        &self,
        app_id: &str,
        redirect_uri: &str,
        scope: &str,
        state: Option<&str>,
    ) -> String {
        let base_url = "https://open.feishu.cn/open-apis/authen/v1/index";
        let mut url = format!(
            "{}?app_id={}&redirect_uri={}&scope={}",
            base_url, app_id, redirect_uri, scope
        );

        if let Some(s) = state {
            url.push_str(&format!("&state={}", s));
        }

        url
    }

    /// 处理OAuth回调
    pub async fn handle_callback(
        &self,
        code: &str,
        state: Option<&str>,
    ) -> AuthResult<OAuthCallbackResponse> {
        // TODO: 实现OAuth回调处理逻辑
        todo!("Implement OAuth callback handling")
    }
}

/// 登录预授权码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexResponse {
    /// 预授权码
    pub pre_auth_code: String,
    /// 状态值
    pub state: Option<String>,
    /// 过期时间（秒）
    pub expires_in: u64,
}

/// OAuth回调响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCallbackResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 刷新令牌
    pub refresh_token: String,
    /// 授权范围
    pub scope: String,
    /// 用户ID
    pub user_id: String,
    /// 租户密钥
    pub tenant_key: String,
    /// 应用权限
    pub permissions: Vec<String>,
}

/// 服务客户端接口
#[async_trait]
pub trait OAuthOldServiceClient: Send + Sync {
    /// 获取服务的基础URL
    fn base_url(&self) -> &str;

    /// 执行HTTP请求
    async fn request<T: Serialize + Send + Sync>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<T>,
    ) -> AuthResult<serde_json::Value>;
}

/// 默认的 OAuth old 服务客户端实现
#[derive(Debug, Clone)]
pub struct DefaultOAuthOldServiceClient {
    base_url: String,
    app_id: String,
    app_secret: String,
}

impl DefaultOAuthOldServiceClient {
    /// 创建新的默认 OAuth old 服务客户端实例
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            base_url: "https://open.feishu.cn".to_string(),
            app_id,
            app_secret,
        }
    }

    /// 设置自定义基础URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[async_trait]
impl OAuthOldServiceClient for DefaultOAuthOldServiceClient {
    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn request<T: Serialize + Send + Sync>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<T>,
    ) -> AuthResult<serde_json::Value> {
        // TODO: 实现HTTP请求逻辑
        todo!("Implement HTTP request logic for oauth old service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_oauth_service_creation() {
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let oauth_service = OAuthServiceOld::new(token_client);
        assert!(oauth_service
            .index()
            .get("app_id", "redirect_uri", "scope")
            .await
            .is_err()); // 需要真实的环境才能执行
    }

    #[test]
    fn test_authorization_url_building() {
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let index_service = IndexService::new(token_client);

        // 测试不带state的URL构建
        let url = index_service.build_authorization_url(
            "app_123",
            "https://example.com/callback",
            "contact:base",
            None,
        );
        assert!(url.contains("app_id=app_123"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("scope=contact:base"));
        assert!(!url.contains("state="));

        // 测试带state的URL构建
        let url_with_state = index_service.build_authorization_url(
            "app_123",
            "https://example.com/callback",
            "contact:base",
            Some("random_state_123"),
        );
        assert!(url_with_state.contains("state=random_state_123"));
    }

    #[test]
    fn test_service_client_creation() {
        let client =
            DefaultOAuthOldServiceClient::new("app_id".to_string(), "app_secret".to_string());
        assert_eq!(client.base_url(), "https://open.feishu.cn");

        let client_with_custom_url =
            DefaultOAuthOldServiceClient::new("app_id".to_string(), "app_secret".to_string())
                .with_base_url("https://open.larksuite.com".to_string());
        assert_eq!(
            client_with_custom_url.base_url(),
            "https://open.larksuite.com"
        );
    }
}
