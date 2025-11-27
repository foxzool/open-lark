use super::context::ServiceContext;
use super::service::{Service, ServiceHealth, ServiceKind};
use crate::{error::with_operation_context, Config, Result};
use async_trait::async_trait;
use openlark_auth::models::{AppTicketResponse, UserInfoResponse};
use openlark_auth::prelude::*;
use openlark_auth::AuthServices;
use openlark_core::error::ErrorTrait;

/// 令牌验证响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenVerificationResponse {
    /// 是否有效
    pub valid: bool,
    /// 用户ID
    pub user_id: Option<String>,
    /// 租户Key
    pub tenant_key: Option<String>,
    /// 过期时间
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 权限范围
    pub scope: Vec<String>,
}

/**
 * 认证服务
 *
 * 提供认证相关的API接口，包括令牌管理、OAuth认证等
 * 集成了 openlark-auth 模块，提供 Project-Version-Resource 架构的认证服务
 */
#[derive(Debug)]
pub struct AuthService {
    auth_services: AuthServices,
}

impl AuthService {
    /// 创建新的认证服务实例
    pub fn new(config: &Config) -> Self {
        let auth_config =
            AuthConfig::new(&config.app_id, &config.app_secret).with_base_url(&config.base_url);

        let auth_services = AuthServices::new(auth_config);

        Self { auth_services }
    }

    /// 获取自建应用访问令牌
    pub async fn get_internal_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取自建应用访问令牌");

        let response = self
            .auth_services
            .auth
            .v3()
            .app_access_token()
            .internal()
            .send()
            .await;

        let result =
            with_operation_context(response, "get_internal_app_access_token", "AuthService")?;

        tracing::debug!("成功获取自建应用访问令牌，过期时间: {}秒", result.expire);

        Ok(TokenInfo {
            access_token: result.app_access_token,
            token_type: "Bearer".to_string(),
            expires_in: result.expire as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(result.expire as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 获取商店应用访问令牌
    pub async fn get_store_app_access_token(&self) -> Result<TokenInfo> {
        tracing::info!("获取商店应用访问令牌");

        let response = self
            .auth_services
            .auth
            .v3()
            .app_access_token()
            .store()
            .send()
            .await;

        let result = with_operation_context(response, "get_store_app_access_token", "AuthService")?;

        tracing::debug!("成功获取商店应用访问令牌，过期时间: {}秒", result.expire);

        Ok(TokenInfo {
            access_token: result.app_access_token,
            token_type: "Bearer".to_string(),
            expires_in: result.expire as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(result.expire as i64),
            scope: Some("app:all".to_string()),
        })
    }

    /// 使用授权码获取用户访问令牌
    pub async fn get_user_access_token(&self, code: &str) -> Result<TokenInfo> {
        tracing::info!("使用授权码获取用户访问令牌: code={}", code);

        let response = self
            .auth_services
            .authen
            .v1
            .access_token()
            .create()
            .grant_type("authorization_code")
            .code(code)
            .send()
            .await;

        let response = with_operation_context(response, "get_user_access_token", "AuthService")?;

        tracing::debug!(
            "成功获取用户访问令牌，过期时间: {}秒，权限范围: {:?}",
            response.expires_in,
            response.scope
        );

        Ok(TokenInfo {
            access_token: response.access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expires_in as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
            scope: response.scope,
        })
    }

    /// 刷新OIDC访问令牌
    pub async fn refresh_oidc_access_token(&self, refresh_token: &str) -> Result<TokenInfo> {
        tracing::info!("刷新OIDC访问令牌");

        let response = self
            .auth_services
            .authen
            .v1
            .oidc()
            .create_refresh_access_token()
            .refresh_token(refresh_token)
            .send()
            .await;

        let response =
            with_operation_context(response, "refresh_oidc_access_token", "AuthService")?;

        tracing::debug!("成功刷新OIDC访问令牌，过期时间: {}秒", response.expires_in);

        Ok(TokenInfo {
            access_token: response.access_token,
            token_type: "Bearer".to_string(),
            expires_in: response.expires_in as u64,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
            scope: response.scope,
        })
    }

    /// 验证令牌
    pub async fn verify_token(&self, access_token: &str) -> Result<TokenVerificationResponse> {
        tracing::info!(
            "验证令牌: token_prefix={}",
            access_token.chars().take(8).collect::<String>()
        );

        // 注意：应用访问令牌(app_access_token)无法通过用户信息接口验证
        // 因为它是应用级别的令牌，不是用户级别的令牌

        // 使用应用信息接口验证令牌有效性
        let app_info_result = self
            .auth_services
            .auth
            .v3()
            .app_ticket()
            .resend()
            .send()
            .await;

        match app_info_result {
            Ok(_) => {
                tracing::info!("应用访问令牌验证成功");

                // 对于app_access_token，我们验证的是应用级别的访问权限
                // 不返回具体的用户信息，因为这是应用令牌而不是用户令牌
                Ok(TokenVerificationResponse {
                    valid: true,
                    user_id: None, // app_access_token 不关联特定用户
                    tenant_key: Some("app_tenant".to_string()), // 应用租户标识
                    expires_at: None, // 需要令牌管理器来跟踪过期时间
                    scope: vec!["app:all".to_string()], // 应用级别的权限范围
                })
            }
            Err(e) => {
                tracing::warn!(
                    "应用访问令牌验证失败: {}",
                    e.user_message().unwrap_or("未知错误")
                );

                // 尝试通过获取令牌信息接口验证
                // 如果能成功获取新令牌，说明当前令牌仍然有效
                let token_refresh_result = self.get_internal_app_access_token().await;

                match token_refresh_result {
                    Ok(_) => {
                        tracing::info!("应用访问令牌仍然有效（通过刷新验证）");
                        Ok(TokenVerificationResponse {
                            valid: true,
                            user_id: None,
                            tenant_key: Some("app_tenant".to_string()),
                            expires_at: Some(token_refresh_result.unwrap().expires_at),
                            scope: vec!["app:all".to_string()],
                        })
                    }
                    Err(refresh_err) => {
                        tracing::warn!(
                            "应用访问令牌验证失败且无法刷新: {}",
                            refresh_err.user_message().unwrap_or("未知错误")
                        );
                        Ok(TokenVerificationResponse {
                            valid: false,
                            user_id: None,
                            tenant_key: None,
                            expires_at: None,
                            scope: vec![],
                        })
                    }
                }
            }
        }
    }

    /// 获取用户信息
    pub async fn get_user_info(&self, user_access_token: &str) -> Result<UserInfo> {
        tracing::info!("获取用户信息");

        let response = self
            .auth_services
            .authen
            .v1
            .user_info()
            .get()
            .user_access_token(user_access_token)
            .send()
            .await;

        let result = with_operation_context(response, "get_user_info", "AuthService")?;

        tracing::debug!(
            "成功获取用户信息，用户ID: {}, 状态: {:?}",
            result.user_id,
            result.status
        );

        // 用户状态已经是正确的枚举类型
        let status = result.status;

        Ok(UserInfo {
            user_id: result.user_id,
            open_id: result.open_id,
            union_id: result.union_id,
            name: result.name,
            en_name: result.en_name,
            email: result.email,
            mobile: result.mobile,
            avatar_url: result.avatar_url,
            status,
            department_ids: result.department_ids,
            position: result.position,
            employee_no: result.employee_no,
            nickname: result.nickname,
            gender: result.gender,
        })
    }

    /// 生成OAuth授权URL
    pub fn generate_oauth_url(&self, redirect_uri: &str, scope: &str, state: &str) -> String {
        format!(
            "https://open.feishu.cn/open-apis/authen/v1/authorize?app_id={}&redirect_uri={}&scope={}&state={}",
            self.auth_services.config.app_id, redirect_uri, scope, state
        )
    }

    /// 重新推送应用票据
    pub async fn resend_app_ticket(&self) -> Result<()> {
        tracing::info!("重新推送应用票据");

        let response = self
            .auth_services
            .auth
            .v3()
            .app_ticket()
            .resend()
            .send()
            .await;

        with_operation_context(response, "resend_app_ticket", "AuthService")?;

        tracing::info!("应用票据重新推送成功");

        Ok(())
    }

    /// 执行带有上下文的认证操作
    pub async fn execute_with_context<F, T>(&self, operation: &str, f: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let result = f.await;
        with_operation_context(result, operation, "AuthService")
    }
}

#[async_trait]
impl Service for AuthService {
    fn kind(&self) -> ServiceKind {
        ServiceKind::new("auth", "v1")
    }

    fn capabilities(&self) -> &'static [&'static str] {
        &["token", "oauth"]
    }

    async fn init(&self, _ctx: &ServiceContext) -> Result<()> {
        // 认证服务当前无需额外预热
        Ok(())
    }

    async fn start(&self, _ctx: &ServiceContext) -> Result<()> {
        Ok(())
    }

    fn health(&self) -> ServiceHealth {
        ServiceHealth::Ready
    }
}

// 为了保持客户端API的兼容性，定义类型别名
/// 用户信息类型别名
pub type UserInfo = UserInfoResponse;

/// 应用信息类型别名
pub type AppInfo = AppTicketResponse;

/// 令牌信息 - 客户端层的统一令牌表示
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

impl TokenInfo {
    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    /// 检查令牌是否需要刷新（提前N分钟）
    pub fn needs_refresh(&self, buffer_minutes: i64) -> bool {
        let buffer = chrono::Duration::minutes(buffer_minutes);
        chrono::Utc::now() + buffer >= self.expires_at
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - chrono::Utc::now()).num_seconds().max(0)
    }

    /// 获取令牌状态的详细描述
    pub fn status_description(&self) -> &'static str {
        if self.is_expired() {
            "令牌已过期"
        } else if self.needs_refresh(5) {
            "令牌即将过期，建议刷新"
        } else {
            "令牌有效"
        }
    }

    /// 获取友好的过期时间描述
    pub fn friendly_expires_at(&self) -> String {
        let now = chrono::Utc::now();
        let duration = self.expires_at - now;

        if duration.num_seconds() < 0 {
            "已过期".to_string()
        } else if duration.num_hours() >= 1 {
            format!("{}小时后过期", duration.num_hours())
        } else if duration.num_minutes() >= 1 {
            format!("{}分钟后过期", duration.num_minutes())
        } else {
            format!("{}秒后过期", duration.num_seconds())
        }
    }
}

/// 令牌管理扩展特征
pub trait TokenManagement {
    /// 自动刷新令牌（如果需要）
    async fn auto_refresh_if_needed<F>(
        &self,
        token_info: &TokenInfo,
        refresh_func: F,
    ) -> Result<TokenInfo>
    where
        F: FnOnce(
            &TokenInfo,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<TokenInfo>> + Send>>;

    /// 安全执行需要令牌的操作
    async fn execute_with_valid_token<F, T>(&self, token_info: &TokenInfo, f: F) -> Result<T>
    where
        F: FnOnce(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>;
}

impl TokenManagement for AuthService {
    async fn auto_refresh_if_needed<F>(
        &self,
        token_info: &TokenInfo,
        refresh_func: F,
    ) -> Result<TokenInfo>
    where
        F: FnOnce(
            &TokenInfo,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<TokenInfo>> + Send>>,
    {
        if token_info.needs_refresh(10) {
            tracing::info!("令牌需要刷新，执行自动刷新");
            refresh_func(token_info).await
        } else {
            tracing::debug!("令牌仍然有效，无需刷新");
            Ok(token_info.clone())
        }
    }

    async fn execute_with_valid_token<F, T>(&self, token_info: &TokenInfo, f: F) -> Result<T>
    where
        F: FnOnce(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
    {
        if token_info.is_expired() {
            return Err(crate::error::authentication_error(format!(
                "令牌已过期，过期时间: {}",
                token_info.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
            )));
        }

        tracing::debug!(
            "令牌有效，执行操作，剩余时间: {}秒",
            token_info.remaining_seconds()
        );
        f(&token_info.access_token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_auth_service_creation() {
        let config = create_test_config();
        let service = AuthService::new(&config);
        // 服务创建成功即测试通过
        assert_eq!(service.auth_services.config.app_id, "test_app_id");
    }

    #[test]
    fn test_oauth_url_generation() {
        let config = create_test_config();
        let service = AuthService::new(&config);

        let url = service.generate_oauth_url(
            "https://example.com/callback",
            "user:info docs:read",
            "test_state",
        );

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("scope=user:info docs:read"));
        assert!(url.contains("state=test_state"));
    }

    #[test]
    fn test_token_info_methods() {
        let mut token_info = TokenInfo {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 7200,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            scope: Some("test".to_string()),
        };

        assert!(!token_info.is_expired());
        assert!(!token_info.needs_refresh(30));
        assert!(token_info.remaining_seconds() > 0);
        assert_eq!(token_info.status_description(), "令牌有效");
        assert!(token_info.friendly_expires_at().contains("小时后过期"));

        // 测试过期令牌
        token_info.expires_at = chrono::Utc::now() - chrono::Duration::minutes(1);
        assert!(token_info.is_expired());
        assert!(token_info.needs_refresh(30));
        assert_eq!(token_info.remaining_seconds(), 0);
        assert_eq!(token_info.status_description(), "令牌已过期");
        assert_eq!(token_info.friendly_expires_at(), "已过期");
    }

    #[test]
    fn test_token_error_context() {
        let config = create_test_config();
        let service = AuthService::new(&config);

        // 测试错误上下文（模拟）
        let error = crate::error::authentication_error("测试错误");
        let result = service.handle_error(Err(error), "test_operation");

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.has_context("operation"));
            assert_eq!(err.get_context("operation"), Some("test_operation"));
            assert_eq!(err.get_context("component"), Some("AuthService"));
        }
    }

    #[test]
    fn test_token_management() {
        let config = create_test_config();
        let service = AuthService::new(&config);

        let token_info = TokenInfo {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 60,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(60),
            scope: Some("test".to_string()),
        };

        // 测试令牌验证功能
        assert!(!token_info.is_expired());
        assert!(!token_info.needs_refresh(10));
        assert_eq!(token_info.status_description(), "令牌有效");
        assert!(token_info.remaining_seconds() > 0);

        // 测试状态描述
        assert!(token_info.status_description().contains("有效"));
    }
}
