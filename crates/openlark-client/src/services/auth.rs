//! 认证服务 - 修复版本
//!
//! 集成 openlark-auth 模块，提供完整的认证API服务

use super::context::ServiceContext;
use super::service::{Service, ServiceHealth, ServiceKind};
use crate::{error::with_operation_context, Config, Result};
use async_trait::async_trait;

/// 令牌信息
#[derive(Debug, Clone)]
pub struct TokenInfo {
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 过期时间戳
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// 权限范围
    pub scope: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户OpenID
    pub open_id: String,
    /// 用户UnionID
    pub union_id: Option<String>,
    /// 用户名
    pub name: String,
    /// 用户昵称
    pub nickname: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 用户头像
    pub avatar_url: Option<String>,
    /// 用户性别
    pub gender: Option<String>,
    /// 租户Key
    pub tenant_key: Option<String>,
}

/**
 * 认证服务
 *
 * 提供认证相关的API接口，包括令牌管理、OAuth认证等
 * 集成了 openlark-auth 模块，提供 Project-Version-Resource 架构的认证服务
 */
#[derive(Debug)]
pub struct AuthService {
    config: openlark_core::config::Config,
}

impl AuthService {
    /// 创建新的认证服务实例
    pub fn new(config: &Config) -> Self {
        let core_config = openlark_core::config::Config::builder()
            .app_id(config.app_id.clone())
            .app_secret(config.app_secret.clone())
            .base_url(config.base_url.clone())
            .req_timeout(config.timeout)
            .header(config.headers.clone())
            .build();

        Self {
            config: core_config,
        }
    }

    /// 获取自建应用访问令牌
    pub async fn get_internal_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取自建应用访问令牌");

        // 使用openlark-auth的API
        use openlark_auth::api::auth::v3::auth::AuthServiceV3;
        let auth_service = AuthServiceV3::new(self.config.clone());

        let response = auth_service
            .app_access_token_internal()
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result =
            with_operation_context(response, "get_internal_app_access_token", "AuthService")?;

        tracing::debug!(
            "成功获取自建应用访问令牌，过期时间: {}秒",
            result.data.expires_in
        );

        Ok(TokenInfo {
            access_token: result.data.app_access_token,
            token_type: result.data.token_type.unwrap_or("Bearer".to_string()),
            expires_in: result.data.expires_in,
            expires_at: chrono::Utc::now()
                + chrono::Duration::seconds(result.data.expires_in as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 获取商店应用访问令牌
    pub async fn get_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取商店应用访问令牌");

        use openlark_auth::api::auth::v3::auth::AuthServiceV3;
        let auth_service = AuthServiceV3::new(self.config.clone());

        let response = auth_service
            .app_access_token()
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result = with_operation_context(response, "get_app_access_token", "AuthService")?;

        tracing::debug!(
            "成功获取商店应用访问令牌，过期时间: {}秒",
            result.data.expires_in
        );

        Ok(TokenInfo {
            access_token: result.data.app_access_token,
            token_type: "Bearer".to_string(),
            expires_in: result.data.expires_in as u64,
            expires_at: chrono::Utc::now()
                + chrono::Duration::seconds(result.data.expires_in as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 获取自建应用租户访问令牌
    pub async fn get_internal_tenant_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取自建应用租户访问令牌");

        use openlark_auth::api::auth::v3::auth::AuthServiceV3;
        let auth_service = AuthServiceV3::new(self.config.clone());

        let response = auth_service
            .tenant_access_token_internal()
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result =
            with_operation_context(response, "get_internal_tenant_access_token", "AuthService")?;

        tracing::debug!(
            "成功获取自建应用租户访问令牌，过期时间: {}秒",
            result.data.expires_in
        );

        Ok(TokenInfo {
            access_token: result.data.tenant_access_token,
            token_type: "Bearer".to_string(),
            expires_in: result.data.expires_in as u64,
            expires_at: chrono::Utc::now()
                + chrono::Duration::seconds(result.data.expires_in as i64),
            scope: Some("tenant:all".to_string()),
        })
    }

    /// 获取商店应用租户访问令牌
    pub async fn get_tenant_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取商店应用租户访问令牌");

        use openlark_auth::api::auth::v3::auth::AuthServiceV3;
        let auth_service = AuthServiceV3::new(self.config.clone());

        let response = auth_service
            .tenant_access_token()
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result = with_operation_context(response, "get_tenant_access_token", "AuthService")?;

        tracing::debug!(
            "成功获取商店应用租户访问令牌，过期时间: {}秒",
            result.data.expires_in
        );

        Ok(TokenInfo {
            access_token: result.data.tenant_access_token,
            token_type: "Bearer".to_string(),
            expires_in: result.data.expires_in as u64,
            expires_at: chrono::Utc::now()
                + chrono::Duration::seconds(result.data.expires_in as i64),
            scope: Some("tenant:all".to_string()),
        })
    }

    /// 获取用户信息
    pub async fn get_user_info(&self, user_access_token: &str) -> Result<UserInfo> {
        tracing::info!("获取用户信息");

        use openlark_auth::api::authen::v1::AuthenServiceV1;
        let authen_service = AuthenServiceV1::new(self.config.clone());

        let response = authen_service
            .user_info()
            .get()
            .user_access_token(user_access_token)
            .execute()
            .await;

        let result = with_operation_context(response, "get_user_info", "AuthService")?;

        // 转换为统一的UserInfo结构
        let user_info = UserInfo {
            user_id: result.data.data.user_id.unwrap_or_default(),
            open_id: result.data.data.open_id,
            union_id: result.data.data.union_id,
            name: result.data.data.name.unwrap_or_default(),
            nickname: result.data.data.en_name,
            email: result.data.data.email,
            mobile: result.data.data.mobile,
            avatar_url: result.data.data.avatar.and_then(|a| a.avatar_240),
            gender: None, // UserInfo 中没有 gender 字段
            tenant_key: result.data.data.tenant_key,
        };

        tracing::debug!("成功获取用户信息: {}", user_info.name);

        Ok(user_info)
    }

    /// 重新获取app_ticket
    pub async fn resend_app_ticket(&self) -> Result<()> {
        tracing::info!("重新获取app_ticket");

        use openlark_auth::api::auth::v3::auth::AuthServiceV3;
        let auth_service = AuthServiceV3::new(self.config.clone());

        let response = auth_service
            .app_ticket_resend()
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let _ = with_operation_context(response, "resend_app_ticket", "AuthService")?;

        tracing::debug!("成功触发app_ticket重新推送");

        Ok(())
    }

    /// 获取用户访问令牌（v1版本）
    pub async fn get_user_access_token_v1(
        &self,
        grant_code: &str,
    ) -> Result<openlark_auth::models::authen::UserAccessTokenResponse> {
        tracing::info!("获取用户访问令牌（v1版本）");

        use openlark_auth::api::authen::v1::AuthenServiceV1;
        let authen_service = AuthenServiceV1::new(self.config.clone());

        let response = authen_service
            .access_token()
            .grant_code(grant_code)
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result = with_operation_context(response, "get_user_access_token_v1", "AuthService")?;

        tracing::debug!("成功获取用户访问令牌");

        Ok(result.data)
    }

    /// 刷新用户访问令牌（v1版本）
    pub async fn refresh_user_access_token_v1(
        &self,
        refresh_token: &str,
    ) -> Result<openlark_auth::models::authen::UserAccessTokenResponse> {
        tracing::info!("刷新用户访问令牌（v1版本）");

        use openlark_auth::api::authen::v1::AuthenServiceV1;
        let authen_service = AuthenServiceV1::new(self.config.clone());

        let response = authen_service
            .refresh_access_token()
            .refresh_token(refresh_token)
            .app_id(&self.config.app_id)
            .app_secret(&self.config.app_secret)
            .execute()
            .await;

        let result =
            with_operation_context(response, "refresh_user_access_token_v1", "AuthService")?;

        tracing::debug!("成功刷新用户访问令牌");

        Ok(result.data)
    }

    /// 获取OIDC用户访问令牌
    pub async fn get_oidc_user_access_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<openlark_auth::models::authen::UserAccessTokenResponse> {
        tracing::info!("获取OIDC用户访问令牌");

        use openlark_auth::api::authen::v1::AuthenServiceV1;
        let authen_service = AuthenServiceV1::new(self.config.clone());

        let response = authen_service
            .oidc()
            .access_token()
            .code(code)
            .redirect_uri(redirect_uri)
            .execute()
            .await;

        let result = with_operation_context(response, "get_oidc_user_access_token", "AuthService")?;

        tracing::debug!("成功获取OIDC用户访问令牌");

        Ok(result.data)
    }

    /// 刷新OIDC用户访问令牌
    pub async fn refresh_oidc_user_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<openlark_auth::models::authen::UserAccessTokenResponse> {
        tracing::info!("刷新OIDC用户访问令牌");

        use openlark_auth::api::authen::v1::AuthenServiceV1;
        let authen_service = AuthenServiceV1::new(self.config.clone());

        let response = authen_service
            .oidc()
            .refresh_access_token()
            .refresh_token(refresh_token)
            .execute()
            .await;

        let result =
            with_operation_context(response, "refresh_oidc_user_access_token", "AuthService")?;

        tracing::debug!("成功刷新OIDC用户访问令牌");

        Ok(result.data)
    }

    /// 构建OAuth授权URL
    pub fn build_oauth_url(
        &self,
        redirect_uri: &str,
        scope: Option<&str>,
        state: Option<&str>,
    ) -> String {
        tracing::info!("构建OAuth授权URL");

        use openlark_auth::api::oauth::old::OAuthServiceOld;
        let oauth_service = OAuthServiceOld::new(self.config.clone());

        let builder = oauth_service
            .authorization()
            .app_id(&self.config.app_id)
            .redirect_uri(redirect_uri);

        let builder = if let Some(s) = scope {
            builder.scope(s)
        } else {
            builder
        };

        let builder = if let Some(s) = state {
            builder.state(s)
        } else {
            builder
        };

        builder.build_url()
    }
}

#[async_trait]
impl Service for AuthService {
    fn kind(&self) -> ServiceKind {
        ServiceKind {
            name: "auth".into(),
            version: "1.0.0".into(),
        }
    }

    fn capabilities(&self) -> &'static [&'static str] {
        &[
            "token_management",
            "user_authentication",
            "oauth_authorization",
            "app_ticket_management",
        ]
    }

    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    async fn init(&self, _context: &ServiceContext) -> Result<()> {
        tracing::info!("初始化认证服务");
        Ok(())
    }

    async fn start(&self, _context: &ServiceContext) -> Result<()> {
        tracing::info!("启动认证服务");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        tracing::info!("停止认证服务");
        Ok(())
    }

    fn health(&self) -> ServiceHealth {
        ServiceHealth::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_config_bridge() {
        let mut config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://open.feishu.cn")
            .build()
            .unwrap();
        config.add_header("X-Test-Header", "test-value");

        let service = AuthService::new(&config);

        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert_eq!(service.config.base_url, "https://open.feishu.cn");
        assert_eq!(service.config.req_timeout, Some(config.timeout));
        assert_eq!(
            service.config.header.get("X-Test-Header"),
            Some(&"test-value".to_string())
        );
    }
}
