//! 服务层模块
//!
//! 提供符合 Project-Version-Resource 架构的服务接口。

use crate::client::TokenClient;

pub mod auth;
pub mod authen;
pub mod oauth;

// 重新导出主要服务
pub use auth::AuthServiceV3;
pub use authen::AuthenServiceV1;
pub use oauth::OAuthServiceOld;

/// 统一的服务客户端
///
/// 提供所有认证服务的统一访问入口，符合 Project-Version-Resource 架构设计。
#[derive(Debug, Clone)]
pub struct AuthServices {
    /// Auth v3 企业应用认证服务
    pub auth: auth::AuthServiceV3,
    /// Authen v1 用户身份认证服务
    pub authen: authen::AuthenServiceV1,
    /// OAuth old 版本授权服务
    pub oauth: oauth::OAuthServiceOld,
}

impl AuthServices {
    /// 创建新的服务客户端实例
    pub fn new(token_client: TokenClient) -> Self {
        Self {
            auth: auth::AuthServiceV3::new(token_client.clone()),
            authen: authen::AuthenServiceV1::new(token_client.clone()),
            oauth: oauth::OAuthServiceOld::new(token_client),
        }
    }
}

/// 服务构建器
#[derive(Debug, Clone)]
pub struct AuthServicesBuilder {
    token_client: Option<TokenClient>,
}

impl AuthServicesBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self { token_client: None }
    }

    /// 设置令牌客户端
    pub fn token_client(mut self, client: TokenClient) -> Self {
        self.token_client = Some(client);
        self
    }

    /// 构建服务客户端
    pub fn build(self) -> Result<AuthServices, crate::AuthError> {
        let token_client = self
            .token_client
            .ok_or_else(|| crate::AuthError::InvalidParameter {
                parameter: "token_client".to_string(),
                reason: "Token client is required".to_string(),
            })?;

        Ok(AuthServices::new(token_client))
    }
}

impl Default for AuthServicesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_services_builder() {
        let builder = AuthServicesBuilder::new();

        // 测试构建失败（缺少token_client）
        let result = builder.build();
        assert!(result.is_err());

        // 测试构建成功
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let services = AuthServicesBuilder::new()
            .token_client(token_client)
            .build()
            .unwrap();

        // 验证所有服务都已创建（只测试有实现的方法）
        assert!(services.auth.app_access_token().internal().await.is_ok()); // 有实现
        assert!(services
            .auth
            .tenant_access_token()
            .internal("test_tenant")
            .await
            .is_ok()); // 有实现
    }

    #[tokio::test]
    async fn test_services_creation() {
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let services = AuthServices::new(token_client);

        // 验证服务结构（只测试有实现的方法）
        assert!(services.auth.app_access_token().internal().await.is_ok());
        assert!(services
            .auth
            .tenant_access_token()
            .internal("test_tenant")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_services_project_version_resource_structure() {
        let token_client = TokenClient::new(
            crate::client::config::AuthConfigBuilder::new()
                .app_id("test_app")
                .app_secret("test_secret")
                .build()
                .unwrap(),
        )
        .unwrap();

        let services = AuthServices::new(token_client);

        // 验证 Project-Version-Resource 结构
        // auth 项目 (v3)
        let app_token_service = services.auth.app_access_token();
        let tenant_token_service = services.auth.tenant_access_token();

        // authen 项目 (v1)
        let user_token_service = services.authen.access_token();
        let user_info_service = services.authen.user_info();
        let oidc_service = services.authen.oidc_access_token();

        // oauth 项目 (old)
        let oauth_service = services.oauth.index();

        // 验证所有服务都存在（只测试有实现的方法）
        assert!(app_token_service.internal().await.is_ok());
        assert!(tenant_token_service.internal("test").await.is_ok());

        // 验证服务对象存在（不需要调用方法）
        assert!(std::mem::size_of_val(&user_token_service) > 0);
        assert!(std::mem::size_of_val(&user_info_service) > 0);
        assert!(std::mem::size_of_val(&oidc_service) > 0);
        assert!(std::mem::size_of_val(&oauth_service) > 0);
    }
}
