//! Auth v3 企业应用认证服务

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::auth::token::{AppType, GetTokenRequest};
use crate::client::TokenClient;
use crate::endpoints::AuthEndpoints;
use crate::error::{AuthError, AuthResult};

/// Auth v3 企业应用认证服务
#[derive(Debug, Clone)]
pub struct AuthServiceV3 {
    token_client: TokenClient,
}

impl AuthServiceV3 {
    /// 创建新的 Auth v3 服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 获取应用访问令牌服务
    pub fn app_access_token(&self) -> AppAccessTokenService {
        AppAccessTokenService::new(self.token_client.clone())
    }

    /// 获取租户访问令牌服务
    pub fn tenant_access_token(&self) -> TenantAccessTokenService {
        TenantAccessTokenService::new(self.token_client.clone())
    }

    /// 应用权限查询服务
    pub fn app_permissions(&self) -> AppPermissionsService {
        AppPermissionsService::new(self.token_client.clone())
    }
}

/// 应用访问令牌服务
#[derive(Debug, Clone)]
pub struct AppAccessTokenService {
    token_client: TokenClient,
}

impl AppAccessTokenService {
    /// 创建新的应用访问令牌服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 自建应用获取应用访问令牌
    pub async fn internal(&self) -> AuthResult<crate::auth::token::AccessToken> {
        let request = GetTokenRequest::self_build_app_access_token();
        self.token_client.get_access_token(request).await
    }

    /// 商店应用获取应用访问令牌
    pub async fn store(&self) -> AuthResult<crate::auth::token::AccessToken> {
        let request = GetTokenRequest::store_app_access_token();
        self.token_client.get_access_token(request).await
    }

    /// 重新推送应用票据
    pub async fn ticket_resend(&self, app_id: &str) -> AuthResult<TicketResendResponse> {
        // TODO: 实现应用票据重新推送逻辑
        todo!("Implement app ticket resend")
    }
}

/// 租户访问令牌服务
#[derive(Debug, Clone)]
pub struct TenantAccessTokenService {
    token_client: TokenClient,
}

impl TenantAccessTokenService {
    /// 创建新的租户访问令牌服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 自建应用获取租户访问令牌
    pub async fn internal(&self, tenant_key: &str) -> AuthResult<crate::auth::token::AccessToken> {
        let request = GetTokenRequest::self_build_tenant_access_token(tenant_key.to_string());
        self.token_client.get_access_token(request).await
    }

    /// 商店应用获取租户访问令牌
    pub async fn store(&self, tenant_key: &str) -> AuthResult<crate::auth::token::AccessToken> {
        let request = GetTokenRequest::store_tenant_access_token(tenant_key.to_string());
        self.token_client.get_access_token(request).await
    }
}

/// 应用权限查询服务
#[derive(Debug, Clone)]
pub struct AppPermissionsService {
    token_client: TokenClient,
}

impl AppPermissionsService {
    /// 创建新的应用权限查询服务实例
    pub fn new(token_client: TokenClient) -> Self {
        Self { token_client }
    }

    /// 查询应用权限
    pub async fn query(&self, app_id: &str) -> AuthResult<AppPermissionsResponse> {
        // TODO: 实现应用权限查询逻辑
        todo!("Implement app permissions query")
    }
}

/// 应用票据重发响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketResendResponse {
    /// 是否成功
    pub success: bool,
    /// 错误码（如果有）
    pub error_code: Option<String>,
    /// 错误信息（如果有）
    pub error_message: Option<String>,
}

/// 应用权限查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPermissionsResponse {
    /// 权限列表
    pub permissions: Vec<AppPermission>,
}

/// 应用权限项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPermission {
    /// 权限编码
    pub code: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    pub description: Option<String>,
    /// 权限范围
    pub scope: String,
}

/// 服务客户端接口
#[async_trait]
pub trait AuthV3ServiceClient: Send + Sync {
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

/// 默认的 Auth v3 服务客户端实现
#[derive(Debug, Clone)]
pub struct DefaultAuthV3ServiceClient {
    base_url: String,
    app_id: String,
    app_secret: String,
}

impl DefaultAuthV3ServiceClient {
    /// 创建新的默认 Auth v3 服务客户端实例
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
impl AuthV3ServiceClient for DefaultAuthV3ServiceClient {
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
        todo!("Implement HTTP request logic for auth v3 service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_service_creation() {
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let auth_service = AuthServiceV3::new(token_client);
        assert!(auth_service.app_access_token().internal().await.is_err()); // 需要真实的环境才能执行
    }

    #[test]
    fn test_service_client_creation() {
        let client =
            DefaultAuthV3ServiceClient::new("app_id".to_string(), "app_secret".to_string());
        assert_eq!(client.base_url(), "https://open.feishu.cn");

        let client_with_custom_url =
            DefaultAuthV3ServiceClient::new("app_id".to_string(), "app_secret".to_string())
                .with_base_url("https://open.larksuite.com".to_string());
        assert_eq!(
            client_with_custom_url.base_url(),
            "https://open.larksuite.com"
        );
    }
}
